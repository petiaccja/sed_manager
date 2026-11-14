//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeSet;
use std::sync::Arc;

use googletest::assert_that;
use googletest::matchers::*;
use rstest::rstest;
use sed_async::{PolyRuntime, TokioRuntime};
use sed_manager::DeviceSpec;
use sed_manager::{Error, SetupSession};
use sed_packet::MaxBytes;
use sed_packet::com_id::ComIdExt;
use sed_packet::discovery::{BlockSIDAuthDescriptor, LockingDescriptor};
use sed_spec::ssc::Ssc;
use sed_spec::uid_registry::opal_2::{admin as opal_admin, locking as opal_locking};
use sed_spec::uid_registry::psid;
use sed_telemetry::with_tracing;
use sed_tper::Tper;
use sed_virtual_device::{BASE_COM_ID, INITIAL_SID_PASSWORD, PSID_PASSWORD, VirtualDevice};
use tracing::instrument;

const NOT_DEFAULT: MaxBytes<32> = MaxBytes::from_array(*b"not_default");

async fn setup(ssc: Ssc) -> (SetupSession, Arc<VirtualDevice>) {
    let device = Arc::new(VirtualDevice::with_ssc(ssc));
    let runtime = Arc::new(PolyRuntime::Tokio(TokioRuntime::current().unwrap()));
    let tper = Tper::connect(BASE_COM_ID, ComIdExt(0), None, device.clone(), runtime);
    let spec = DeviceSpec::try_new(tper.discover_current().await.unwrap()).unwrap();
    let session = SetupSession::new(&tper, spec);
    (session, device)
}

#[instrument]
#[tokio::test]
#[with_tracing]
async fn take_ownership() {
    let (session, device) = setup(Ssc::OpalV2).await;

    let block_sid_before = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid_before.sid_msid_pin_differ, eq(false));

    let result = session.take_owneship(NOT_DEFAULT).await;
    assert_that!(result, ok(anything()));

    let block_sid_after = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid_after.sid_msid_pin_differ, eq(true));
}

#[instrument]
#[tokio::test]
#[with_tracing]
async fn take_ownership_already_owned() {
    let (session, _device) = setup(Ssc::OpalV2).await;

    session.take_owneship(NOT_DEFAULT).await.unwrap();
    let result = session.take_owneship(NOT_DEFAULT).await;
    assert_that!(result, err(eq(&Error::AlreadyOwned)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
async fn activate_secondary_sp() {
    let (session, device) = setup(Ssc::OpalV2).await;

    let locking_before = device.discover().get::<LockingDescriptor>().unwrap().clone();
    assert_that!(locking_before.locking_enabled, eq(false));

    let result = session.activate_secondary_sp(INITIAL_SID_PASSWORD).await;
    assert_that!(result, ok(anything()));

    let locking_after = device.discover().get::<LockingDescriptor>().unwrap().clone();
    assert_that!(locking_after.locking_enabled, eq(true));
}

#[instrument]
#[tokio::test]
#[with_tracing]
async fn activate_secondary_sp_already_activated() {
    let (session, _device) = setup(Ssc::OpalV2).await;

    session.activate_secondary_sp(INITIAL_SID_PASSWORD).await.unwrap();
    let result = session.activate_secondary_sp(INITIAL_SID_PASSWORD).await;
    assert_that!(result, err(eq(&Error::AlreadyActivated)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
async fn revert_tper_with_sid() {
    let new_sid_password = NOT_DEFAULT;
    let (session, device) = setup(Ssc::OpalV2).await;

    session.take_owneship(new_sid_password.clone()).await.unwrap();
    session.activate_secondary_sp(new_sid_password.clone()).await.unwrap();

    let result = session.revert_tper(opal_admin::authority::SID, new_sid_password).await;
    assert_that!(result, ok(anything()));

    let block_sid = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid.sid_msid_pin_differ, eq(false));
    let locking = device.discover().get::<LockingDescriptor>().unwrap().clone();
    assert_that!(locking.locking_enabled, eq(false));
}

#[instrument]
#[tokio::test]
#[with_tracing]
async fn revert_tper_with_psid() {
    let new_sid_password = NOT_DEFAULT;
    let (session, device) = setup(Ssc::OpalV2).await;

    session.take_owneship(new_sid_password.clone()).await.unwrap();
    session.activate_secondary_sp(new_sid_password).await.unwrap();

    let result = session.revert_tper(psid::admin::authority::PSID, PSID_PASSWORD).await;
    assert_that!(result, ok(anything()));

    let block_sid = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid.sid_msid_pin_differ, eq(false));
    let locking = device.discover().get::<LockingDescriptor>().unwrap().clone();
    assert_that!(locking.locking_enabled, eq(false));
}

#[instrument]
#[tokio::test]
#[with_tracing]
async fn revert_secondary_sp() {
    let new_sid_password = NOT_DEFAULT;
    let (session, device) = setup(Ssc::OpalV2).await;

    session.take_owneship(new_sid_password.clone()).await.unwrap();
    session.activate_secondary_sp(new_sid_password.clone()).await.unwrap();

    let result = session.revert_secondary_sp(new_sid_password).await;
    assert_that!(result, ok(anything()));

    // Secondary SP is reverted.
    let locking = device.discover().get::<LockingDescriptor>().unwrap().clone();
    assert_that!(locking.locking_enabled, eq(false));

    // Admin SP is unaffected: the changed SID password persists.
    let block_sid = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid.sid_msid_pin_differ, eq(true));
}

#[instrument]
#[tokio::test]
#[with_tracing]
async fn revert_secondary_sp_ex() {
    let (session, device) = setup(Ssc::OpalV2).await;

    session.take_owneship(NOT_DEFAULT).await.unwrap();
    session.activate_secondary_sp(NOT_DEFAULT).await.unwrap();

    let admin1 = opal_locking::authority::ADMIN.get(0).unwrap();
    let result = session.revert_secondary_sp_ex(admin1, NOT_DEFAULT, None).await;
    assert_that!(result, ok(anything()));

    // Secondary SP is reverted.
    let locking = device.discover().get::<LockingDescriptor>().unwrap().clone();
    assert_that!(locking.locking_enabled, eq(false));

    // Admin SP is unaffected: the changed SID password persists.
    let block_sid = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid.sid_msid_pin_differ, eq(true));
}

#[instrument]
#[tokio::test]
#[with_tracing]
async fn change_password() {
    let (session, _device) = setup(Ssc::OpalV2).await;

    // Change the password of SID, authenticating in with the MSID password.
    let result = session
        .change_password(opal_admin::sp::ADMIN, opal_admin::authority::SID, INITIAL_SID_PASSWORD, NOT_DEFAULT)
        .await;
    assert_that!(result, ok(eq(&())));

    // Change the password of SID again, but this time authenticate with the newly set password.
    let result = session
        .change_password(opal_admin::sp::ADMIN, opal_admin::authority::SID, NOT_DEFAULT, NOT_DEFAULT)
        .await;
    assert_that!(result, ok(eq(&())));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_2(Ssc::OpalV2)]
#[case::opal_2(Ssc::Enterprise)]
async fn list_authorities_admin(#[case] ssc: Ssc) {
    let (session, _device) = setup(ssc).await;

    let result = session.list_authorities(session.spec().admin_sp()).await;
    assert_that!(result, ok(anything()));
    let authorities = result.unwrap();
    let actual_uids: BTreeSet<_> = authorities.iter().filter_map(|authority| authority.uid).collect();
    let expected_uids: BTreeSet<_> = _device.inspect(|tper| tper.admin.authority.keys().cloned().collect());
    assert_that!(actual_uids, eq(&expected_uids));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_2(Ssc::OpalV2)]
#[case::opal_2(Ssc::Enterprise)]
async fn list_authorities_locking(#[case] ssc: Ssc) {
    let (session, device) = setup(ssc).await;
    session.take_owneship(NOT_DEFAULT).await.unwrap();
    if ssc != Ssc::Enterprise {
        session.activate_secondary_sp(NOT_DEFAULT).await.unwrap();
    }

    let result = session.list_authorities(session.spec().secondary_sp()).await;
    assert_that!(result, ok(anything()));
    let authorities = result.unwrap();
    let actual_uids: BTreeSet<_> = authorities.iter().filter_map(|authority| authority.uid).collect();
    let expected_uids: BTreeSet<_> =
        device.inspect(|tper| tper.locking.as_ref().unwrap().authority.keys().cloned().collect());
    assert_that!(actual_uids, eq(&expected_uids));
}
