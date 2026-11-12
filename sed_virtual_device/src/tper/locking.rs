//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::any::Any;

use sed_spec::{
    objects::{
        AccessControl, Ace, Authority, CPin, KAes256, LockingInfoExt, LockingRange, MbrControl, SecurityProviderRef,
        TableDesc,
    },
    uid_registry::{core::shared::table, enterprise, opal_2},
};

use crate::tper::security_provider::{SecurityProvider, Table};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Locking {
    pub uid: SecurityProviderRef,
    pub access_control: Table<AccessControl>,
    pub ace: Table<Ace>,
    pub authority: Table<Authority>,
    pub c_pin: Table<CPin>,
    pub k_aes_256: Table<KAes256>,
    pub locking: Table<LockingRange>,
    pub locking_info_ext: Table<LockingInfoExt>,
    pub mbr_control: Table<MbrControl>,
    pub table: Table<TableDesc>,
    #[serde(skip_serializing, default = "empty_byte_table")]
    pub mbr: Vec<u8>,
    #[serde(skip_serializing)]
    pub data_store: Option<Vec<u8>>,
}

impl Locking {
    /// Resizes the MBR table so that its size matches the one indicated
    /// in the `Table` table.
    pub fn resize_mbr(&mut self) {
        if let Some(desc) = self.table.get(&table::MBR) {
            self.mbr.resize(desc.rows.unwrap_or(128 * 1048576) as usize, 0);
        }
    }

    /// Resizes the DataStore table so that its size matches the one indicated
    /// in the `Table` table.
    pub fn resize_data_store(&mut self) {
        if let Some(desc) = self
            .table
            .get(&opal_2::locking::table::DATA_STORE)
            .or(self.table.get(&enterprise::locking::table::DATA_STORE))
        {
            if let Some(data_store) = &mut self.data_store {
                data_store.resize(desc.rows.unwrap_or(10 * 1048576) as usize, 0);
            } else {
                self.data_store = Some(vec![0; 10 * 1048576])
            }
        }
    }
}

impl SecurityProvider for Locking {
    fn access_control(&self) -> &Table<AccessControl> {
        &self.access_control
    }

    fn access_control_mut(&mut self) -> &mut Table<AccessControl> {
        &mut self.access_control
    }

    fn ace(&self) -> &Table<Ace> {
        &self.ace
    }

    fn ace_mut(&mut self) -> &mut Table<Ace> {
        &mut self.ace
    }

    fn authority(&self) -> &Table<Authority> {
        &self.authority
    }

    fn authority_mut(&mut self) -> &mut Table<Authority> {
        &mut self.authority
    }

    fn c_pin(&self) -> &Table<CPin> {
        &self.c_pin
    }

    fn c_pin_mut(&mut self) -> &mut Table<CPin> {
        &mut self.c_pin
    }

    fn table(&self) -> &Table<TableDesc> {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table<TableDesc> {
        &mut self.table
    }

    fn k_aes_256(&self) -> Option<&Table<KAes256>> {
        Some(&self.k_aes_256)
    }

    fn k_aes_256_mut(&mut self) -> Option<&mut Table<KAes256>> {
        Some(&mut self.k_aes_256)
    }

    fn locking(&self) -> Option<&Table<LockingRange>> {
        Some(&self.locking)
    }

    fn locking_mut(&mut self) -> Option<&mut Table<LockingRange>> {
        Some(&mut self.locking)
    }

    fn locking_info_ext(&self) -> Option<&Table<LockingInfoExt>> {
        Some(&self.locking_info_ext)
    }

    fn locking_info_ext_mut(&mut self) -> Option<&mut Table<LockingInfoExt>> {
        Some(&mut self.locking_info_ext)
    }

    fn mbr_control(&self) -> Option<&Table<MbrControl>> {
        Some(&self.mbr_control)
    }

    fn mbr_control_mut(&mut self) -> Option<&mut Table<MbrControl>> {
        Some(&mut self.mbr_control)
    }

    fn mbr(&self) -> Option<&Vec<u8>> {
        Some(&self.mbr)
    }

    fn mbr_mut(&mut self) -> Option<&mut Vec<u8>> {
        Some(&mut self.mbr)
    }

    fn data_store(&self) -> Option<&Vec<u8>> {
        self.data_store.as_ref()
    }

    fn data_store_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.data_store.as_mut()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn empty_byte_table() -> Vec<u8> {
    Vec::default()
}
