//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

mod admin_sp;
mod kpio_sp;
mod locking_sp;
mod preconfig;
mod security_provider;

use std::marker::PhantomData;
use std::str::FromStr;

pub use admin_sp::Admin;
pub use kpio_sp::KeyPerIo;
pub use locking_sp::Locking;
pub use preconfig::{INITIAL_SID_PASSWORD, PSID_PASSWORD};
pub use security_provider::{SecurityProvider, Table};
use sed_packet::discovery::{
    BlockSIDAuthDescriptor, Discovery, EnterpriseDescriptor, FeatureDescriptor, GeometryDescriptor, KeyPerIODescriptor,
    LockingDescriptor, OpalV1Descriptor, OpalV2Descriptor, OpaliteDescriptor, OwnerPasswordState, PyriteV1Descriptor,
    PyriteV2Descriptor, RubyDescriptor, TperDescriptor,
};
use sed_spec::methods::{MethodStatus, Properties};
use sed_spec::objects::SecurityProviderRef;
use sed_spec::preconfig::{ENTERPRISE, KPIO, OPAL_1, OPAL_2, OPALITE, PYRITE_1, PYRITE_2, RUBY};
use sed_spec::ssc::Ssc;
use sed_spec::types::LifeCycleState;
use sed_spec::uid_registry::opal_2::admin::c_pin;
use sed_spec::uid_registry::{kpio, opal_2};
use sorbit::ser_de::ToBytes;

use crate::device::{BASE_COM_ID, NUM_COM_IDS};
use crate::internal_error::Expect;
use crate::tper::preconfig::resolve_paths;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Tper {
    pub ssc: Ssc,
    pub admin: Admin,
    pub locking: Option<Locking>,
    pub kpio: Option<KeyPerIo>,
}

impl Tper {
    pub fn from_preconfig(ssc: Ssc) -> Self {
        let raw_json_str = match ssc {
            Ssc::Enterprise => ENTERPRISE,
            Ssc::OpalV1 => OPAL_1,
            Ssc::OpalV2 => OPAL_2,
            Ssc::Opalite => OPALITE,
            Ssc::PyriteV1 => PYRITE_1,
            Ssc::PyriteV2 => PYRITE_2,
            Ssc::Ruby => RUBY,
            Ssc::KeyPerIO => KPIO,
        };
        let raw_json_value =
            serde_json::Value::from_str(raw_json_str).expect("the JSON syntax is ensured by unit tests");
        let json_value = resolve_paths(raw_json_value, ssc).expect("the UID path resolution is ensured by unit tests");
        let mut tper: Self = serde_json::from_value(json_value).expect("the schema is ensured by unit tests");
        tper.ssc = ssc;
        if let Some(locking) = tper.locking.as_mut() {
            locking.resize_mbr();
            locking.resize_data_store();
        }
        if let Some(kpio) = tper.kpio.as_mut() {
            kpio.resize_data_store();
        }
        tper
    }

    pub fn sp(&self, uid: SecurityProviderRef) -> Option<&dyn SecurityProvider> {
        let admin = &self.admin as &dyn SecurityProvider;
        let locking = self.locking.as_ref().map(|locking| locking as &dyn SecurityProvider);
        let kpio = self.kpio.as_ref().map(|kpio| kpio as &dyn SecurityProvider);
        std::iter::once(admin).chain(locking).chain(kpio).find(|sp| sp.uid() == uid)
    }

    pub fn sp_mut(&mut self, uid: SecurityProviderRef) -> Option<&mut dyn SecurityProvider> {
        let admin = &mut self.admin as &mut dyn SecurityProvider;
        let locking = self.locking.as_mut().map(|locking| locking as &mut dyn SecurityProvider);
        let kpio = self.kpio.as_mut().map(|kpio| kpio as &mut dyn SecurityProvider);
        std::iter::once(admin).chain(locking).chain(kpio).find(|sp| sp.uid() == uid)
    }

    pub fn restore_preconfig(&mut self, uid: SecurityProviderRef) -> Result<Vec<SecurityProviderRef>, MethodStatus> {
        if uid == self.admin.uid {
            let preconfig = Self::from_preconfig(self.ssc);
            *self = preconfig;
            Ok(std::iter::once(self.admin.uid)
                .chain(self.locking.as_ref().map(|sp| sp.uid))
                .chain(self.kpio.as_ref().map(|sp| sp.uid))
                .collect())
        } else if let Some(locking) = self.locking.as_mut()
            && locking.uid == uid
        {
            let preconfig = Self::from_preconfig(self.ssc);
            if let Some(locking_sp) = self.admin.sp.get_mut(&locking.uid) {
                locking_sp.life_cycle_state = Some(LifeCycleState::ManufacturedInactive);
            }
            self.locking = preconfig.locking;
            Ok(vec![uid])
        } else if let Some(kpio) = self.kpio.as_mut()
            && kpio.uid == uid
        {
            let preconfig = Self::from_preconfig(self.ssc);
            if let Some(kpio_sp) = self.admin.sp.get_mut(&kpio.uid) {
                kpio_sp.life_cycle_state = Some(LifeCycleState::ManufacturedInactive);
            }
            self.kpio = preconfig.kpio;
            Ok(vec![uid])
        } else {
            Err(MethodStatus::InvalidParameter)
        }
    }

    pub fn resize_byte_tables(&mut self) {
        if let Some(locking) = &mut self.locking {
            locking.resize_mbr();
            locking.resize_data_store();
        }
        if let Some(kpio) = &mut self.kpio {
            kpio.resize_data_store();
        }
    }

    pub fn discover(&self, capabilities: &Properties) -> Discovery {
        let fixed = [
            Self::tper_feature_desc(capabilities),
            self.ssc_feature_desc(),
            Self::geometry_feature_desc(),
            self.block_sid_authentication_desc(),
        ];
        let locking = self.locking_feature_desc();
        Discovery { feature_descriptors: fixed.into_iter().chain(locking).collect(), ..Default::default() }
    }

    pub fn pop_discovery(&self, capabilities: &Properties) -> Vec<u8> {
        self.discover(capabilities).to_bytes().expect("serializing discovery failed")
    }

    fn tper_feature_desc(capabilities: &Properties) -> FeatureDescriptor {
        let desc = TperDescriptor {
            version: PhantomData,
            length: PhantomData,
            sync_supported: true,
            async_supported: capabilities.asynchronous,
            ack_nak_supported: capabilities.ack_nak,
            buffer_mgmt_supported: false,
            streaming_supported: true,
            com_id_mgmt_supported: false,
        };
        FeatureDescriptor::TPer(desc)
    }

    fn locking_feature_desc(&self) -> Option<FeatureDescriptor> {
        self.locking.as_ref().map(|locking_sp| {
            let locking_enabled = self.is_sp_enabled(locking_sp.uid);

            let locked = locking_sp
                .locking
                .values()
                .any(|range| range.read_locked.unwrap_or(false) || range.write_locked.unwrap_or(false));

            let mbr_control = locking_sp.mbr_control.as_ref().and_then(|mbr_control| mbr_control.first_key_value());
            let mbr_enabled = mbr_control.and_then(|(_, mbr_control)| mbr_control.enable).unwrap_or(false);
            let mbr_done = mbr_control.and_then(|(_, mbr_control)| mbr_control.done).unwrap_or(false);
            let mbr_shadowing_supported = mbr_control.is_some() || self.ssc == Ssc::Enterprise;

            FeatureDescriptor::Locking(LockingDescriptor {
                version: PhantomData,
                length: PhantomData,
                hw_reset_supported: true,
                locked,
                locking_enabled,
                locking_supported: true,
                media_encryption: true,
                mbr_enabled,
                mbr_done,
                mbr_shadowing_not_supported: !mbr_shadowing_supported,
            })
        })
    }

    fn is_sp_enabled(&self, sp: SecurityProviderRef) -> bool {
        let admin_sp = &self.admin;
        let life_cycle_state = admin_sp.sp.get(&sp).and_then(|sp_obj| sp_obj.life_cycle_state).expect_sp(sp);

        [LifeCycleState::Manufactured, LifeCycleState::Issued].contains(&life_cycle_state)
    }

    fn ssc_feature_desc(&self) -> FeatureDescriptor {
        match self.ssc {
            Ssc::Enterprise => FeatureDescriptor::Enterprise(EnterpriseDescriptor {
                version: 1,
                length: PhantomData,
                base_com_id: BASE_COM_ID,
                num_com_ids: NUM_COM_IDS,
                no_range_crossing: false,
            }),
            Ssc::OpalV1 => FeatureDescriptor::OpalV1(OpalV1Descriptor {
                version: 1,
                length: PhantomData,
                base_com_id: BASE_COM_ID,
                num_com_ids: NUM_COM_IDS,
                no_range_crossing: false,
            }),
            Ssc::OpalV2 => FeatureDescriptor::OpalV2(OpalV2Descriptor {
                version: 1,
                length: PhantomData,
                base_com_id: BASE_COM_ID,
                num_com_ids: NUM_COM_IDS,
                no_range_crossing: false,
                num_locking_admins_supported: self.num_locking_admins_supported(),
                num_locking_users_supported: self.num_locking_users_supported(),
                initial_owner_pw: OwnerPasswordState::SameAsMSID,
                reverted_owner_pw: OwnerPasswordState::SameAsMSID,
            }),
            Ssc::Opalite => FeatureDescriptor::Opalite(OpaliteDescriptor {
                version: 1,
                length: PhantomData,
                base_com_id: BASE_COM_ID,
                num_com_ids: NUM_COM_IDS,
                initial_owner_pw: OwnerPasswordState::SameAsMSID,
                reverted_owner_pw: OwnerPasswordState::SameAsMSID,
            }),
            Ssc::PyriteV1 => FeatureDescriptor::PyriteV1(PyriteV1Descriptor {
                version: 1,
                length: PhantomData,
                base_com_id: BASE_COM_ID,
                num_com_ids: NUM_COM_IDS,
                initial_owner_pw: OwnerPasswordState::SameAsMSID,
                reverted_owner_pw: OwnerPasswordState::SameAsMSID,
            }),
            Ssc::PyriteV2 => FeatureDescriptor::PyriteV2(PyriteV2Descriptor {
                version: 1,
                length: PhantomData,
                base_com_id: BASE_COM_ID,
                num_com_ids: NUM_COM_IDS,
                initial_owner_pw: OwnerPasswordState::SameAsMSID,
                reverted_owner_pw: OwnerPasswordState::SameAsMSID,
            }),
            Ssc::Ruby => FeatureDescriptor::Ruby(RubyDescriptor {
                version: 1,
                length: PhantomData,
                base_com_id: BASE_COM_ID,
                num_com_ids: NUM_COM_IDS,
                no_range_crossing: false,
                num_locking_admins_supported: self.num_locking_admins_supported(),
                num_locking_users_supported: self.num_locking_users_supported(),
                initial_owner_pw: OwnerPasswordState::SameAsMSID,
                reverted_owner_pw: OwnerPasswordState::SameAsMSID,
            }),
            Ssc::KeyPerIO => FeatureDescriptor::KeyPerIO(KeyPerIODescriptor {
                version: 1,
                minor_version: 0,
                length: PhantomData,
                base_com_id_p1: BASE_COM_ID,
                num_com_ids_p1: NUM_COM_IDS,
                base_com_id_p3: BASE_COM_ID + NUM_COM_IDS,
                num_com_ids_p3: NUM_COM_IDS,
                initial_owner_pw: OwnerPasswordState::SameAsMSID,
                reverted_owner_pw: OwnerPasswordState::SameAsMSID,
                num_kpio_admins_supported: self.num_kpio_admins_supported(),
                replay_protection_enabled: false,
                replay_protection_supported: false,
                incorrect_key_detection_supported: false,
                tweak_key_required: false,
                kpio_scope: false,
                kpio_enabled: self.is_sp_enabled(kpio::admin::sp::KEY_PER_IO),
                max_key_uid_len: 16,
                kmip_key_injection_supported: false,
                nist_rsa_oaep_supported: false,
                nist_aes_gcm_supported: false,
                nist_aes_kw_supported: false,
                aes256_wrapping_supported: false,
                rsa2k_wrapping_supported: false,
                rsa3k_wrapping_supported: false,
                rsa4k_wrapping_supported: false,
                pki_kek_transport_supported: false,
                plaintext_kek_prov_supported: false,
                num_keks_supported: 1,
                total_key_tags_supported: 1,
                max_key_tags_per_namespace: 1,
                get_nonce_cmd_nonce_len: 1,
            }),
        }
    }

    fn num_locking_admins_supported(&self) -> u16 {
        use opal_2::locking::authority::ADMIN;

        match self.locking.as_ref() {
            Some(locking) => locking.authority.iter().filter(|(uid, _)| ADMIN.contains(**uid)).count() as u16,
            None => 0,
        }
    }

    fn num_locking_users_supported(&self) -> u16 {
        use opal_2::locking::authority::USER;

        match self.locking.as_ref() {
            Some(locking) => locking.authority.iter().filter(|(uid, _)| USER.contains(**uid)).count() as u16,
            None => 0,
        }
    }

    fn num_kpio_admins_supported(&self) -> u16 {
        use kpio::key_per_io::authority::ADMIN;

        match self.kpio.as_ref() {
            Some(kpio) => kpio.authority.iter().filter(|(uid, _)| ADMIN.contains(**uid)).count() as u16,
            None => 0,
        }
    }

    fn geometry_feature_desc() -> FeatureDescriptor {
        let desc = GeometryDescriptor {
            version: PhantomData,
            length: PhantomData,
            align: true,
            logical_block_size: 512,
            alignment_granularity: 16,
            lowest_aligned_lba: 4,
        };
        FeatureDescriptor::Geometry(desc)
    }

    fn block_sid_authentication_desc(&self) -> FeatureDescriptor {
        let admin_sp = &self.admin;
        let c_pin_table = &admin_sp.c_pin;
        let c_pin_sid = c_pin_table.get(&c_pin::SID).expect("C_PIN::SID missing from preconfig");
        let c_pin_msid = c_pin_table.get(&c_pin::MSID).expect("C_PIN::MSID missing from preconfig");
        FeatureDescriptor::BlockSIDAuth(BlockSIDAuthDescriptor {
            version: 2,
            length: PhantomData,
            locking_sp_frozen: false,
            locking_sp_freeze_supported: false,
            sid_authentication_blocked: false,
            sid_msid_pin_differ: c_pin_sid.pin != c_pin_msid.pin,
            hw_reset_unblocks: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use rstest::rstest;

    use super::*;

    /// Compares each table separately so that a failure prints only the mismatching tables.
    fn expect_admin_eq(actual: &Admin, expected: &Admin) {
        // The structs are destructured exhaustively so that newly added fields can't be forgotten.
        let Admin { uid, access_control, ace, authority, c_pin, sp, table } = actual;
        expect_that!(uid, eq(&expected.uid));
        expect_that!(access_control, eq(&expected.access_control));
        expect_that!(ace, eq(&expected.ace));
        expect_that!(authority, eq(&expected.authority));
        expect_that!(c_pin, eq(&expected.c_pin));
        expect_that!(sp, eq(&expected.sp));
        expect_that!(table, eq(&expected.table));
    }

    fn expect_locking_eq(actual: &Locking, expected: &Locking) {
        let Locking {
            uid,
            access_control,
            ace,
            authority,
            c_pin,
            k_aes_128,
            k_aes_256,
            locking,
            locking_info_ext,
            mbr_control,
            table,
            mbr,
            data_store,
        } = actual;
        expect_that!(uid, eq(&expected.uid));
        expect_that!(access_control, eq(&expected.access_control));
        expect_that!(ace, eq(&expected.ace));
        expect_that!(authority, eq(&expected.authority));
        expect_that!(c_pin, eq(&expected.c_pin));
        expect_that!(k_aes_128, eq(&expected.k_aes_128));
        expect_that!(k_aes_256, eq(&expected.k_aes_256));
        expect_that!(locking, eq(&expected.locking));
        expect_that!(locking_info_ext, eq(&expected.locking_info_ext));
        expect_that!(mbr_control, eq(&expected.mbr_control));
        expect_that!(table, eq(&expected.table));
        expect_that!(mbr, eq(&expected.mbr));
        expect_that!(data_store, eq(&expected.data_store));
    }

    fn expect_kpio_eq(actual: &KeyPerIo, expected: &KeyPerIo) {
        let KeyPerIo { uid, access_control, ace, authority, c_pin, table, data_store } = actual;
        expect_that!(uid, eq(&expected.uid));
        expect_that!(access_control, eq(&expected.access_control));
        expect_that!(ace, eq(&expected.ace));
        expect_that!(authority, eq(&expected.authority));
        expect_that!(c_pin, eq(&expected.c_pin));
        expect_that!(table, eq(&expected.table));
        expect_that!(data_store, eq(&expected.data_store));
    }

    fn expect_tper_eq(actual: &Tper, expected: &Tper) {
        let Tper { ssc: _, admin, locking, kpio } = actual;
        expect_admin_eq(admin, &expected.admin);
        match (locking, &expected.locking) {
            (Some(actual), Some(expected)) => expect_locking_eq(actual, expected),
            (actual, expected) => expect_that!(actual.is_some(), eq(expected.is_some())),
        }
        match (kpio, &expected.kpio) {
            (Some(actual), Some(expected)) => expect_kpio_eq(actual, expected),
            (actual, expected) => expect_that!(actual.is_some(), eq(expected.is_some())),
        }
    }

    /// The state without the byte tables, which are not serialized.
    fn serializable_state(ssc: Ssc) -> Tper {
        let mut tper = Tper::from_preconfig(ssc);
        if let Some(locking) = tper.locking.as_mut() {
            locking.mbr = None;
            locking.data_store = None;
        }
        if let Some(kpio) = tper.kpio.as_mut() {
            kpio.data_store = None;
        }
        tper
    }

    #[rstest]
    #[case::enterprise(Ssc::Enterprise)]
    #[case::opal_1(Ssc::OpalV1)]
    #[case::opal_2(Ssc::OpalV2)]
    #[case::opalite(Ssc::Opalite)]
    #[case::pyrite_1(Ssc::PyriteV1)]
    #[case::pyrite_2(Ssc::PyriteV2)]
    #[case::ruby(Ssc::Ruby)]
    #[case::kpio(Ssc::KeyPerIO)]
    #[gtest]
    fn serde_json_roundtrip(#[case] ssc: Ssc) {
        let tper = serializable_state(ssc);
        let json = serde_json::to_string(&tper).unwrap();
        let deserialized: Tper = serde_json::from_str(&json).unwrap();
        expect_tper_eq(&deserialized, &tper);
    }

    #[rstest]
    #[case::enterprise(Ssc::Enterprise)]
    #[case::opal_1(Ssc::OpalV1)]
    #[case::opal_2(Ssc::OpalV2)]
    #[case::opalite(Ssc::Opalite)]
    #[case::pyrite_1(Ssc::PyriteV1)]
    #[case::pyrite_2(Ssc::PyriteV2)]
    #[case::ruby(Ssc::Ruby)]
    #[case::kpio(Ssc::KeyPerIO)]
    #[gtest]
    fn serde_postcard_roundtrip(#[case] ssc: Ssc) {
        let tper = serializable_state(ssc);
        let bytes = postcard::to_allocvec(&tper).unwrap();
        let deserialized: Tper = postcard::from_bytes(&bytes).unwrap();
        expect_tper_eq(&deserialized, &tper);
    }

    #[rstest]
    #[case::enterprise(Ssc::Enterprise)]
    #[case::opal_1(Ssc::OpalV1)]
    #[case::opal_2(Ssc::OpalV2)]
    #[case::opalite(Ssc::Opalite)]
    #[case::pyrite_1(Ssc::PyriteV1)]
    #[case::pyrite_2(Ssc::PyriteV2)]
    #[case::ruby(Ssc::Ruby)]
    #[case::kpio(Ssc::KeyPerIO)]
    fn from_preconfig(#[case] ssc: Ssc) {
        let tper = Tper::from_preconfig(ssc);
        assert_eq!(tper.ssc, ssc)
    }
}
