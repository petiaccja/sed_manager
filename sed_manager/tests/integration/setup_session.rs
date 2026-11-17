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
use sed_packet::discovery::BlockSIDAuthDescriptor;
use sed_spec::ssc::Ssc;
use sed_spec::uid_registry::common::{admin, locking};
use sed_spec::uid_registry::enterprise;
use sed_spec::uid_registry::kpio;
use sed_spec::uid_registry::psid;
use sed_telemetry::with_tracing;
use sed_tper::Tper;
use sed_virtual_device::{BASE_COM_ID, INITIAL_SID_PASSWORD, PSID_PASSWORD, VirtualDevice};
use tracing::instrument;

const NOT_DEFAULT: MaxBytes<32> = MaxBytes::from_array(*b"not_default");

async fn setup_tper(ssc: Ssc) -> (SetupSession, Tper, Arc<VirtualDevice>) {
    let device = Arc::new(VirtualDevice::with_ssc(ssc));
    let runtime = Arc::new(PolyRuntime::Tokio(TokioRuntime::current().unwrap()));
    let tper = Tper::connect(BASE_COM_ID, ComIdExt(0), None, device.clone(), runtime);
    let spec = DeviceSpec::try_new(tper.discover_current().await.unwrap()).unwrap();
    let session = SetupSession::new(&tper, spec);
    (session, tper, device)
}

async fn setup(ssc: Ssc) -> (SetupSession, Arc<VirtualDevice>) {
    let (session, _, device) = setup_tper(ssc).await;
    (session, device)
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
async fn take_ownership(#[case] ssc: Ssc) {
    let (session, tper, device) = setup_tper(ssc).await;

    let block_sid_before = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid_before.sid_msid_pin_differ, eq(false));

    let result = session.take_owneship(NOT_DEFAULT).await;
    assert_that!(result, ok(anything()));

    let block_sid_after = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid_after.sid_msid_pin_differ, eq(true));

    if ssc == Ssc::Enterprise {
        // Make sure the SID PIN was copied to all bands.
        tper.start_session(
            enterprise::admin::sp::LOCKING,
            Some(enterprise::locking::authority::ERASE_MASTER),
            Some(NOT_DEFAULT),
        )
        .await
        .unwrap();
    }
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
async fn take_ownership_already_owned(#[case] ssc: Ssc) {
    let (session, _device) = setup(ssc).await;

    session.take_owneship(NOT_DEFAULT).await.unwrap();
    let result = session.take_owneship(NOT_DEFAULT).await;
    assert_that!(result, err(eq(&Error::AlreadyOwned)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
async fn activate_secondary_sp(#[case] ssc: Ssc) {
    let (session, device) = setup(ssc).await;

    let activated_before = device.discover().is_secondary_sp_enabled();
    assert_that!(activated_before, some(eq(false)));

    let result = session.activate_secondary_sp(INITIAL_SID_PASSWORD).await;
    assert_that!(result, ok(anything()));

    let activated_after = device.discover().is_secondary_sp_enabled();
    assert_that!(activated_after, some(eq(true)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
async fn activate_secondary_sp_already_activated(#[case] ssc: Ssc) {
    let (session, _device) = setup(ssc).await;

    // Enterprise is always activated.
    if ssc != Ssc::Enterprise {
        session.activate_secondary_sp(INITIAL_SID_PASSWORD).await.unwrap();
    }
    let result = session.activate_secondary_sp(INITIAL_SID_PASSWORD).await;
    assert_that!(result, err(eq(&Error::AlreadyActivated)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
async fn revert_tper_with_sid_supported(#[case] ssc: Ssc) {
    let new_sid_password = NOT_DEFAULT;
    let (session, device) = setup(ssc).await;

    session.take_owneship(new_sid_password.clone()).await.unwrap();
    session.activate_secondary_sp(new_sid_password.clone()).await.unwrap();
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(true)));

    let result = session.revert_tper(admin::authority::SID, new_sid_password).await;
    assert_that!(result, ok(anything()));

    let block_sid = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid.sid_msid_pin_differ, eq(false));
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(false)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
async fn revert_tper_with_sid_unsupported(#[case] ssc: Ssc) {
    let new_sid_password = NOT_DEFAULT;
    let (session, device) = setup(ssc).await;

    session.take_owneship(new_sid_password.clone()).await.unwrap();
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(true)));

    let result = session.revert_tper(admin::authority::SID, new_sid_password).await;
    assert_that!(result, err(eq(&Error::NotSupported)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
async fn revert_tper_with_psid_supported(#[case] ssc: Ssc) {
    let new_sid_password = NOT_DEFAULT;
    let (session, device) = setup(ssc).await;

    session.take_owneship(new_sid_password.clone()).await.unwrap();
    session.activate_secondary_sp(new_sid_password).await.unwrap();
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(true)));

    let result = session.revert_tper(psid::admin::authority::PSID, PSID_PASSWORD).await;
    assert_that!(result, ok(anything()));

    let block_sid = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid.sid_msid_pin_differ, eq(false));
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(false)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::pyrite_1(Ssc::PyriteV1)]
async fn revert_tper_with_psid_unsupported(#[case] ssc: Ssc) {
    let new_sid_password = NOT_DEFAULT;
    let (session, device) = setup(ssc).await;

    session.take_owneship(new_sid_password.clone()).await.unwrap();
    if ssc != Ssc::Enterprise {
        session.activate_secondary_sp(new_sid_password).await.unwrap();
    }
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(true)));

    let result = session.revert_tper(psid::admin::authority::PSID, PSID_PASSWORD).await;
    assert_that!(result, err(eq(&Error::NotSupported)));

    let block_sid = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid.sid_msid_pin_differ, eq(true));
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(true)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
async fn revert_secondary_sp_supported(#[case] ssc: Ssc) {
    let new_sid_password = NOT_DEFAULT;
    let (session, device) = setup(ssc).await;

    session.take_owneship(new_sid_password.clone()).await.unwrap();
    session.activate_secondary_sp(new_sid_password.clone()).await.unwrap();
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(true)));

    let result = session.revert_secondary_sp(new_sid_password).await;
    assert_that!(result, ok(anything()));

    // Secondary SP is reverted.
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(false)));

    // Admin SP is unaffected: the changed SID password persists.
    let block_sid = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid.sid_msid_pin_differ, eq(true));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
async fn revert_secondary_sp_unsupported(#[case] ssc: Ssc) {
    let new_sid_password = NOT_DEFAULT;
    let (session, device) = setup(ssc).await;

    session.take_owneship(new_sid_password.clone()).await.unwrap();
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(true)));

    let result = session.revert_secondary_sp(new_sid_password).await;
    assert_that!(result, err(eq(&Error::NotSupported)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
async fn revert_secondary_sp_ex_supported(#[case] ssc: Ssc) {
    let (session, device) = setup(ssc).await;

    session.take_owneship(NOT_DEFAULT).await.unwrap();
    session.activate_secondary_sp(NOT_DEFAULT).await.unwrap();
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(true)));

    let admin1 = locking::authority::ADMIN.get(0).unwrap();
    let result = session.revert_secondary_sp_ex(admin1, NOT_DEFAULT, None).await;
    assert_that!(result, ok(anything()));

    // Secondary SP is reverted.
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(false)));

    // Admin SP is unaffected: the changed SID password persists.
    let block_sid = device.discover().get::<BlockSIDAuthDescriptor>().unwrap().clone();
    assert_that!(block_sid.sid_msid_pin_differ, eq(true));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::kpio(Ssc::KeyPerIO)]
async fn revert_secondary_sp_ex_unsupported(#[case] ssc: Ssc) {
    let (session, device) = setup(ssc).await;

    session.take_owneship(NOT_DEFAULT).await.unwrap();
    if ssc != Ssc::Enterprise {
        session.activate_secondary_sp(NOT_DEFAULT).await.unwrap();
    }
    assert_that!(device.discover().is_secondary_sp_enabled(), some(eq(true)));

    let admin1 = locking::authority::ADMIN.get(0).unwrap();
    let result = session.revert_secondary_sp_ex(admin1, NOT_DEFAULT, None).await;
    assert_that!(result, err(eq(&Error::NotSupported)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
async fn change_password_admin_sp(#[case] ssc: Ssc) {
    let (session, _device) = setup(ssc).await;
    let sp = session.spec().admin_sp();
    let authority = admin::authority::SID;

    // Change the password of SID, authenticating in with the MSID password.
    let result = session.change_password(sp, authority, INITIAL_SID_PASSWORD, NOT_DEFAULT).await;
    assert_that!(result, ok(eq(&())));

    // Change the password of SID again, but this time authenticate with the newly set password.
    let result = session.change_password(sp, authority, NOT_DEFAULT, NOT_DEFAULT).await;
    assert_that!(result, ok(eq(&())));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
async fn change_password_secondary_sp(#[case] ssc: Ssc) {
    let (session, _device) = setup(ssc).await;
    if ssc != Ssc::Enterprise {
        session.activate_secondary_sp(INITIAL_SID_PASSWORD).await.unwrap();
    }

    let sp = session.spec().secondary_sp();
    let authority = match ssc {
        Ssc::Enterprise => enterprise::locking::authority::ERASE_MASTER,
        Ssc::KeyPerIO => kpio::key_per_io::authority::ADMIN.get(0).unwrap(),
        _ => locking::authority::ADMIN.get(0).unwrap(),
    };

    // Change the password of SID, authenticating in with the MSID password.
    let result = session.change_password(sp, authority, INITIAL_SID_PASSWORD, NOT_DEFAULT).await;
    assert_that!(result, ok(eq(&())));

    // Change the password of SID again, but this time authenticate with the newly set password.
    let result = session.change_password(sp, authority, NOT_DEFAULT, NOT_DEFAULT).await;
    assert_that!(result, ok(eq(&())));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
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
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
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
    let locking_uids =
        device.inspect(|tper| tper.locking.as_ref().map(|sp| sp.authority.keys().cloned().collect::<BTreeSet<_>>()));
    let kpio_uids =
        device.inspect(|tper| tper.kpio.as_ref().map(|sp| sp.authority.keys().cloned().collect::<BTreeSet<_>>()));
    let expected_uids = locking_uids.or(kpio_uids).unwrap();
    assert_that!(actual_uids, eq(&expected_uids));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
#[case::kpio(Ssc::KeyPerIO)]
async fn is_psid_supported(#[case] ssc: Ssc) {
    let (session, device) = setup(ssc).await;
    let expected = device.inspect(|tper| tper.admin.authority.contains_key(&psid::admin::authority::PSID));
    assert_that!(session.is_psid_supported().await, ok(eq(&expected)));
}
