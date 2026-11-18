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
use sed_manager::{Alignment, DeviceSpec, Error, LockingConfigSession, LockingRangePlanError, SetupSession};
use sed_packet::MaxBytes;
use sed_packet::com_id::ComIdExt;
use sed_spec::ace_expr;
use sed_spec::objects::CPinRef;
use sed_spec::objects::{Authority, AuthorityRef, LockingRange, LockingRangeRef, MbrControl, MediaKeyRef};
use sed_spec::ssc::Ssc;
use sed_spec::ssc::Ssc::Enterprise;
use sed_spec::uid_registry::common::locking;
use sed_spec::uid_registry::core::shared::authority::ANYBODY;
use sed_spec::uid_registry::core::shared::mbr_control;
use sed_spec::uid_registry::enterprise;
use sed_spec::uid_registry::opal_2::admin::sp;
use sed_telemetry::with_tracing;
use sed_tper::Tper;
use sed_virtual_device::{BASE_COM_ID, LOGICAL_SECTOR_COUNT, VirtualDevice};
use tracing::instrument;

const NEW_SID_PASSWORD: MaxBytes<32> = MaxBytes::from_array(*b"new_sid_password");
const NOT_DEFAULT: MaxBytes<32> = MaxBytes::from_array(*b"not_default");

async fn setup_tper(ssc: Ssc) -> Tper {
    let (tper, _device) = setup(ssc).await;
    tper
}

async fn setup(ssc: Ssc) -> (Tper, Arc<VirtualDevice>) {
    let runtime = Arc::new(PolyRuntime::Tokio(TokioRuntime::current().unwrap()));
    let device = Arc::new(VirtualDevice::with_ssc(ssc));
    let tper = Tper::connect(BASE_COM_ID, ComIdExt(0), None, device.clone(), runtime);
    let spec = DeviceSpec::try_new(tper.discover_current().await.unwrap()).unwrap();
    let setup_session = SetupSession::new(&tper, spec);

    setup_session.take_owneship(NEW_SID_PASSWORD).await.unwrap();
    if ssc.activate_supported() {
        setup_session.activate_secondary_sp(NEW_SID_PASSWORD).await.unwrap();
    }

    (tper, device)
}

fn editing_authority(ssc: Ssc) -> AuthorityRef {
    match ssc {
        Ssc::Enterprise => enterprise::locking::authority::BAND_MASTER.get(1).unwrap(),
        _ => locking::authority::ADMIN.get(0).unwrap(),
    }
}

fn erasing_authority(ssc: Ssc) -> AuthorityRef {
    match ssc {
        Ssc::Enterprise => enterprise::locking::authority::ERASE_MASTER,
        _ => locking::authority::ADMIN.get(0).unwrap(),
    }
}

fn user_authority() -> AuthorityRef {
    locking::authority::USER.get(0).unwrap()
}

async fn login_for_editing(tper: &Tper) -> Result<LockingConfigSession, Error> {
    let spec = DeviceSpec::try_new(tper.discover_current().await.unwrap()).unwrap();
    let ssc = spec.ssc();
    let authority = editing_authority(ssc);
    LockingConfigSession::login(tper, spec, authority, Some(NEW_SID_PASSWORD)).await
}

async fn login_for_erase(tper: &Tper) -> Result<LockingConfigSession, Error> {
    let spec = DeviceSpec::try_new(tper.discover_current().await.unwrap()).unwrap();
    let ssc = spec.ssc();
    let authority = erasing_authority(ssc);
    LockingConfigSession::login(tper, spec, authority, Some(NEW_SID_PASSWORD)).await
}

async fn login_as_user1(tper: &Tper) -> Result<LockingConfigSession, Error> {
    let spec = DeviceSpec::try_new(tper.discover_current().await.unwrap()).unwrap();
    let authority = user_authority();
    LockingConfigSession::login(tper, spec, authority, Some(NEW_SID_PASSWORD)).await
}

fn enable_user1(device: &VirtualDevice) {
    device.modify(|tper| {
        if tper.ssc != Enterprise {
            let locking = tper.locking.as_mut().unwrap();
            let user1 = locking.authority.get_mut(&user_authority()).unwrap();
            user1.enabled = Some(true);
            let user1_pin_ref = CPinRef::try_from(user1.credential.unwrap()).unwrap();
            let user1_pin = locking.c_pin.get_mut(&user1_pin_ref).unwrap();
            user1_pin.pin = Some(NEW_SID_PASSWORD);
        }
    });
}

fn range_uid(index: usize) -> LockingRangeRef {
    locking::locking::RANGE.get(index).unwrap()
}

fn user_uid(index: usize) -> AuthorityRef {
    locking::authority::USER.get(index).unwrap()
}

fn make_range_from_span(index: usize, start: u64, length: u64) -> LockingRange {
    LockingRange {
        uid: Some(range_uid(index)),
        range_start: Some(start),
        range_length: Some(length),
        ..Default::default()
    }
}

fn find_range_by_uid(ranges: &[LockingRange], uid: LockingRangeRef) -> &LockingRange {
    ranges.iter().find(|range| range.uid == Some(uid)).expect("locking range not found")
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
async fn login_supported(#[case] ssc: Ssc) {
    let tper = setup_tper(ssc).await;

    let result = login_for_editing(&tper).await;
    assert_that!(result, ok(anything()));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::kpio(Ssc::KeyPerIO)]
async fn login_unsupported(#[case] ssc: Ssc) {
    let tper = setup_tper(ssc).await;

    let result = login_for_editing(&tper).await;
    assert_that!(result, err(eq(&Error::IncompatibleSsc)));
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
async fn login_wrong_password(#[case] ssc: Ssc) {
    let tper = setup_tper(ssc).await;
    let wrong_password = MaxBytes::<32>::from_array(*b"wrong_password");

    let admin1 = editing_authority(ssc);
    let spec = DeviceSpec::try_new(tper.discover_current().await.unwrap()).unwrap();
    let result = LockingConfigSession::login(&tper, spec, admin1, Some(wrong_password)).await;
    assert_that!(result, err(anything()));
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
async fn get_authorities_by_editor(#[case] ssc: Ssc) {
    let (tper, device) = setup(ssc).await;

    let session = login_for_editing(&tper).await.unwrap();
    let authorities = session.get_authorities().await.unwrap();

    let actual = authorities.iter().map(|authority| *authority.uid.as_ref().unwrap()).collect::<BTreeSet<_>>();
    let expected =
        device.inspect(|tper| tper.locking.as_ref().unwrap().authority.keys().cloned().collect::<BTreeSet<_>>());
    assert_that!(actual, eq(&expected));
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
async fn get_authorities_by_user(#[case] ssc: Ssc) {
    let (tper, device) = setup(ssc).await;
    enable_user1(&device);

    let session = login_as_user1(&tper).await.unwrap();
    let authorities = session.get_authorities().await.unwrap();

    let actual = authorities.iter().map(|authority| *authority.uid.as_ref().unwrap()).collect::<BTreeSet<_>>();
    let expected =
        device.inspect(|tper| tper.locking.as_ref().unwrap().authority.keys().cloned().collect::<BTreeSet<_>>());
    assert_that!(actual, eq(&expected));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
#[case::ruby(Ssc::Ruby)]
async fn get_locking_ranges_by_editor(#[case] ssc: Ssc) {
    let (tper, device) = setup(ssc).await;

    let session = login_for_editing(&tper).await.unwrap();
    let locking_ranges = session.get_locking_ranges().await.unwrap();

    let actual = locking_ranges.iter().map(|lr| *lr.uid.as_ref().unwrap()).collect::<BTreeSet<_>>();
    let expected =
        device.inspect(|tper| tper.locking.as_ref().unwrap().locking.keys().cloned().collect::<BTreeSet<_>>());
    assert_that!(actual, eq(&expected));
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
async fn get_locking_ranges_by_user(#[case] ssc: Ssc) {
    let (tper, device) = setup(ssc).await;
    enable_user1(&device);

    let session = login_as_user1(&tper).await.unwrap();
    let locking_ranges = session.get_locking_ranges().await.unwrap();

    let actual = locking_ranges.iter().map(|lr| *lr.uid.as_ref().unwrap()).collect::<BTreeSet<_>>();
    let expected =
        device.inspect(|tper| tper.locking.as_ref().unwrap().locking.keys().cloned().collect::<BTreeSet<_>>());
    assert_that!(actual, eq(&expected));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise, Err(Error::NotSupported))]
#[case::opal_1(Ssc::OpalV1, Ok(128 * 1048576))]
#[case::opal_2(Ssc::OpalV2, Ok(128 * 1048576))]
#[case::opalite(Ssc::Opalite, Ok(128 * 1048576))]
#[case::pyrite_1(Ssc::PyriteV1, Ok(128 * 1048576))]
#[case::pyrite_2(Ssc::PyriteV2, Ok(128 * 1048576))]
#[case::ruby(Ssc::Ruby, Ok(128 * 1048576))]
async fn get_mbr_size_by_editor(#[case] ssc: Ssc, #[case] expected: Result<u32, Error>) {
    let tper = setup_tper(ssc).await;

    let session = login_for_editing(&tper).await.unwrap();
    let result = session.get_mbr_size().await;
    assert_that!(result, eq(&expected));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_1(Ssc::OpalV1, Ok(128 * 1048576))]
#[case::opal_2(Ssc::OpalV2, Ok(128 * 1048576))]
#[case::opalite(Ssc::Opalite, Ok(128 * 1048576))]
#[case::pyrite_1(Ssc::PyriteV1, Ok(128 * 1048576))]
#[case::pyrite_2(Ssc::PyriteV2, Ok(128 * 1048576))]
#[case::ruby(Ssc::Ruby, Ok(128 * 1048576))]
async fn get_mbr_size_by_user(#[case] ssc: Ssc, #[case] expected: Result<u32, Error>) {
    let (tper, device) = setup(ssc).await;
    enable_user1(&device);

    let session = login_as_user1(&tper).await.unwrap();
    let result = session.get_mbr_size().await;
    assert_that!(result, eq(&expected));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise, Err(Error::NotSupported))]
#[case::opal_1(Ssc::OpalV1, Ok(Some(false)))]
#[case::opal_2(Ssc::OpalV2, Ok(Some(false)))]
#[case::opalite(Ssc::Opalite, Ok(Some(false)))]
#[case::pyrite_1(Ssc::PyriteV1, Ok(Some(false)))]
#[case::pyrite_2(Ssc::PyriteV2, Ok(Some(false)))]
#[case::ruby(Ssc::Ruby, Ok(Some(false)))]
async fn get_mbr_control(#[case] ssc: Ssc, #[case] enable_expected: Result<Option<bool>, Error>) {
    let (tper, device) = setup(ssc).await;
    enable_user1(&device);

    {
        let session = login_for_editing(&tper).await.unwrap();
        let result = session.get_mbr_control().await;
        assert_that!(result.map(|mbr_control| mbr_control.enable), eq(&enable_expected));
        session.close().await.unwrap();
    }

    if ssc != Ssc::Enterprise {
        let session = login_as_user1(&tper).await.unwrap();
        let result = session.get_mbr_control().await;
        assert_that!(result.map(|mbr_control| mbr_control.enable), eq(&enable_expected));
        session.close().await.unwrap();
    }
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise, Alignment{ alignment_required: false, alignment_granularity: 1, lowest_aligned_lba: 0 })]
#[case::opal_1(Ssc::OpalV1, Alignment{ alignment_required: false, alignment_granularity: 1, lowest_aligned_lba: 0 })]
#[case::opal_2(Ssc::OpalV2, Alignment{ alignment_required: true, alignment_granularity: 8, lowest_aligned_lba: 0 })]
#[case::opalite(Ssc::Opalite, Alignment{ alignment_required: true, alignment_granularity: 8, lowest_aligned_lba: 0 })]
#[case::pyrite_1(Ssc::PyriteV1, Alignment{ alignment_required: true, alignment_granularity: 8, lowest_aligned_lba: 0 })]
#[case::pyrite_2(Ssc::PyriteV2, Alignment{ alignment_required: true, alignment_granularity: 8, lowest_aligned_lba: 0 })]
#[case::ruby(Ssc::Ruby, Alignment{ alignment_required: true, alignment_granularity: 8, lowest_aligned_lba: 0 })]
async fn get_alignment(#[case] ssc: Ssc, #[case] expected: Alignment) {
    let (tper, device) = setup(ssc).await;
    enable_user1(&device);

    {
        let session = login_for_editing(&tper).await.unwrap();
        let result = session.get_alignment().await;
        assert_that!(result, ok(eq(&expected)));
        session.close().await.unwrap();
    }

    if ssc != Ssc::Enterprise {
        let session = login_as_user1(&tper).await.unwrap();
        let result = session.get_alignment().await;
        assert_that!(result, ok(eq(&expected)));
        session.close().await.unwrap();
    }
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::ruby(Ssc::Ruby)]
async fn set_locking_ranges_multi_range(#[case] ssc: Ssc) {
    let tper = setup_tper(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    let current = session.get_locking_ranges().await.unwrap();
    let desired = [
        make_range_from_span(0, 64, 128),
        LockingRange { read_lock_enabled: Some(true), write_locked: Some(true), ..make_range_from_span(1, 256, 64) },
    ];
    let result = session.set_locking_ranges(Some(&current), &desired, LOGICAL_SECTOR_COUNT).await;
    assert_that!(result, ok(anything()));

    let updated = session.get_locking_ranges().await.unwrap();
    let range_1 = find_range_by_uid(&updated, range_uid(0));
    let range_2 = find_range_by_uid(&updated, range_uid(1));
    assert_eq!((range_1.range_start, range_1.range_length), (Some(64), Some(128)));
    assert_eq!((range_2.range_start, range_2.range_length), (Some(256), Some(64)));
    assert_eq!(range_2.read_lock_enabled, Some(true));
    assert_eq!(range_2.write_locked, Some(true));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opalite(Ssc::Opalite)]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
async fn set_locking_ranges_single_range(#[case] ssc: Ssc) {
    use locking::locking::GLOBAL_RANGE;

    let tper = setup_tper(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    let current = session.get_locking_ranges().await.unwrap();
    let desired = [LockingRange {
        uid: Some(GLOBAL_RANGE),
        read_lock_enabled: Some(true),
        write_locked: Some(true),
        ..Default::default()
    }];
    let result = session.set_locking_ranges(Some(&current), &desired, LOGICAL_SECTOR_COUNT).await;
    assert_that!(result, ok(anything()));

    let updated = session.get_locking_ranges().await.unwrap();
    let global_range = updated.first().unwrap();
    assert_eq!(global_range.read_lock_enabled, Some(true));
    assert_eq!(global_range.write_locked, Some(true));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
async fn set_locking_ranges_band(#[case] ssc: Ssc) {
    let band = enterprise::locking::locking::BAND.get(0).unwrap();

    let tper = setup_tper(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    let current = session.get_locking_ranges().await.unwrap();
    let desired = [LockingRange {
        uid: Some(band),
        read_lock_enabled: Some(true),
        write_locked: Some(true),
        ..Default::default()
    }];
    let result = session.set_locking_ranges(Some(&current), &desired, LOGICAL_SECTOR_COUNT).await;
    assert_that!(result, ok(anything()));

    let updated = session.get_locking_ranges().await.unwrap();
    let band = find_range_by_uid(&updated, band);
    assert_eq!(band.read_lock_enabled, Some(true));
    assert_eq!(band.write_locked, Some(true));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::ruby(Ssc::Ruby)]
async fn set_locking_ranges_swap(#[case] ssc: Ssc) {
    let tper = setup_tper(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    session
        .set_locking_ranges(
            None,
            &[
                make_range_from_span(0, 64, 64),
                make_range_from_span(1, 128, 64),
            ],
            LOGICAL_SECTOR_COUNT,
        )
        .await
        .unwrap();

    // The ranges are only allowed to overlap in the final state, not in the
    // intermediate ones, so this requires collapsing both before moving them.
    let result = session
        .set_locking_ranges(
            None,
            &[
                make_range_from_span(0, 128, 64),
                make_range_from_span(1, 64, 64),
            ],
            LOGICAL_SECTOR_COUNT,
        )
        .await;
    assert_that!(result, ok(anything()));

    let updated = session.get_locking_ranges().await.unwrap();
    let range_1 = find_range_by_uid(&updated, range_uid(0));
    let range_2 = find_range_by_uid(&updated, range_uid(1));
    assert_eq!((range_1.range_start, range_1.range_length), (Some(128), Some(64)));
    assert_eq!((range_2.range_start, range_2.range_length), (Some(64), Some(64)));
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
async fn set_locking_ranges_stale_expected(#[case] ssc: Ssc) {
    let tper = setup_tper(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    let current = session.get_locking_ranges().await.unwrap();

    let mut stale = current.clone();
    stale[0].common_name = Some("stale".into());
    let stale_uid = stale[0].uid.unwrap();

    let result = session.set_locking_ranges(Some(&stale), &[current[0].clone()], LOGICAL_SECTOR_COUNT).await;
    assert_that!(
        result,
        err(eq(&Error::LockingRangeUpdateRejected(LockingRangePlanError::CurrentNotMatchingExpected {
            uid: stale_uid
        })))
    );
    assert_that!(session.get_locking_ranges().await, ok(eq(&current)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::ruby(Ssc::Ruby)]
async fn set_locking_ranges_invalid_desired(#[case] ssc: Ssc) {
    let tper = setup_tper(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    let current = session.get_locking_ranges().await.unwrap();
    let result = session
        .set_locking_ranges(
            None,
            &[
                make_range_from_span(0, 64, 128),
                make_range_from_span(1, 128, 64),
            ],
            LOGICAL_SECTOR_COUNT,
        )
        .await;
    assert_that!(
        result,
        err(eq(&Error::LockingRangeUpdateRejected(LockingRangePlanError::OverlappingRanges {
            first: range_uid(0),
            second: range_uid(1)
        })))
    );
    assert_that!(session.get_locking_ranges().await, ok(eq(&current)));
}

fn get_media_encryption_key(virtual_device: &VirtualDevice, range: LockingRangeRef) -> Option<heapless::Vec<u8, 64>> {
    virtual_device.inspect(|tper| {
        let locking_sp = tper.locking.as_ref().unwrap();
        let key_ref = locking_sp.locking[&range].active_key.unwrap();
        match key_ref {
            MediaKeyRef::Aes128(key_ref) => locking_sp.k_aes_128.as_ref().unwrap()[&key_ref]
                .key
                .as_ref()
                .map(|key| heapless::Vec::<u8, 64>::from_slice(key.as_slice()).unwrap()),
            MediaKeyRef::Aes256(key_ref) => locking_sp.k_aes_256.as_ref().unwrap()[&key_ref].key.clone(),
        }
    })
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::ruby(Ssc::Ruby)]
async fn erase_locking_range_with_media_encryption(#[case] ssc: Ssc) {
    let range = locking::locking::GLOBAL_RANGE;
    let (tper, device) = setup(ssc).await;
    let session = login_for_erase(&tper).await.unwrap();

    let ranges_before = session.get_locking_ranges().await.unwrap();
    let key_before = get_media_encryption_key(&device, range);
    assert_that!(key_before, some(anything()));

    session.erase_locking_range(range).await.unwrap();

    assert_that!(get_media_encryption_key(&device, range), not(eq(&key_before)));
    assert_that!(session.get_locking_ranges().await, ok(eq(&ranges_before)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::opal_1(Ssc::OpalV1)]
#[case::opal_2(Ssc::OpalV2)]
#[case::opalite(Ssc::Opalite)]
#[case::ruby(Ssc::Ruby)]
async fn erase_locking_range_by_user(#[case] ssc: Ssc) {
    let range = locking::locking::GLOBAL_RANGE;
    let (tper, device) = setup(ssc).await;
    enable_user1(&device);

    // Allow everyone to erase the global range.
    device.modify(|tper| {
        let locking_sp = tper.locking.as_mut().unwrap();
        locking_sp.ace.get_mut(&locking::ace::K_AES_256_GLOBAL_RANGE_GEN_KEY).unwrap().boolean_expr =
            Some(ace_expr!(ANYBODY));
    });

    let session = login_as_user1(&tper).await.unwrap();

    let ranges_before = session.get_locking_ranges().await.unwrap();
    let key_before = get_media_encryption_key(&device, range);
    assert_that!(key_before, some(anything()));

    session.erase_locking_range(range).await.unwrap();

    assert_that!(get_media_encryption_key(&device, range), not(eq(&key_before)));
    assert_that!(session.get_locking_ranges().await, ok(eq(&ranges_before)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::pyrite_1(Ssc::PyriteV1)]
#[case::pyrite_2(Ssc::PyriteV2)]
async fn erase_locking_range_no_media_encryption(#[case] ssc: Ssc) {
    let range = locking::locking::GLOBAL_RANGE;
    let (tper, _device) = setup(ssc).await;
    let session = login_for_erase(&tper).await.unwrap();

    let result = session.erase_locking_range(range).await;
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
async fn set_authorities(#[case] ssc: Ssc) {
    let tper = setup_tper(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    let uid = user_uid(1);

    let current = session.get_authorities().await.unwrap();
    let desired = [Authority { uid: Some(uid), enabled: Some(true), ..Default::default() }];
    let result = session.set_authorities(Some(&current), &desired).await;
    assert_that!(result, ok(anything()));

    let updated = session.get_authorities().await.unwrap();
    let updated_auth = updated.iter().find(|authority| authority.uid == Some(uid)).unwrap();
    assert_eq!(updated_auth.enabled, Some(true));
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
async fn change_password_other_authority(#[case] ssc: Ssc) {
    let tper = setup_tper(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    let authority = user_uid(1);

    session.change_password(authority, NOT_DEFAULT).await.unwrap();
    session.close().await.unwrap();

    assert_that!(tper.start_session(sp::LOCKING, Some(authority), Some(NOT_DEFAULT)).await, ok(anything()));
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
async fn change_password_self(#[case] ssc: Ssc) {
    let (tper, device) = setup(ssc).await;
    enable_user1(&device);

    let authority = match ssc {
        Ssc::Enterprise => enterprise::locking::authority::ERASE_MASTER,
        _ => user_uid(0),
    };

    let spec = DeviceSpec::try_new(tper.discover_current().await.unwrap()).unwrap();
    let session = LockingConfigSession::login(&tper, spec.clone(), authority, Some(NEW_SID_PASSWORD)).await.unwrap();
    session.change_password(authority, NOT_DEFAULT).await.unwrap();
    session.close().await.unwrap();

    LockingConfigSession::login(&tper, spec, authority, Some(NOT_DEFAULT)).await.unwrap();
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
async fn set_mbr_control_supported(#[case] ssc: Ssc) {
    let (tper, device) = setup(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    let mbr_control =
        MbrControl { uid: Some(mbr_control::MBR_CONTROL), enable: Some(true), done: Some(true), done_on_reset: None };

    let get_mbr_control = |tper: &sed_virtual_device::Tper| {
        tper.locking.as_ref().unwrap().mbr_control.as_ref().unwrap().first_key_value().unwrap().1.clone()
    };

    assert_that!(device.inspect(|tper| get_mbr_control(tper).enable), some(eq(false)));
    assert_that!(device.inspect(|tper| get_mbr_control(tper).done), some(eq(false)));

    session.set_mbr_control(mbr_control).await.unwrap();

    assert_that!(device.inspect(|tper| get_mbr_control(tper).enable), some(eq(true)));
    assert_that!(device.inspect(|tper| get_mbr_control(tper).done), some(eq(true)));
}

#[instrument]
#[tokio::test]
#[with_tracing]
#[rstest]
#[case::enterprise(Ssc::Enterprise)]
async fn set_mbr_control_unsupported(#[case] ssc: Ssc) {
    let (tper, _device) = setup(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    let mbr_control =
        MbrControl { uid: Some(mbr_control::MBR_CONTROL), enable: Some(true), done: Some(true), done_on_reset: None };
    // We accept the raw TPer error instead of `Unsupported`, because this is a
    // single `Set` that might very well be denied for legitimate reasons. As,
    // such we cannot reliable detect the missing MBR from the single call, and
    // making more calls would be a bit wasteful.
    assert_that!(session.set_mbr_control(mbr_control).await, err(anything()));
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
async fn write_mbr(#[case] ssc: Ssc) {
    let (tper, device) = setup(ssc).await;
    let session = login_for_editing(&tper).await.unwrap();

    let data: Vec<_> = (1..987154_u32).map(|value| value as u8).collect();
    session.write_mbr(500, &data).await.unwrap();

    device.inspect(|tper| {
        assert!(tper.locking.as_ref().unwrap().mbr.as_ref().unwrap()[0..500].iter().all(|x| *x == 0));
        assert!(tper.locking.as_ref().unwrap().mbr.as_ref().unwrap()[500..987653] == data);
        assert!(tper.locking.as_ref().unwrap().mbr.as_ref().unwrap()[987653..988154].iter().all(|x| *x == 0));
    });
}
