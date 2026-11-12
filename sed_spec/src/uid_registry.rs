//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------
use crate::lookup::GlobalLookup;

#[allow(clippy::identity_op, reason = "codegen follow patterns")]
#[allow(clippy::module_inception, reason = "codegen follow patterns")]
#[allow(unused)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/uid_registry.rs"));

    /// The purpose of this is only so that it's easy to jump into the generated
    /// code using "go to definition".
    #[allow(unused)]
    const MARKER: () = GENERATED_MARKER;
}

pub use generated::*;

pub const GLOBAL_LOOKUP: GlobalLookup = GlobalLookup;

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use sed_packet::{TableRef, Uid};

    use super::*;
    use crate::lookup::{FeatureLookup, SecurityProviderLookup, TableLookup};
    use crate::path::Path;

    #[rstest]
    #[case(&self::core::LOOKUP)]
    #[case(&self::enterprise::LOOKUP)]
    #[case(&self::kpio::LOOKUP)]
    #[case(&self::opal_2::LOOKUP)]
    #[case(&self::opalite::LOOKUP)]
    #[case(&self::pyrite_2::LOOKUP)]
    #[case(&self::ruby::LOOKUP)]
    #[case(&self::data_store::LOOKUP)]
    #[case(&self::psid::LOOKUP)]
    fn sortedness_feature_lookups(#[case] lookup: &FeatureLookup) {
        assert!(lookup.sp_lookups.items.is_sorted_by_key(|(key, _)| key));
    }

    #[rstest]
    #[case(&self::core::shared::LOOKUP)]
    #[case(&self::opal_2::admin::LOOKUP)]
    #[case(&self::opal_2::locking::LOOKUP)]
    fn sortedness_security_provider_lookups(#[case] lookup: &SecurityProviderLookup) {
        assert!(lookup.table_lookups.items.is_sorted_by_key(|(key, _)| key));
    }

    #[test]
    fn sortedness_object_table_lookups() {
        assert!(self::core::shared::table::LOOKUP.by_name.items.is_sorted_by_key(|(key, _)| key));
        assert!(self::core::shared::table::LOOKUP.by_uid.items.is_sorted_by_key(|(key, _)| key));
    }

    #[test]
    fn sortedness_special_table_lookups() {
        assert!(self::core::shared::method_id::LOOKUP.by_name.items.is_sorted_by_key(|(key, _)| key));
        assert!(self::core::shared::method_id::LOOKUP.by_uid.items.is_sorted_by_key(|(key, _)| key));
        assert!(self::core::shared::table_id::LOOKUP.by_name.items.is_sorted_by_key(|(key, _)| key));
        assert!(self::core::shared::table_id::LOOKUP.by_uid.items.is_sorted_by_key(|(key, _)| key));
    }

    #[rstest]
    #[case("Anybody", Some(self::core::shared::authority::ANYBODY.to_uid()))]
    #[case("Authority::Anybody", Some(self::core::shared::authority::ANYBODY.to_uid()))]
    #[case("Another::Anybody", None)]
    fn lookup_table(#[case] path: impl AsRef<Path>, #[case] uid: Option<Uid>) {
        let path = path.as_ref();
        assert_eq!(self::core::shared::authority::LOOKUP.by_path(path), uid);
        if let Some(uid) = uid {
            assert_eq!(self::core::shared::authority::LOOKUP.by_uid(uid), Some(path.object().to_owned()));
        }
    }

    #[rstest]
    #[case("Authority::Anybody", Some(self::core::shared::authority::ANYBODY.to_uid()))]
    #[case("Shared::Authority::Anybody", Some(self::core::shared::authority::ANYBODY.to_uid()))]
    #[case("Another::Authority::Anybody", None)]
    fn lookup_security_provider(#[case] name: impl AsRef<Path>, #[case] uid: Option<Uid>) {
        let name = name.as_ref();
        let stripped = format!("{}::{}", name.table().unwrap(), name.object());
        assert_eq!(self::core::shared::LOOKUP.by_path(name), uid);
        if let Some(uid) = uid {
            assert_eq!(self::core::shared::LOOKUP.by_uid(uid), Some(stripped));
        }
    }

    #[rstest]
    #[case("General::Null".to_owned(), self::core::shared::general::NULL)]
    #[case("General::CharSetUnrestricted".to_owned(), self::core::shared::general::CHAR_SET_UNRESTRICTED)]
    #[case("InvokingID::ThisSP".to_owned(), self::core::shared::invoking_id::THIS_SP)]
    #[case("InvokingID::SessionManager".to_owned(), self::core::shared::invoking_id::SESSION_MANAGER)]
    #[case("SMMethodID::StartSession".to_owned(), self::core::shared::sm_method_id::START_SESSION)]
    fn lookup_security_provider_special(#[case] path: String, #[case] uid: Uid) {
        assert_eq!(self::core::shared::LOOKUP.by_path(&path), Some(uid));
        assert_eq!(self::core::shared::LOOKUP.by_uid(uid), Some(path));
    }

    #[rstest]
    #[case("Authority::Anybody", Some(self::core::shared::authority::ANYBODY.to_uid()))]
    #[case("Shared::Authority::Anybody", Some(self::core::shared::authority::ANYBODY.to_uid()))]
    #[case("Invalid::Authority::Anybody", None)]
    fn lookup_feature_shared(#[case] path: impl AsRef<Path>, #[case] uid: Option<Uid>) {
        let path = path.as_ref();
        let stripped = format!("{}::{}", path.table().unwrap(), path.object());
        assert_eq!(self::core::LOOKUP.by_path(path), uid);
        if let Some(uid) = uid {
            assert_eq!(self::core::LOOKUP.by_uid(uid, None), Some(stripped));
        }
    }

    #[rstest]
    #[case("Authority::Anybody", self::core::shared::authority::ANYBODY.to_uid())]
    #[case("Admin::Authority::PSID", self::psid::admin::authority::PSID.to_uid())]
    fn lookup_global(#[case] name: impl AsRef<Path>, #[case] uid: Uid) {
        let features = [];
        assert_eq!(GLOBAL_LOOKUP.by_path(&features, name), Some(uid));
    }

    #[rstest]
    #[case("Admin1".to_owned(), Some(self::opal_2::admin::authority::ADMIN.get(0).unwrap().to_uid()))]
    #[case("Admin256".to_owned(), Some(self::opal_2::admin::authority::ADMIN.get(255).unwrap().to_uid()))]
    #[case("Admin0".to_owned(), None)]
    #[case("Admin257".to_owned(), None)]
    #[case("AdminX".to_owned(), None)]
    #[case("Admin".to_owned(), None)]
    fn lookup_object_table_range(#[case] name: String, #[case] uid: Option<Uid>) {
        assert_eq!(self::opal_2::admin::authority::LOOKUP.by_name(&name), uid);
        if let Some(uid) = uid {
            assert_eq!(self::opal_2::admin::authority::LOOKUP.by_uid(uid), Some(name));
        }
    }

    #[rstest]
    #[case("DataStore2_Get_All".to_owned(), Some(self::data_store::locking::ace::DATA_STORE_GET_ALL.get(0).unwrap().to_uid()))]
    #[case("DataStore5_Set_All".to_owned(), Some(self::data_store::locking::ace::DATA_STORE_SET_ALL.get(3).unwrap().to_uid()))]
    #[case("DataStore128_Get_All".to_owned(), Some(self::data_store::locking::ace::DATA_STORE_GET_ALL.get(126).unwrap().to_uid()))]
    #[case("DataStore129_Get_All".to_owned(), None)]
    #[case("DataStore1_Get_All".to_owned(), None)]
    #[case("DataStore5_Set_All_Extra".to_owned(), None)]
    #[case("DataStore_Get_All".to_owned(), None)]
    fn lookup_object_table_multiple_ranges(#[case] name: String, #[case] uid: Option<Uid>) {
        assert_eq!(self::data_store::locking::ace::LOOKUP.by_name(&name), uid);
        if let Some(uid) = uid {
            assert_eq!(self::data_store::locking::ace::LOOKUP.by_uid(uid), Some(name));
        }
    }

    #[rstest]
    #[case("DataStore2".to_owned(), Some(self::data_store::locking::table_id::DATA_STORE.get(0).unwrap()))]
    #[case("DataStore128".to_owned(), Some(self::data_store::locking::table_id::DATA_STORE.get(126).unwrap()))]
    #[case("DataStore129".to_owned(), None)]
    #[case("DataStore1".to_owned(), None)]
    #[case("DataStore".to_owned(), None)]
    fn lookup_table_id_range(#[case] name: String, #[case] uid: Option<TableRef>) {
        assert_eq!(self::data_store::locking::table_id::LOOKUP.by_name(&name), uid);
        if let Some(uid) = uid {
            assert_eq!(self::data_store::locking::table_id::LOOKUP.by_uid(uid), Some(name));
        }
    }

    #[rstest]
    #[case::own_table(&self::opal_2::locking::LOOKUP, "DataStore", Some(self::opal_2::locking::table_id::DATA_STORE))]
    #[case::own_range(&self::data_store::locking::LOOKUP, "DataStore7", Some(self::data_store::locking::table_id::DATA_STORE.get(5).unwrap()))]
    #[case::core_table(&self::opal_2::locking::LOOKUP, "Locking", Some(self::core::shared::table_id::LOCKING))]
    #[case::other_sp(&self::opal_2::admin::LOOKUP, "DataStore", None)]
    #[case::differs_by_ssc(&self::enterprise::locking::LOOKUP, "DataStore", Some(self::enterprise::locking::table_id::DATA_STORE))]
    fn lookup_security_provider_table_id(
        #[case] lookup: &SecurityProviderLookup,
        #[case] name: impl AsRef<str>,
        #[case] uid: Option<TableRef>,
    ) {
        assert_eq!(lookup.table_id_by_name(name.as_ref()), uid);
    }

    #[rstest]
    #[case::ssc(&self::opal_2::LOOKUP, "Locking::Table::DataStore", Some(self::opal_2::locking::table::DATA_STORE.to_uid()))]
    #[case::feature(&self::data_store::LOOKUP, "Locking::Table::DataStore2", Some(self::data_store::locking::table::DATA_STORE.get(0).unwrap().to_uid()))]
    #[case::enterprise(&self::enterprise::LOOKUP, "Locking::Table::DataStore", Some(self::enterprise::locking::table::DATA_STORE.to_uid()))]
    #[case::not_in_core(&self::core::LOOKUP, "Table::DataStore", None)]
    fn lookup_data_store_table(
        #[case] lookup: &FeatureLookup,
        #[case] path: impl AsRef<Path>,
        #[case] uid: Option<Uid>,
    ) {
        assert_eq!(lookup.by_path(path), uid);
    }
}
