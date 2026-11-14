//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::any::Any;

use sed_spec::{
    objects::{AccessControl, Ace, Authority, CPin, SecurityProviderRef, TableDesc},
    uid_registry::kpio,
};

use crate::tper::security_provider::{SecurityProvider, Table};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct KeyPerIo {
    pub uid: SecurityProviderRef,
    pub access_control: Table<AccessControl>,
    pub ace: Table<Ace>,
    pub authority: Table<Authority>,
    pub c_pin: Table<CPin>,
    pub table: Table<TableDesc>,
    // The byte tables are not serialized to keep snapshots of the virtual
    // device small. They are restored by `Tper::resize_byte_tables`.
    #[serde(skip)]
    pub data_store: Option<Vec<u8>>,
}

impl KeyPerIo {
    /// Resizes the DataStore table so that its size matches the one indicated
    /// in the `Table` table.
    pub fn resize_data_store(&mut self) {
        if let Some(desc) = self.table.get(&kpio::key_per_io::table::DATA_STORE) {
            if let Some(data_store) = &mut self.data_store {
                data_store.resize(desc.rows.unwrap_or(10 * 1048576) as usize, 0);
            } else {
                self.data_store = Some(vec![0; 10 * 1048576])
            }
        }
    }
}

impl SecurityProvider for KeyPerIo {
    fn uid(&self) -> SecurityProviderRef {
        self.uid
    }

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
