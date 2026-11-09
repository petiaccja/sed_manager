//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_packet::ObjectUid;
use sed_spec_macros::{DetokenizeStruct, FieldList, Object, TokenizeField, TokenizeStruct};

use crate::objects::KAes128Ref;
use crate::types::SymmetricModeMedia;
use crate::uid_registry::core::shared::table_id;

#[derive(Debug, Clone, Default, PartialEq, Eq, Object, TokenizeStruct, DetokenizeStruct, FieldList, TokenizeField)]
#[object(table=table_id::K_AES_128)]
pub struct KAes128 {
    pub uid: Option<KAes128Ref>,
    pub name: Option<String>,
    pub common_name: Option<String>,
    pub key: Option<[u8; 32]>,
    pub mode: Option<SymmetricModeMedia>,
}

impl ObjectUid for KAes128 {
    fn uid(&self) -> Option<Self::Ref> {
        self.uid
    }

    fn set_uid(&mut self, uid: Option<Self::Ref>) {
        self.uid = uid;
    }
}
