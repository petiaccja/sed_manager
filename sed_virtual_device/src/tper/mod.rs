//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

mod admin;
mod locking;
mod opal_2;
mod preconfig_shared;
mod security_provider;

use std::marker::PhantomData;

pub use admin::Admin;
pub use locking::Locking;
pub use preconfig_shared::{INITIAL_SID_PASSWORD, PSID_PASSWORD};
pub use security_provider::{SecurityProvider, Table};
use sed_packet::discovery::{
    BlockSIDAuthDescriptor, Discovery, FeatureDescriptor, GeometryDescriptor, LockingDescriptor, OpalV2Descriptor,
    OwnerPasswordState, TperDescriptor,
};
use sed_spec::methods::{MethodStatus, Properties};
use sed_spec::objects::SecurityProviderRef;
use sed_spec::types::LifeCycleState;
use sed_spec::uid_registry::opal_2::admin::c_pin;
use sorbit::ser_de::ToBytes;

use crate::device::{BASE_COM_ID, NUM_COM_IDS};

use sed_spec::uid_registry::opal_2::admin::sp;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Tper {
    pub admin: Admin,
    pub locking: Option<Locking>,
}

impl Tper {
    pub fn sp(&self, uid: SecurityProviderRef) -> Option<&dyn SecurityProvider> {
        match uid {
            sp::ADMIN => Some(&self.admin),
            sp::LOCKING => self.locking.as_ref().map(|sp| sp as &dyn SecurityProvider),
            _ => None,
        }
    }

    pub fn sp_mut(&mut self, uid: SecurityProviderRef) -> Option<&mut dyn SecurityProvider> {
        match uid {
            sp::ADMIN => Some(&mut self.admin),
            sp::LOCKING => self.locking.as_mut().map(|sp| sp as &mut dyn SecurityProvider),
            _ => None,
        }
    }

    pub fn restore_preconfig(&mut self, sp: SecurityProviderRef) -> Result<Vec<SecurityProviderRef>, MethodStatus> {
        match sp {
            sp::ADMIN => {
                self.admin = opal_2::preconfig_admin::preconfig();
                self.locking = Some(opal_2::preconfig_locking::preconfig());
                Ok(vec![sp::ADMIN, sp::LOCKING])
            }
            sp::LOCKING => {
                if let Some(locking_sp) = self.admin.sp.get_mut(&sp::LOCKING) {
                    locking_sp.life_cycle_state = Some(LifeCycleState::ManufacturedInactive);
                }
                self.locking = Some(opal_2::preconfig_locking::preconfig());
                Ok(vec![sp::LOCKING])
            }
            _ => Err(MethodStatus::InvalidParameter),
        }
    }

    pub fn resize_byte_tables(&mut self) {
        if let Some(locking) = &mut self.locking {
            locking.resize_mbr();
            locking.resize_data_stores();
        }
    }
}

impl Tper {
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
            let admin_sp = &self.admin;
            let locking_sp_obj = admin_sp.sp.get(&locking_sp.uid).expect("locking SP missing from preconfig");
            let life_cycle_state = locking_sp_obj.life_cycle_state.unwrap_or(LifeCycleState::ManufacturedInactive);
            let locking_enabled =
                life_cycle_state == LifeCycleState::Manufactured || life_cycle_state == LifeCycleState::Issued;

            let locked = locking_sp
                .locking
                .values()
                .any(|range| range.read_locked.unwrap_or(false) || range.write_locked.unwrap_or(false));

            let (_, mbr_control) =
                locking_sp.mbr_control.first_key_value().expect("MBR control missing from preconfig");
            let mbr_enabled = mbr_control.enable.unwrap_or(false);
            let mbr_done = mbr_control.done.unwrap_or(false);

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
                mbr_shadowing_not_supported: false,
            })
        })
    }

    fn ssc_feature_desc(&self) -> FeatureDescriptor {
        FeatureDescriptor::OpalV2(OpalV2Descriptor {
            version: 1,
            length: PhantomData,
            base_com_id: BASE_COM_ID,
            num_com_ids: NUM_COM_IDS,
            no_range_crossing: false,
            num_locking_admins_supported: 4,
            num_locking_users_supported: 8,
            initial_owner_pw: OwnerPasswordState::SameAsMSID,
            reverted_owner_pw: OwnerPasswordState::SameAsMSID,
        })
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

impl Default for Tper {
    fn default() -> Self {
        Self { admin: opal_2::preconfig_admin::preconfig(), locking: Some(opal_2::preconfig_locking::preconfig()) }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use googletest::prelude::*;

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
        expect_that!(k_aes_256, eq(&expected.k_aes_256));
        expect_that!(locking, eq(&expected.locking));
        expect_that!(locking_info_ext, eq(&expected.locking_info_ext));
        expect_that!(mbr_control, eq(&expected.mbr_control));
        expect_that!(table, eq(&expected.table));
        expect_that!(mbr, eq(&expected.mbr));
        expect_that!(data_store, eq(&expected.data_store));
    }

    fn expect_tper_eq(actual: &Tper, expected: &Tper) {
        let Tper { admin, locking } = actual;
        expect_admin_eq(admin, &expected.admin);
        match (locking, &expected.locking) {
            (Some(actual), Some(expected)) => expect_locking_eq(actual, expected),
            (actual, expected) => expect_that!(actual.is_some(), eq(expected.is_some())),
        }
    }

    #[test]
    fn print_opal_2_preconfig() {
        let tper = Tper::default();
        let output = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("sed_spec")
            .join("data")
            .join("preconfig_opal_2.json");
        let json_str = serde_json::to_string_pretty(&tper).unwrap();
        std::fs::write(output, json_str).unwrap();
    }

    #[gtest]
    fn read_opal_2_preconfig() {
        let mut expected = Tper::default();
        expected.locking.as_mut().unwrap().mbr.clear();
        expected.locking.as_mut().unwrap().data_store = vec![vec![]];
        let output = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("sed_spec")
            .join("data")
            .join("preconfig_opal_2.json");
        let json_str = std::fs::read_to_string(output).unwrap();
        let loaded: Tper = serde_json::from_str(&json_str).unwrap();
        expect_tper_eq(&loaded, &expected);
    }
}
