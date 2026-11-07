const GENERATED_MARKER: () = ();
pub mod opalite {
    use crate::lookup::SpRef;
    pub mod admin {
        use sed_packet::TableRef;
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const MSID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00008402_u64);
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] = [("MSID", MSID), ("SID", SID)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] = [(SID, "SID"), (MSID, "MSID")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000006_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 3usize] =
                [("Admins", ADMINS), ("Anybody", ANYBODY), ("SID", SID)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 3usize] =
                [(ANYBODY, "Anybody"), (ADMINS, "Admins"), (SID, "SID")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod sp {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SP;
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000001_u64);
            pub const LOCKING: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000002_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] =
                [("Admin", ADMIN), ("Locking", LOCKING)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] =
                [(ADMIN, "Admin"), (LOCKING, "Locking")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SP",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const C_PIN_MSID_GET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c04_u64);
            pub const T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800030003_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            pub const SP_SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800030002_u64);
            pub const C_PIN_SID_GET_NOPIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c02_u64);
            pub const C_PIN_SID_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c03_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 6usize] = [
                ("Anybody", ANYBODY),
                ("C_PIN_MSID_Get_PIN", C_PIN_MSID_GET_PIN),
                ("C_PIN_SID_Get_NOPIN", C_PIN_SID_GET_NOPIN),
                ("C_PIN_SID_Set_PIN", C_PIN_SID_SET_PIN),
                ("SP_SID", SP_SID),
                ("TPerInfo_Set_ProgrammaticResetEnable", T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 6usize] = [
                (ANYBODY, "Anybody"),
                (C_PIN_SID_GET_NOPIN, "C_PIN_SID_Get_NOPIN"),
                (C_PIN_SID_SET_PIN, "C_PIN_SID_Set_PIN"),
                (C_PIN_MSID_GET_PIN, "C_PIN_MSID_Get_PIN"),
                (SP_SID, "SP_SID"),
                (T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE, "TPerInfo_Set_ProgrammaticResetEnable"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 4usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::SP, &sp::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Admin",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    pub mod locking {
        use sed_packet::TableRef;
        pub mod k_aes_128 {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::K_AES_128;
            pub const GLOBAL_RANGE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080500000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] =
                [("GlobalRange_Key", GLOBAL_RANGE_KEY)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] =
                [(GLOBAL_RANGE_KEY, "GlobalRange_Key")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "K_AES_128",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod locking {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::LOCKING;
            pub const GLOBAL_RANGE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080200000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("GlobalRange", GLOBAL_RANGE)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(GLOBAL_RANGE, "GlobalRange")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Locking",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const ADMIN1: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900010001_u64);
            pub const USER2: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900030002_u64);
            pub const USER1: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900030001_u64);
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            pub const USERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900030000_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 6usize] = [
                ("Admin1", ADMIN1),
                ("Admins", ADMINS),
                ("Anybody", ANYBODY),
                ("User1", USER1),
                ("User2", USER2),
                ("Users", USERS),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 6usize] = [
                (ANYBODY, "Anybody"),
                (ADMINS, "Admins"),
                (ADMIN1, "Admin1"),
                (USERS, "Users"),
                (USER1, "User1"),
                (USER2, "User2"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const USER1: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00030001_u64);
            pub const ADMIN1: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00010001_u64);
            pub const USER2: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00030002_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 3usize] =
                [("Admin1", ADMIN1), ("User1", USER1), ("User2", USER2)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 3usize] =
                [(ADMIN1, "Admin1"), (USER1, "User1"), (USER2, "User2")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod k_aes_256 {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::K_AES_256;
            pub const GLOBAL_RANGE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080600000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] =
                [("GlobalRange_Key", GLOBAL_RANGE_KEY)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] =
                [(GLOBAL_RANGE_KEY, "GlobalRange_Key")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "K_AES_256",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const ACE_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800038000_u64);
            pub const ACE_SET_BOOLEAN_EXPRESSION: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800038001_u64);
            pub const K_AES_256_GLOBAL_RANGE_GEN_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003b800_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000002_u64);
            pub const MBR_CONTROL_ADMINS_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f800_u64);
            pub const K_AES_128_GLOBAL_RANGE_GEN_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003b000_u64);
            pub const DATA_STORE_SET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003fc01_u64);
            pub const C_PIN_USER1_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a801_u64);
            pub const AUTHORITY_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800039000_u64);
            pub const C_PIN_USER2_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a802_u64);
            pub const LOCKING_GLOBAL_RANGE_SET_RD_LOCKED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003e000_u64);
            pub const LOCKING_GLOBAL_RANGE_SET_WR_LOCKED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003e800_u64);
            pub const LOCKING_GLBL_RNG_ADMINS_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f000_u64);
            pub const DATA_STORE_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003fc00_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            pub const MBR_CONTROL_SET_DONE_TO_DOR: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f801_u64);
            pub const LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003d000_u64);
            pub const C_PIN_ADMINS_GET_ALL_NOPIN: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003a000_u64);
            pub const AUTHORITY_SET_ENABLED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800039001_u64);
            pub const K_AES_MODE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003bfff_u64);
            pub const C_PIN_ADMINS_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 21usize] = [
                ("ACE_Get_All", ACE_GET_ALL),
                ("ACE_Set_BooleanExpression", ACE_SET_BOOLEAN_EXPRESSION),
                ("Admin", ADMIN),
                ("Anybody", ANYBODY),
                ("Authority_Get_All", AUTHORITY_GET_ALL),
                ("Authority_Set_Enabled", AUTHORITY_SET_ENABLED),
                ("C_PIN_Admins_Get_All_NOPIN", C_PIN_ADMINS_GET_ALL_NOPIN),
                ("C_PIN_Admins_Set_PIN", C_PIN_ADMINS_SET_PIN),
                ("C_PIN_User1_Set_PIN", C_PIN_USER1_SET_PIN),
                ("C_PIN_User2_Set_PIN", C_PIN_USER2_SET_PIN),
                ("DataStore_Get_All", DATA_STORE_GET_ALL),
                ("DataStore_Set_All", DATA_STORE_SET_ALL),
                ("K_AES_128_GlobalRange_GenKey", K_AES_128_GLOBAL_RANGE_GEN_KEY),
                ("K_AES_256_GlobalRange_GenKey", K_AES_256_GLOBAL_RANGE_GEN_KEY),
                ("K_AES_Mode", K_AES_MODE),
                ("Locking_GlblRng_Admins_Set", LOCKING_GLBL_RNG_ADMINS_SET),
                ("Locking_GlobalRange_Get_RangeStartToActiveKey", LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY),
                ("Locking_GlobalRange_Set_RdLocked", LOCKING_GLOBAL_RANGE_SET_RD_LOCKED),
                ("Locking_GlobalRange_Set_WrLocked", LOCKING_GLOBAL_RANGE_SET_WR_LOCKED),
                ("MBRControl_Admins_Set", MBR_CONTROL_ADMINS_SET),
                ("MBRControl_Set_DoneToDOR", MBR_CONTROL_SET_DONE_TO_DOR),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 21usize] = [
                (ANYBODY, "Anybody"),
                (ADMIN, "Admin"),
                (ACE_GET_ALL, "ACE_Get_All"),
                (ACE_SET_BOOLEAN_EXPRESSION, "ACE_Set_BooleanExpression"),
                (AUTHORITY_GET_ALL, "Authority_Get_All"),
                (AUTHORITY_SET_ENABLED, "Authority_Set_Enabled"),
                (C_PIN_ADMINS_GET_ALL_NOPIN, "C_PIN_Admins_Get_All_NOPIN"),
                (C_PIN_ADMINS_SET_PIN, "C_PIN_Admins_Set_PIN"),
                (C_PIN_USER1_SET_PIN, "C_PIN_User1_Set_PIN"),
                (C_PIN_USER2_SET_PIN, "C_PIN_User2_Set_PIN"),
                (K_AES_128_GLOBAL_RANGE_GEN_KEY, "K_AES_128_GlobalRange_GenKey"),
                (K_AES_256_GLOBAL_RANGE_GEN_KEY, "K_AES_256_GlobalRange_GenKey"),
                (K_AES_MODE, "K_AES_Mode"),
                (LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY, "Locking_GlobalRange_Get_RangeStartToActiveKey"),
                (LOCKING_GLOBAL_RANGE_SET_RD_LOCKED, "Locking_GlobalRange_Set_RdLocked"),
                (LOCKING_GLOBAL_RANGE_SET_WR_LOCKED, "Locking_GlobalRange_Set_WrLocked"),
                (LOCKING_GLBL_RNG_ADMINS_SET, "Locking_GlblRng_Admins_Set"),
                (MBR_CONTROL_ADMINS_SET, "MBRControl_Admins_Set"),
                (MBR_CONTROL_SET_DONE_TO_DOR, "MBRControl_Set_DoneToDOR"),
                (DATA_STORE_GET_ALL, "DataStore_Get_All"),
                (DATA_STORE_SET_ALL, "DataStore_Set_All"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 6usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::LOCKING, &locking::LOOKUP),
            (super::super::core::shared::table_id::K_AES_128, &k_aes_128::LOOKUP),
            (super::super::core::shared::table_id::K_AES_256, &k_aes_256::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Locking",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    const SP_LOOKUPS: [(Option<SpRef>, &crate::lookup::SecurityProviderLookup); 2usize] = [
        (Some(admin::sp::ADMIN), &admin::LOOKUP),
        (Some(admin::sp::LOCKING), &locking::LOOKUP),
    ];
    pub const LOOKUP: crate::lookup::FeatureLookup<'static> =
        crate::lookup::FeatureLookup { sp_lookups: crate::lookup::SortedMap { items: &SP_LOOKUPS } };
}
pub mod enterprise {
    use crate::lookup::SpRef;
    pub mod admin {
        use sed_packet::TableRef;
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const SID_SET_SELF: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c03_u64);
            pub const MAKERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000003_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            pub const MSID_GET: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c04_u64);
            pub const SID_SET_MAKERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c05_u64);
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000201_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 6usize] = [
                ("Anybody", ANYBODY),
                ("MSID_Get", MSID_GET),
                ("Makers", MAKERS),
                ("SID", SID),
                ("SID_SetMakers", SID_SET_MAKERS),
                ("SID_SetSelf", SID_SET_SELF),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 6usize] = [
                (ANYBODY, "Anybody"),
                (MAKERS, "Makers"),
                (SID, "SID"),
                (SID_SET_SELF, "SID_SetSelf"),
                (MSID_GET, "MSID_Get"),
                (SID_SET_MAKERS, "SID_SetMakers"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const MSID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00008402_u64);
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] = [("MSID", MSID), ("SID", SID)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] = [(SID, "SID"), (MSID, "MSID")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod sp {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SP;
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000001_u64);
            pub const LOCKING: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000002_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] =
                [("Admin", ADMIN), ("Locking", LOCKING)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] =
                [(ADMIN, "Admin"), (LOCKING, "Locking")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SP",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const MAKERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000003_u64);
            pub const MAKER_SYM_K: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000004_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000006_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 4usize] = [
                ("Anybody", ANYBODY),
                ("MakerSymK", MAKER_SYM_K),
                ("Makers", MAKERS),
                ("SID", SID),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 4usize] = [
                (ANYBODY, "Anybody"),
                (MAKERS, "Makers"),
                (MAKER_SYM_K, "MakerSymK"),
                (SID, "SID"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 4usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::SP, &sp::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Admin",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    pub mod locking {
        use sed_packet::TableRef;
        pub mod locking {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::LOCKING;
            pub const BAND: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000080200000002_u64),
                end: ObjectRef::new(0x0000080200000002_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const GLOBAL_RANGE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080200000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("GlobalRange", GLOBAL_RANGE)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(GLOBAL_RANGE, "GlobalRange")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000080200000002_u64),
                        end: ObjectRef::new(0x0000080200000002_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Band",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Locking",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod k_aes_256 {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::K_AES_256;
            pub const BAND_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000080600000002_u64),
                end: ObjectRef::new(0x0000080600000002_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const GLOBAL_RANGE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080600000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] =
                [("GlobalRange_Key", GLOBAL_RANGE_KEY)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] =
                [(GLOBAL_RANGE_KEY, "GlobalRange_Key")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000080600000002_u64),
                        end: ObjectRef::new(0x0000080600000002_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Band",
                    suffix: "_Key",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "K_AES_256",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const BAND_MASTER: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00008001_u64),
                end: ObjectRef::new(0x0000000b00008001_u64 + 1024u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ERASE_MASTER: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00008401_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("EraseMaster", ERASE_MASTER)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(ERASE_MASTER, "EraseMaster")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00008001_u64),
                        end: ObjectRef::new(0x0000000b00008001_u64).add(1024u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "BandMaster",
                    suffix: "",
                    display_base: 0u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            pub const ANY_MASTER: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c05_u64);
            pub const BAND_MASTER_SET_SELF: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000800008401_u64),
                end: ObjectRef::new(0x0000000800008401_u64 + 1024u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const BAND_MASTER: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000800008001_u64),
                end: ObjectRef::new(0x0000000800008001_u64 + 1024u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ERASE_MASTER_SET_SELF: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800008c02_u64);
            pub const BAND_MASTERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c06_u64);
            pub const BAND_MASTER_SET_BAND: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000800008801_u64),
                end: ObjectRef::new(0x0000000800008801_u64 + 1024u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ERASE_MASTER: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c01_u64);
            pub const ANYBODY_GET_BAND: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800020001_u64);
            pub const GET_K_AES_MODE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003bfff_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 7usize] = [
                ("AnyMaster", ANY_MASTER),
                ("Anybody", ANYBODY),
                ("Anybody_GetBand", ANYBODY_GET_BAND),
                ("BandMasters", BAND_MASTERS),
                ("EraseMaster", ERASE_MASTER),
                ("EraseMaster_SetSelf", ERASE_MASTER_SET_SELF),
                ("Get_K_AES_Mode", GET_K_AES_MODE),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 7usize] = [
                (ANYBODY, "Anybody"),
                (ERASE_MASTER, "EraseMaster"),
                (ERASE_MASTER_SET_SELF, "EraseMaster_SetSelf"),
                (ANY_MASTER, "AnyMaster"),
                (BAND_MASTERS, "BandMasters"),
                (ANYBODY_GET_BAND, "Anybody_GetBand"),
                (GET_K_AES_MODE, "Get_K_AES_Mode"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 3usize] = [
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000800008001_u64),
                        end: ObjectRef::new(0x0000000800008001_u64).add(1024u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "BandMaster",
                    suffix: "",
                    display_base: 0u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000800008401_u64),
                        end: ObjectRef::new(0x0000000800008401_u64).add(1024u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "BandMaster",
                    suffix: "_SetSelf",
                    display_base: 0u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000800008801_u64),
                        end: ObjectRef::new(0x0000000800008801_u64).add(1024u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "BandMaster",
                    suffix: "_SetBand",
                    display_base: 0u64,
                },
            ];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const BAND_MASTER: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900008001_u64),
                end: ObjectRef::new(0x0000000900008001_u64 + 1024u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ERASE_MASTER: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900008401_u64);
            pub const BAND_MASTERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900008403_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 3usize] = [
                ("Anybody", ANYBODY),
                ("BandMasters", BAND_MASTERS),
                ("EraseMaster", ERASE_MASTER),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 3usize] = [
                (ANYBODY, "Anybody"),
                (ERASE_MASTER, "EraseMaster"),
                (BAND_MASTERS, "BandMasters"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900008001_u64),
                        end: ObjectRef::new(0x0000000900008001_u64).add(1024u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "BandMaster",
                    suffix: "",
                    display_base: 0u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod k_aes_128 {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::K_AES_128;
            pub const GLOBAL_RANGE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080500000001_u64);
            pub const BAND_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000080500000002_u64),
                end: ObjectRef::new(0x0000080500000002_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] =
                [("GlobalRange_Key", GLOBAL_RANGE_KEY)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] =
                [(GLOBAL_RANGE_KEY, "GlobalRange_Key")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000080500000002_u64),
                        end: ObjectRef::new(0x0000080500000002_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Band",
                    suffix: "_Key",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "K_AES_128",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 6usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::LOCKING, &locking::LOOKUP),
            (super::super::core::shared::table_id::K_AES_128, &k_aes_128::LOOKUP),
            (super::super::core::shared::table_id::K_AES_256, &k_aes_256::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Locking",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    const SP_LOOKUPS: [(Option<SpRef>, &crate::lookup::SecurityProviderLookup); 2usize] = [
        (Some(admin::sp::ADMIN), &admin::LOOKUP),
        (Some(admin::sp::LOCKING), &locking::LOOKUP),
    ];
    pub const LOOKUP: crate::lookup::FeatureLookup<'static> =
        crate::lookup::FeatureLookup { sp_lookups: crate::lookup::SortedMap { items: &SP_LOOKUPS } };
}
pub mod kpio {
    use crate::lookup::SpRef;
    pub mod key_per_io {
        use sed_packet::TableRef;
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const AUTHORITY_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800039000_u64);
            pub const AUTHORITY_SET_ENABLED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800039001_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000002_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            pub const C_PIN_ADMINS_GET_ALL_NOPIN: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003a000_u64);
            pub const ACE_SET_BOOLEAN_EXPRESSION: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800038001_u64);
            pub const KEY_ENCRYPTION_KEY_ADMIN_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800062001_u64);
            pub const C_PIN_ADMINS_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a001_u64);
            pub const KEY_ENCRYPTION_KEY_ADMIN_GET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800062002_u64);
            pub const ADMINS_SET_COMMON_NAME: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800000004_u64);
            pub const DATA_STORE_SET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003fc01_u64);
            pub const ACE_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800038000_u64);
            pub const ANYBODY_GET_COMMON_NAME: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800000003_u64);
            pub const KPIO_POLICIES_ADMIN_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800060001_u64);
            pub const KEY_TAG_ALLOCATION_ADMIN_GET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800061002_u64);
            pub const DATA_STORE_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003fc00_u64);
            pub const KPIO_POLICIES_ADMIN_GET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800060002_u64);
            pub const KEY_TAG_ALLOCATION_ADMIN_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800061001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 18usize] = [
                ("ACE_Get_All", ACE_GET_ALL),
                ("ACE_Set_BooleanExpression", ACE_SET_BOOLEAN_EXPRESSION),
                ("Admin", ADMIN),
                ("Admins_Set_CommonName", ADMINS_SET_COMMON_NAME),
                ("Anybody", ANYBODY),
                ("Anybody_Get_CommonName", ANYBODY_GET_COMMON_NAME),
                ("Authority_Get_All", AUTHORITY_GET_ALL),
                ("Authority_Set_Enabled", AUTHORITY_SET_ENABLED),
                ("C_PIN_Admins_Get_All_NOPIN", C_PIN_ADMINS_GET_ALL_NOPIN),
                ("C_PIN_Admins_Set_PIN", C_PIN_ADMINS_SET_PIN),
                ("DataStore_Get_All", DATA_STORE_GET_ALL),
                ("DataStore_Set_All", DATA_STORE_SET_ALL),
                ("KPIOPolicies_Admin_Get", KPIO_POLICIES_ADMIN_GET),
                ("KPIOPolicies_Admin_Set", KPIO_POLICIES_ADMIN_SET),
                ("KeyEncryptionKey_Admin_Get", KEY_ENCRYPTION_KEY_ADMIN_GET),
                ("KeyEncryptionKey_Admin_Set", KEY_ENCRYPTION_KEY_ADMIN_SET),
                ("KeyTagAllocation_Admin_Get", KEY_TAG_ALLOCATION_ADMIN_GET),
                ("KeyTagAllocation_Admin_Set", KEY_TAG_ALLOCATION_ADMIN_SET),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 18usize] = [
                (ANYBODY, "Anybody"),
                (ADMIN, "Admin"),
                (ANYBODY_GET_COMMON_NAME, "Anybody_Get_CommonName"),
                (ADMINS_SET_COMMON_NAME, "Admins_Set_CommonName"),
                (ACE_GET_ALL, "ACE_Get_All"),
                (ACE_SET_BOOLEAN_EXPRESSION, "ACE_Set_BooleanExpression"),
                (AUTHORITY_GET_ALL, "Authority_Get_All"),
                (AUTHORITY_SET_ENABLED, "Authority_Set_Enabled"),
                (C_PIN_ADMINS_GET_ALL_NOPIN, "C_PIN_Admins_Get_All_NOPIN"),
                (C_PIN_ADMINS_SET_PIN, "C_PIN_Admins_Set_PIN"),
                (DATA_STORE_GET_ALL, "DataStore_Get_All"),
                (DATA_STORE_SET_ALL, "DataStore_Set_All"),
                (KPIO_POLICIES_ADMIN_SET, "KPIOPolicies_Admin_Set"),
                (KPIO_POLICIES_ADMIN_GET, "KPIOPolicies_Admin_Get"),
                (KEY_TAG_ALLOCATION_ADMIN_SET, "KeyTagAllocation_Admin_Set"),
                (KEY_TAG_ALLOCATION_ADMIN_GET, "KeyTagAllocation_Admin_Get"),
                (KEY_ENCRYPTION_KEY_ADMIN_SET, "KeyEncryptionKey_Admin_Set"),
                (KEY_ENCRYPTION_KEY_ADMIN_GET, "KeyEncryptionKey_Admin_Get"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900010001_u64),
                end: ObjectRef::new(0x0000000900010001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] =
                [("Admins", ADMINS), ("Anybody", ANYBODY)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] =
                [(ANYBODY, "Anybody"), (ADMINS, "Admins")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900010001_u64),
                        end: ObjectRef::new(0x0000000900010001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00010001_u64),
                end: ObjectRef::new(0x0000000b00010001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 0usize] = [];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 0usize] = [];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00010001_u64),
                        end: ObjectRef::new(0x0000000b00010001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 3usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "KeyPerIO",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    pub mod admin {
        use sed_packet::TableRef;
        pub mod sp {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SP;
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000001_u64);
            pub const KEY_PER_IO: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000003_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] =
                [("Admin", ADMIN), ("KeyPerIO", KEY_PER_IO)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] =
                [(ADMIN, "Admin"), (KEY_PER_IO, "KeyPerIO")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SP",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            pub const MAKERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000003_u64);
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900000201_u64),
                end: ObjectRef::new(0x0000000900000201_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000006_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 4usize] = [
                ("Admins", ADMINS),
                ("Anybody", ANYBODY),
                ("Makers", MAKERS),
                ("SID", SID),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 4usize] = [
                (ANYBODY, "Anybody"),
                (ADMINS, "Admins"),
                (MAKERS, "Makers"),
                (SID, "SID"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900000201_u64),
                        end: ObjectRef::new(0x0000000900000201_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const SP_SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800030002_u64);
            pub const T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800030003_u64);
            pub const DATA_REMOVAL_MECHANISM_SET_ACTIVE_DATA_REMOVAL_MECHANISM: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800050001_u64);
            pub const C_PIN_SID_GET_NOPIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c02_u64);
            pub const C_PIN_ADMINS_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a001_u64);
            pub const C_PIN_SID_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c03_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            pub const C_PIN_MSID_GET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c04_u64);
            pub const SET_ENABLED: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800030001_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000002_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 10usize] = [
                ("Admin", ADMIN),
                ("Anybody", ANYBODY),
                ("C_PIN_Admins_Set_PIN", C_PIN_ADMINS_SET_PIN),
                ("C_PIN_MSID_Get_PIN", C_PIN_MSID_GET_PIN),
                ("C_PIN_SID_Get_NOPIN", C_PIN_SID_GET_NOPIN),
                ("C_PIN_SID_Set_PIN", C_PIN_SID_SET_PIN),
                (
                    "DataRemovalMechanism_Set_ActiveDataRemovalMechanism",
                    DATA_REMOVAL_MECHANISM_SET_ACTIVE_DATA_REMOVAL_MECHANISM,
                ),
                ("SP_SID", SP_SID),
                ("Set_Enabled", SET_ENABLED),
                ("TPerInfo_Set_ProgrammaticResetEnable", T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 10usize] = [
                (ANYBODY, "Anybody"),
                (ADMIN, "Admin"),
                (C_PIN_SID_GET_NOPIN, "C_PIN_SID_Get_NOPIN"),
                (C_PIN_SID_SET_PIN, "C_PIN_SID_Set_PIN"),
                (C_PIN_MSID_GET_PIN, "C_PIN_MSID_Get_PIN"),
                (SET_ENABLED, "Set_Enabled"),
                (SP_SID, "SP_SID"),
                (T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE, "TPerInfo_Set_ProgrammaticResetEnable"),
                (C_PIN_ADMINS_SET_PIN, "C_PIN_Admins_Set_PIN"),
                (
                    DATA_REMOVAL_MECHANISM_SET_ACTIVE_DATA_REMOVAL_MECHANISM,
                    "DataRemovalMechanism_Set_ActiveDataRemovalMechanism",
                ),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00000201_u64),
                end: ObjectRef::new(0x0000000b00000201_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00000001_u64);
            pub const MSID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00008402_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] = [("MSID", MSID), ("SID", SID)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] = [(SID, "SID"), (MSID, "MSID")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00000201_u64),
                        end: ObjectRef::new(0x0000000b00000201_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 4usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::SP, &sp::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Admin",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    const SP_LOOKUPS: [(Option<SpRef>, &crate::lookup::SecurityProviderLookup); 2usize] = [
        (Some(admin::sp::ADMIN), &admin::LOOKUP),
        (Some(admin::sp::KEY_PER_IO), &key_per_io::LOOKUP),
    ];
    pub const LOOKUP: crate::lookup::FeatureLookup<'static> =
        crate::lookup::FeatureLookup { sp_lookups: crate::lookup::SortedMap { items: &SP_LOOKUPS } };
}
pub mod pyrite_2 {
    use crate::lookup::SpRef;
    pub mod admin {
        use sed_packet::TableRef;
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00000001_u64);
            pub const MSID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00008402_u64);
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00000201_u64),
                end: ObjectRef::new(0x0000000b00000201_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] = [("MSID", MSID), ("SID", SID)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] = [(SID, "SID"), (MSID, "MSID")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00000201_u64),
                        end: ObjectRef::new(0x0000000b00000201_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900000201_u64),
                end: ObjectRef::new(0x0000000900000201_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000006_u64);
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 3usize] =
                [("Admins", ADMINS), ("Anybody", ANYBODY), ("SID", SID)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 3usize] =
                [(ANYBODY, "Anybody"), (ADMINS, "Admins"), (SID, "SID")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900000201_u64),
                        end: ObjectRef::new(0x0000000900000201_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const C_PIN_MSID_GET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c04_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000002_u64);
            pub const T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800030003_u64);
            pub const SP_SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800030002_u64);
            pub const SET_ENABLED: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800030001_u64);
            pub const C_PIN_ADMINS_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a001_u64);
            pub const C_PIN_SID_GET_NOPIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c02_u64);
            pub const DATA_REMOVAL_MECHANISM_SET_ACTIVE_DATA_REMOVAL_MECHANISM: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800050001_u64);
            pub const C_PIN_SID_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c03_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 10usize] = [
                ("Admin", ADMIN),
                ("Anybody", ANYBODY),
                ("C_PIN_Admins_Set_PIN", C_PIN_ADMINS_SET_PIN),
                ("C_PIN_MSID_Get_PIN", C_PIN_MSID_GET_PIN),
                ("C_PIN_SID_Get_NOPIN", C_PIN_SID_GET_NOPIN),
                ("C_PIN_SID_Set_PIN", C_PIN_SID_SET_PIN),
                (
                    "DataRemovalMechanism_Set_ActiveDataRemovalMechanism",
                    DATA_REMOVAL_MECHANISM_SET_ACTIVE_DATA_REMOVAL_MECHANISM,
                ),
                ("SP_SID", SP_SID),
                ("Set_Enabled", SET_ENABLED),
                ("TPerInfo_Set_ProgrammaticResetEnable", T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 10usize] = [
                (ANYBODY, "Anybody"),
                (ADMIN, "Admin"),
                (C_PIN_SID_GET_NOPIN, "C_PIN_SID_Get_NOPIN"),
                (C_PIN_SID_SET_PIN, "C_PIN_SID_Set_PIN"),
                (C_PIN_MSID_GET_PIN, "C_PIN_MSID_Get_PIN"),
                (SET_ENABLED, "Set_Enabled"),
                (SP_SID, "SP_SID"),
                (T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE, "TPerInfo_Set_ProgrammaticResetEnable"),
                (C_PIN_ADMINS_SET_PIN, "C_PIN_Admins_Set_PIN"),
                (
                    DATA_REMOVAL_MECHANISM_SET_ACTIVE_DATA_REMOVAL_MECHANISM,
                    "DataRemovalMechanism_Set_ActiveDataRemovalMechanism",
                ),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod sp {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SP;
            pub const LOCKING: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000002_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] =
                [("Admin", ADMIN), ("Locking", LOCKING)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] =
                [(ADMIN, "Admin"), (LOCKING, "Locking")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SP",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 4usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::SP, &sp::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Admin",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    pub mod locking {
        use sed_packet::TableRef;
        pub mod locking {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::LOCKING;
            pub const GLOBAL_RANGE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080200000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("GlobalRange", GLOBAL_RANGE)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(GLOBAL_RANGE, "GlobalRange")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Locking",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const AUTHORITY_SET_ENABLED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800039001_u64);
            pub const AUTHORITY_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800039000_u64);
            pub const DATA_STORE_SET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003fc01_u64);
            pub const C_PIN_USER1_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a801_u64);
            pub const C_PIN_USER2_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a802_u64);
            pub const C_PIN_ADMINS_GET_ALL_NOPIN: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003a000_u64);
            pub const LOCKING_GLOBAL_RANGE_SET_WR_LOCKED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003e800_u64);
            pub const MBR_CONTROL_ADMINS_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f800_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            pub const C_PIN_ADMINS_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a001_u64);
            pub const LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003d000_u64);
            pub const LOCKING_GLBL_RNG_ADMINS_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f000_u64);
            pub const ACE_SET_BOOLEAN_EXPRESSION: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800038001_u64);
            pub const MBR_CONTROL_SET_DONE_TO_DOR: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f801_u64);
            pub const DATA_STORE_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003fc00_u64);
            pub const LOCKING_GLOBAL_RANGE_SET_RD_LOCKED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003e000_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000002_u64);
            pub const ACE_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800038000_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 18usize] = [
                ("ACE_Get_All", ACE_GET_ALL),
                ("ACE_Set_BooleanExpression", ACE_SET_BOOLEAN_EXPRESSION),
                ("Admin", ADMIN),
                ("Anybody", ANYBODY),
                ("Authority_Get_All", AUTHORITY_GET_ALL),
                ("Authority_Set_Enabled", AUTHORITY_SET_ENABLED),
                ("C_PIN_Admins_Get_All_NOPIN", C_PIN_ADMINS_GET_ALL_NOPIN),
                ("C_PIN_Admins_Set_PIN", C_PIN_ADMINS_SET_PIN),
                ("C_PIN_User1_Set_PIN", C_PIN_USER1_SET_PIN),
                ("C_PIN_User2_Set_PIN", C_PIN_USER2_SET_PIN),
                ("DataStore_Get_All", DATA_STORE_GET_ALL),
                ("DataStore_Set_All", DATA_STORE_SET_ALL),
                ("Locking_GlblRng_Admins_Set", LOCKING_GLBL_RNG_ADMINS_SET),
                ("Locking_GlobalRange_Get_RangeStartToActiveKey", LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY),
                ("Locking_GlobalRange_Set_RdLocked", LOCKING_GLOBAL_RANGE_SET_RD_LOCKED),
                ("Locking_GlobalRange_Set_WrLocked", LOCKING_GLOBAL_RANGE_SET_WR_LOCKED),
                ("MBRControl_Admins_Set", MBR_CONTROL_ADMINS_SET),
                ("MBRControl_Set_DoneToDOR", MBR_CONTROL_SET_DONE_TO_DOR),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 18usize] = [
                (ANYBODY, "Anybody"),
                (ADMIN, "Admin"),
                (ACE_GET_ALL, "ACE_Get_All"),
                (ACE_SET_BOOLEAN_EXPRESSION, "ACE_Set_BooleanExpression"),
                (AUTHORITY_GET_ALL, "Authority_Get_All"),
                (AUTHORITY_SET_ENABLED, "Authority_Set_Enabled"),
                (C_PIN_ADMINS_GET_ALL_NOPIN, "C_PIN_Admins_Get_All_NOPIN"),
                (C_PIN_ADMINS_SET_PIN, "C_PIN_Admins_Set_PIN"),
                (C_PIN_USER1_SET_PIN, "C_PIN_User1_Set_PIN"),
                (C_PIN_USER2_SET_PIN, "C_PIN_User2_Set_PIN"),
                (LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY, "Locking_GlobalRange_Get_RangeStartToActiveKey"),
                (LOCKING_GLOBAL_RANGE_SET_RD_LOCKED, "Locking_GlobalRange_Set_RdLocked"),
                (LOCKING_GLOBAL_RANGE_SET_WR_LOCKED, "Locking_GlobalRange_Set_WrLocked"),
                (LOCKING_GLBL_RNG_ADMINS_SET, "Locking_GlblRng_Admins_Set"),
                (MBR_CONTROL_ADMINS_SET, "MBRControl_Admins_Set"),
                (MBR_CONTROL_SET_DONE_TO_DOR, "MBRControl_Set_DoneToDOR"),
                (DATA_STORE_GET_ALL, "DataStore_Get_All"),
                (DATA_STORE_SET_ALL, "DataStore_Set_All"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const USER2: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900030002_u64);
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900010001_u64),
                end: ObjectRef::new(0x0000000900010001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            pub const USER1: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900030001_u64);
            pub const USERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900030000_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 5usize] = [
                ("Admins", ADMINS),
                ("Anybody", ANYBODY),
                ("User1", USER1),
                ("User2", USER2),
                ("Users", USERS),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 5usize] = [
                (ANYBODY, "Anybody"),
                (ADMINS, "Admins"),
                (USERS, "Users"),
                (USER1, "User1"),
                (USER2, "User2"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900010001_u64),
                        end: ObjectRef::new(0x0000000900010001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00010001_u64),
                end: ObjectRef::new(0x0000000b00010001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const USER2: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00030002_u64);
            pub const USER1: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00030001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] = [("User1", USER1), ("User2", USER2)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] = [(USER1, "User1"), (USER2, "User2")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00010001_u64),
                        end: ObjectRef::new(0x0000000b00010001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 4usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::LOCKING, &locking::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Locking",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    const SP_LOOKUPS: [(Option<SpRef>, &crate::lookup::SecurityProviderLookup); 2usize] = [
        (Some(admin::sp::ADMIN), &admin::LOOKUP),
        (Some(admin::sp::LOCKING), &locking::LOOKUP),
    ];
    pub const LOOKUP: crate::lookup::FeatureLookup<'static> =
        crate::lookup::FeatureLookup { sp_lookups: crate::lookup::SortedMap { items: &SP_LOOKUPS } };
}
pub mod ruby {
    use crate::lookup::SpRef;
    pub mod admin {
        use sed_packet::TableRef;
        pub mod sp {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SP;
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000001_u64);
            pub const LOCKING: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000002_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] =
                [("Admin", ADMIN), ("Locking", LOCKING)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] =
                [(ADMIN, "Admin"), (LOCKING, "Locking")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SP",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00000201_u64),
                end: ObjectRef::new(0x0000000b00000201_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00000001_u64);
            pub const MSID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00008402_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] = [("MSID", MSID), ("SID", SID)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] = [(SID, "SID"), (MSID, "MSID")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00000201_u64),
                        end: ObjectRef::new(0x0000000b00000201_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const C_PIN_ADMINS_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a001_u64);
            pub const C_PIN_SID_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c03_u64);
            pub const C_PIN_MSID_GET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c04_u64);
            pub const SET_ENABLED: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800030001_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000002_u64);
            pub const T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800030003_u64);
            pub const SP_SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800030002_u64);
            pub const C_PIN_SID_GET_NOPIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c02_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 9usize] = [
                ("Admin", ADMIN),
                ("Anybody", ANYBODY),
                ("C_PIN_Admins_Set_PIN", C_PIN_ADMINS_SET_PIN),
                ("C_PIN_MSID_Get_PIN", C_PIN_MSID_GET_PIN),
                ("C_PIN_SID_Get_NOPIN", C_PIN_SID_GET_NOPIN),
                ("C_PIN_SID_Set_PIN", C_PIN_SID_SET_PIN),
                ("SP_SID", SP_SID),
                ("Set_Enabled", SET_ENABLED),
                ("TPerInfo_Set_ProgrammaticResetEnable", T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 9usize] = [
                (ANYBODY, "Anybody"),
                (ADMIN, "Admin"),
                (C_PIN_SID_GET_NOPIN, "C_PIN_SID_Get_NOPIN"),
                (C_PIN_SID_SET_PIN, "C_PIN_SID_Set_PIN"),
                (C_PIN_MSID_GET_PIN, "C_PIN_MSID_Get_PIN"),
                (SET_ENABLED, "Set_Enabled"),
                (SP_SID, "SP_SID"),
                (T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE, "TPerInfo_Set_ProgrammaticResetEnable"),
                (C_PIN_ADMINS_SET_PIN, "C_PIN_Admins_Set_PIN"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000006_u64);
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900000201_u64),
                end: ObjectRef::new(0x0000000900000201_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            pub const MAKERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000003_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 4usize] = [
                ("Admins", ADMINS),
                ("Anybody", ANYBODY),
                ("Makers", MAKERS),
                ("SID", SID),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 4usize] = [
                (ANYBODY, "Anybody"),
                (ADMINS, "Admins"),
                (MAKERS, "Makers"),
                (SID, "SID"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900000201_u64),
                        end: ObjectRef::new(0x0000000900000201_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 4usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::SP, &sp::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Admin",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    pub mod locking {
        use sed_packet::TableRef;
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const USER: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00030001_u64),
                end: ObjectRef::new(0x0000000b00030001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00010001_u64),
                end: ObjectRef::new(0x0000000b00010001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 0usize] = [];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 0usize] = [];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 2usize] = [
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00010001_u64),
                        end: ObjectRef::new(0x0000000b00010001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00030001_u64),
                        end: ObjectRef::new(0x0000000b00030001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "User",
                    suffix: "",
                    display_base: 1u64,
                },
            ];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod k_aes_128 {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::K_AES_128;
            pub const GLOBAL_RANGE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080500000001_u64);
            pub const RANGE_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000080500030001_u64),
                end: ObjectRef::new(0x0000080500030001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] =
                [("GlobalRange_Key", GLOBAL_RANGE_KEY)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] =
                [(GLOBAL_RANGE_KEY, "GlobalRange_Key")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000080500030001_u64),
                        end: ObjectRef::new(0x0000080500030001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Range",
                    suffix: "_Key",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "K_AES_128",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod locking {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::LOCKING;
            pub const GLOBAL_RANGE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080200000001_u64);
            pub const RANGE: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000080200030001_u64),
                end: ObjectRef::new(0x0000080200030001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("GlobalRange", GLOBAL_RANGE)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(GLOBAL_RANGE, "GlobalRange")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000080200030001_u64),
                        end: ObjectRef::new(0x0000080200030001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Range",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Locking",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const ADMINS_SET_COMMON_NAME: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800000004_u64);
            pub const ACE_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800038000_u64);
            pub const K_AES_256_RANGE_GEN_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003b801_u64),
                end: ObjectRef::new(0x000000080003b801_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const LOCKING_ADMINS_RANGE_START_TO_LOR: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f001_u64);
            pub const LOCKING_GLBL_RNG_ADMINS_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f000_u64);
            pub const AUTHORITY_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800039000_u64);
            pub const MBR_CONTROL_ADMINS_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f800_u64);
            pub const DATA_STORE_SET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003fc01_u64);
            pub const LOCKING_RANGE_SET_RD_LOCKED: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003e001_u64),
                end: ObjectRef::new(0x000000080003e001_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const K_AES_256_GLOBAL_RANGE_GEN_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003b800_u64);
            pub const C_PIN_USER_SET_PIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003a801_u64),
                end: ObjectRef::new(0x000000080003a801_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const AUTHORITY_SET_ENABLED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800039001_u64);
            pub const ACE_SET_BOOLEAN_EXPRESSION: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800038001_u64);
            pub const LOCKING_RANGE_GET_RANGE_START_TO_ACTIVE_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> =
                UidRange {
                    start: ObjectRef::new(0x000000080003d001_u64),
                    end: ObjectRef::new(0x000000080003d001_u64 + 256u64 * 1u64),
                    step: 1u64 as u32,
                };
            pub const C_PIN_ADMINS_GET_ALL_NOPIN: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003a000_u64);
            pub const LOCKING_GLOBAL_RANGE_SET_RD_LOCKED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003e000_u64);
            pub const K_AES_128_RANGE_GEN_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003b001_u64),
                end: ObjectRef::new(0x000000080003b001_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const K_AES_MODE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003bfff_u64);
            pub const LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003d000_u64);
            pub const USER_SET_COMMON_NAME: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000800044001_u64),
                end: ObjectRef::new(0x0000000800044001_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            pub const MBR_CONTROL_SET_DONE_TO_DOR: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f801_u64);
            pub const K_AES_128_GLOBAL_RANGE_GEN_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003b000_u64);
            pub const LOCKING_GLOBAL_RANGE_SET_WR_LOCKED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003e800_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000002_u64);
            pub const C_PIN_ADMINS_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a001_u64);
            pub const DATA_STORE_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003fc00_u64);
            pub const LOCKING_RANGE_SET_WR_LOCKED: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003e801_u64),
                end: ObjectRef::new(0x000000080003e801_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ANYBODY_GET_COMMON_NAME: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800000003_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 22usize] = [
                ("ACE_Get_All", ACE_GET_ALL),
                ("ACE_Set_BooleanExpression", ACE_SET_BOOLEAN_EXPRESSION),
                ("Admin", ADMIN),
                ("Admins_Set_CommonName", ADMINS_SET_COMMON_NAME),
                ("Anybody", ANYBODY),
                ("Anybody_Get_CommonName", ANYBODY_GET_COMMON_NAME),
                ("Authority_Get_All", AUTHORITY_GET_ALL),
                ("Authority_Set_Enabled", AUTHORITY_SET_ENABLED),
                ("C_PIN_Admins_Get_All_NOPIN", C_PIN_ADMINS_GET_ALL_NOPIN),
                ("C_PIN_Admins_Set_PIN", C_PIN_ADMINS_SET_PIN),
                ("DataStore_Get_All", DATA_STORE_GET_ALL),
                ("DataStore_Set_All", DATA_STORE_SET_ALL),
                ("K_AES_128_GlobalRange_GenKey", K_AES_128_GLOBAL_RANGE_GEN_KEY),
                ("K_AES_256_GlobalRange_GenKey", K_AES_256_GLOBAL_RANGE_GEN_KEY),
                ("K_AES_Mode", K_AES_MODE),
                ("Locking_Admins_RangeStartToLOR", LOCKING_ADMINS_RANGE_START_TO_LOR),
                ("Locking_GlblRng_Admins_Set", LOCKING_GLBL_RNG_ADMINS_SET),
                ("Locking_GlobalRange_Get_RangeStartToActiveKey", LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY),
                ("Locking_GlobalRange_Set_RdLocked", LOCKING_GLOBAL_RANGE_SET_RD_LOCKED),
                ("Locking_GlobalRange_Set_WrLocked", LOCKING_GLOBAL_RANGE_SET_WR_LOCKED),
                ("MBRControl_Admins_Set", MBR_CONTROL_ADMINS_SET),
                ("MBRControl_Set_DoneToDOR", MBR_CONTROL_SET_DONE_TO_DOR),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 22usize] = [
                (ANYBODY, "Anybody"),
                (ADMIN, "Admin"),
                (ANYBODY_GET_COMMON_NAME, "Anybody_Get_CommonName"),
                (ADMINS_SET_COMMON_NAME, "Admins_Set_CommonName"),
                (ACE_GET_ALL, "ACE_Get_All"),
                (ACE_SET_BOOLEAN_EXPRESSION, "ACE_Set_BooleanExpression"),
                (AUTHORITY_GET_ALL, "Authority_Get_All"),
                (AUTHORITY_SET_ENABLED, "Authority_Set_Enabled"),
                (C_PIN_ADMINS_GET_ALL_NOPIN, "C_PIN_Admins_Get_All_NOPIN"),
                (C_PIN_ADMINS_SET_PIN, "C_PIN_Admins_Set_PIN"),
                (K_AES_128_GLOBAL_RANGE_GEN_KEY, "K_AES_128_GlobalRange_GenKey"),
                (K_AES_256_GLOBAL_RANGE_GEN_KEY, "K_AES_256_GlobalRange_GenKey"),
                (K_AES_MODE, "K_AES_Mode"),
                (LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY, "Locking_GlobalRange_Get_RangeStartToActiveKey"),
                (LOCKING_GLOBAL_RANGE_SET_RD_LOCKED, "Locking_GlobalRange_Set_RdLocked"),
                (LOCKING_GLOBAL_RANGE_SET_WR_LOCKED, "Locking_GlobalRange_Set_WrLocked"),
                (LOCKING_GLBL_RNG_ADMINS_SET, "Locking_GlblRng_Admins_Set"),
                (LOCKING_ADMINS_RANGE_START_TO_LOR, "Locking_Admins_RangeStartToLOR"),
                (MBR_CONTROL_ADMINS_SET, "MBRControl_Admins_Set"),
                (MBR_CONTROL_SET_DONE_TO_DOR, "MBRControl_Set_DoneToDOR"),
                (DATA_STORE_GET_ALL, "DataStore_Get_All"),
                (DATA_STORE_SET_ALL, "DataStore_Set_All"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 7usize] = [
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003a801_u64),
                        end: ObjectRef::new(0x000000080003a801_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "C_PIN_User",
                    suffix: "_Set_PIN",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003b001_u64),
                        end: ObjectRef::new(0x000000080003b001_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "K_AES_128_Range",
                    suffix: "_GenKey",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003b801_u64),
                        end: ObjectRef::new(0x000000080003b801_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "K_AES_256_Range",
                    suffix: "_GenKey",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003d001_u64),
                        end: ObjectRef::new(0x000000080003d001_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Locking_Range",
                    suffix: "_Get_RangeStartToActiveKey",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003e001_u64),
                        end: ObjectRef::new(0x000000080003e001_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Locking_Range",
                    suffix: "_Set_RdLocked",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003e801_u64),
                        end: ObjectRef::new(0x000000080003e801_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Locking_Range",
                    suffix: "_Set_WrLocked",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000800044001_u64),
                        end: ObjectRef::new(0x0000000800044001_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "User",
                    suffix: "_Set_CommonName",
                    display_base: 1u64,
                },
            ];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            pub const USER: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900030001_u64),
                end: ObjectRef::new(0x0000000900030001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900010001_u64),
                end: ObjectRef::new(0x0000000900010001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const USERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900030000_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 3usize] =
                [("Admins", ADMINS), ("Anybody", ANYBODY), ("Users", USERS)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 3usize] =
                [(ANYBODY, "Anybody"), (ADMINS, "Admins"), (USERS, "Users")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 2usize] = [
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900010001_u64),
                        end: ObjectRef::new(0x0000000900010001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900030001_u64),
                        end: ObjectRef::new(0x0000000900030001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "User",
                    suffix: "",
                    display_base: 1u64,
                },
            ];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod k_aes_256 {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::K_AES_256;
            pub const GLOBAL_RANGE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080600000001_u64);
            pub const RANGE_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000080600030001_u64),
                end: ObjectRef::new(0x0000080600030001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] =
                [("GlobalRange_Key", GLOBAL_RANGE_KEY)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] =
                [(GLOBAL_RANGE_KEY, "GlobalRange_Key")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000080600030001_u64),
                        end: ObjectRef::new(0x0000080600030001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Range",
                    suffix: "_Key",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "K_AES_256",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 6usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::LOCKING, &locking::LOOKUP),
            (super::super::core::shared::table_id::K_AES_128, &k_aes_128::LOOKUP),
            (super::super::core::shared::table_id::K_AES_256, &k_aes_256::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Locking",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    const SP_LOOKUPS: [(Option<SpRef>, &crate::lookup::SecurityProviderLookup); 2usize] = [
        (Some(admin::sp::ADMIN), &admin::LOOKUP),
        (Some(admin::sp::LOCKING), &locking::LOOKUP),
    ];
    pub const LOOKUP: crate::lookup::FeatureLookup<'static> =
        crate::lookup::FeatureLookup { sp_lookups: crate::lookup::SortedMap { items: &SP_LOOKUPS } };
}
pub mod opal_2 {
    use crate::lookup::SpRef;
    pub mod admin {
        use sed_packet::TableRef;
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900000201_u64),
                end: ObjectRef::new(0x0000000900000201_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const MAKERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000003_u64);
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000006_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 4usize] = [
                ("Admins", ADMINS),
                ("Anybody", ANYBODY),
                ("Makers", MAKERS),
                ("SID", SID),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 4usize] = [
                (ANYBODY, "Anybody"),
                (ADMINS, "Admins"),
                (MAKERS, "Makers"),
                (SID, "SID"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900000201_u64),
                        end: ObjectRef::new(0x0000000900000201_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const MSID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00008402_u64);
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b00000001_u64);
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00000201_u64),
                end: ObjectRef::new(0x0000000b00000201_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] = [("MSID", MSID), ("SID", SID)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] = [(SID, "SID"), (MSID, "MSID")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00000201_u64),
                        end: ObjectRef::new(0x0000000b00000201_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod sp {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SP;
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000001_u64);
            pub const LOCKING: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000002_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] =
                [("Admin", ADMIN), ("Locking", LOCKING)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] =
                [(ADMIN, "Admin"), (LOCKING, "Locking")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SP",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const C_PIN_ADMINS_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a001_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            pub const T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800030003_u64);
            pub const SET_ENABLED: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800030001_u64);
            pub const DATA_REMOVAL_MECHANISM_SET_ACTIVE_DATA_REMOVAL_MECHANISM: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800050001_u64);
            pub const C_PIN_MSID_GET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c04_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000002_u64);
            pub const C_PIN_SID_GET_NOPIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c02_u64);
            pub const C_PIN_SID_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800008c03_u64);
            pub const SP_SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800030002_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 10usize] = [
                ("Admin", ADMIN),
                ("Anybody", ANYBODY),
                ("C_PIN_Admins_Set_PIN", C_PIN_ADMINS_SET_PIN),
                ("C_PIN_MSID_Get_PIN", C_PIN_MSID_GET_PIN),
                ("C_PIN_SID_Get_NOPIN", C_PIN_SID_GET_NOPIN),
                ("C_PIN_SID_Set_PIN", C_PIN_SID_SET_PIN),
                (
                    "DataRemovalMechanism_Set_ActiveDataRemovalMechanism",
                    DATA_REMOVAL_MECHANISM_SET_ACTIVE_DATA_REMOVAL_MECHANISM,
                ),
                ("SP_SID", SP_SID),
                ("Set_Enabled", SET_ENABLED),
                ("TPerInfo_Set_ProgrammaticResetEnable", T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 10usize] = [
                (ANYBODY, "Anybody"),
                (ADMIN, "Admin"),
                (C_PIN_SID_GET_NOPIN, "C_PIN_SID_Get_NOPIN"),
                (C_PIN_SID_SET_PIN, "C_PIN_SID_Set_PIN"),
                (C_PIN_MSID_GET_PIN, "C_PIN_MSID_Get_PIN"),
                (SET_ENABLED, "Set_Enabled"),
                (SP_SID, "SP_SID"),
                (T_PER_INFO_SET_PROGRAMMATIC_RESET_ENABLE, "TPerInfo_Set_ProgrammaticResetEnable"),
                (C_PIN_ADMINS_SET_PIN, "C_PIN_Admins_Set_PIN"),
                (
                    DATA_REMOVAL_MECHANISM_SET_ACTIVE_DATA_REMOVAL_MECHANISM,
                    "DataRemovalMechanism_Set_ActiveDataRemovalMechanism",
                ),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 4usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::SP, &sp::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Admin",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    pub mod locking {
        use sed_packet::TableRef;
        pub mod k_aes_128 {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::K_AES_128;
            pub const GLOBAL_RANGE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080500000001_u64);
            pub const RANGE_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000080500030001_u64),
                end: ObjectRef::new(0x0000080500030001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] =
                [("GlobalRange_Key", GLOBAL_RANGE_KEY)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] =
                [(GLOBAL_RANGE_KEY, "GlobalRange_Key")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000080500030001_u64),
                        end: ObjectRef::new(0x0000080500030001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Range",
                    suffix: "_Key",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "K_AES_128",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod k_aes_256 {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::K_AES_256;
            pub const RANGE_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000080600030001_u64),
                end: ObjectRef::new(0x0000080600030001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const GLOBAL_RANGE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080600000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] =
                [("GlobalRange_Key", GLOBAL_RANGE_KEY)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] =
                [(GLOBAL_RANGE_KEY, "GlobalRange_Key")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000080600030001_u64),
                        end: ObjectRef::new(0x0000080600030001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Range",
                    suffix: "_Key",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "K_AES_256",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const USER: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900030001_u64),
                end: ObjectRef::new(0x0000000900030001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000900010001_u64),
                end: ObjectRef::new(0x0000000900010001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const USERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900030000_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 3usize] =
                [("Admins", ADMINS), ("Anybody", ANYBODY), ("Users", USERS)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 3usize] =
                [(ANYBODY, "Anybody"), (ADMINS, "Admins"), (USERS, "Users")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 2usize] = [
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900010001_u64),
                        end: ObjectRef::new(0x0000000900010001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000900030001_u64),
                        end: ObjectRef::new(0x0000000900030001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "User",
                    suffix: "",
                    display_base: 1u64,
                },
            ];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const LOCKING_RANGE_GET_RANGE_START_TO_ACTIVE_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> =
                UidRange {
                    start: ObjectRef::new(0x000000080003d001_u64),
                    end: ObjectRef::new(0x000000080003d001_u64 + 256u64 * 1u64),
                    step: 1u64 as u32,
                };
            pub const LOCKING_RANGE_SET_WR_LOCKED: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003e801_u64),
                end: ObjectRef::new(0x000000080003e801_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const AUTHORITY_SET_ENABLED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800039001_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000002_u64);
            pub const ANYBODY_GET_COMMON_NAME: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800000003_u64);
            pub const C_PIN_ADMINS_SET_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003a001_u64);
            pub const LOCKING_GLBL_RNG_ADMINS_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f000_u64);
            pub const K_AES_256_GLOBAL_RANGE_GEN_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003b800_u64);
            pub const MBR_CONTROL_SET_DONE_TO_DOR: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f801_u64);
            pub const K_AES_256_RANGE_GEN_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003b801_u64),
                end: ObjectRef::new(0x000000080003b801_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const ACE_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800038000_u64);
            pub const LOCKING_GLOBAL_RANGE_SET_WR_LOCKED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003e800_u64);
            pub const LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003d000_u64);
            pub const LOCKING_ADMINS_RANGE_START_TO_LOR: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f001_u64);
            pub const DATA_STORE_SET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003fc01_u64);
            pub const USER_SET_COMMON_NAME: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000800044001_u64),
                end: ObjectRef::new(0x0000000800044001_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const LOCKING_GLOBAL_RANGE_SET_RD_LOCKED: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003e000_u64);
            pub const K_AES_128_GLOBAL_RANGE_GEN_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003b000_u64);
            pub const C_PIN_USER_SET_PIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003a801_u64),
                end: ObjectRef::new(0x000000080003a801_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const C_PIN_ADMINS_GET_ALL_NOPIN: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003a000_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800000001_u64);
            pub const ADMINS_SET_COMMON_NAME: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800000004_u64);
            pub const K_AES_MODE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003bfff_u64);
            pub const AUTHORITY_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000800039000_u64);
            pub const MBR_CONTROL_ADMINS_SET: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x000000080003f800_u64);
            pub const ACE_SET_BOOLEAN_EXPRESSION: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x0000000800038001_u64);
            pub const LOCKING_RANGE_SET_RD_LOCKED: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003e001_u64),
                end: ObjectRef::new(0x000000080003e001_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const K_AES_128_RANGE_GEN_KEY: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003b001_u64),
                end: ObjectRef::new(0x000000080003b001_u64 + 256u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const DATA_STORE_GET_ALL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000080003fc00_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 22usize] = [
                ("ACE_Get_All", ACE_GET_ALL),
                ("ACE_Set_BooleanExpression", ACE_SET_BOOLEAN_EXPRESSION),
                ("Admin", ADMIN),
                ("Admins_Set_CommonName", ADMINS_SET_COMMON_NAME),
                ("Anybody", ANYBODY),
                ("Anybody_Get_CommonName", ANYBODY_GET_COMMON_NAME),
                ("Authority_Get_All", AUTHORITY_GET_ALL),
                ("Authority_Set_Enabled", AUTHORITY_SET_ENABLED),
                ("C_PIN_Admins_Get_All_NOPIN", C_PIN_ADMINS_GET_ALL_NOPIN),
                ("C_PIN_Admins_Set_PIN", C_PIN_ADMINS_SET_PIN),
                ("DataStore_Get_All", DATA_STORE_GET_ALL),
                ("DataStore_Set_All", DATA_STORE_SET_ALL),
                ("K_AES_128_GlobalRange_GenKey", K_AES_128_GLOBAL_RANGE_GEN_KEY),
                ("K_AES_256_GlobalRange_GenKey", K_AES_256_GLOBAL_RANGE_GEN_KEY),
                ("K_AES_Mode", K_AES_MODE),
                ("Locking_Admins_RangeStartToLOR", LOCKING_ADMINS_RANGE_START_TO_LOR),
                ("Locking_GlblRng_Admins_Set", LOCKING_GLBL_RNG_ADMINS_SET),
                ("Locking_GlobalRange_Get_RangeStartToActiveKey", LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY),
                ("Locking_GlobalRange_Set_RdLocked", LOCKING_GLOBAL_RANGE_SET_RD_LOCKED),
                ("Locking_GlobalRange_Set_WrLocked", LOCKING_GLOBAL_RANGE_SET_WR_LOCKED),
                ("MBRControl_Admins_Set", MBR_CONTROL_ADMINS_SET),
                ("MBRControl_Set_DoneToDOR", MBR_CONTROL_SET_DONE_TO_DOR),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 22usize] = [
                (ANYBODY, "Anybody"),
                (ADMIN, "Admin"),
                (ANYBODY_GET_COMMON_NAME, "Anybody_Get_CommonName"),
                (ADMINS_SET_COMMON_NAME, "Admins_Set_CommonName"),
                (ACE_GET_ALL, "ACE_Get_All"),
                (ACE_SET_BOOLEAN_EXPRESSION, "ACE_Set_BooleanExpression"),
                (AUTHORITY_GET_ALL, "Authority_Get_All"),
                (AUTHORITY_SET_ENABLED, "Authority_Set_Enabled"),
                (C_PIN_ADMINS_GET_ALL_NOPIN, "C_PIN_Admins_Get_All_NOPIN"),
                (C_PIN_ADMINS_SET_PIN, "C_PIN_Admins_Set_PIN"),
                (K_AES_128_GLOBAL_RANGE_GEN_KEY, "K_AES_128_GlobalRange_GenKey"),
                (K_AES_256_GLOBAL_RANGE_GEN_KEY, "K_AES_256_GlobalRange_GenKey"),
                (K_AES_MODE, "K_AES_Mode"),
                (LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY, "Locking_GlobalRange_Get_RangeStartToActiveKey"),
                (LOCKING_GLOBAL_RANGE_SET_RD_LOCKED, "Locking_GlobalRange_Set_RdLocked"),
                (LOCKING_GLOBAL_RANGE_SET_WR_LOCKED, "Locking_GlobalRange_Set_WrLocked"),
                (LOCKING_GLBL_RNG_ADMINS_SET, "Locking_GlblRng_Admins_Set"),
                (LOCKING_ADMINS_RANGE_START_TO_LOR, "Locking_Admins_RangeStartToLOR"),
                (MBR_CONTROL_ADMINS_SET, "MBRControl_Admins_Set"),
                (MBR_CONTROL_SET_DONE_TO_DOR, "MBRControl_Set_DoneToDOR"),
                (DATA_STORE_GET_ALL, "DataStore_Get_All"),
                (DATA_STORE_SET_ALL, "DataStore_Set_All"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 7usize] = [
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003a801_u64),
                        end: ObjectRef::new(0x000000080003a801_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "C_PIN_User",
                    suffix: "_Set_PIN",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003b001_u64),
                        end: ObjectRef::new(0x000000080003b001_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "K_AES_128_Range",
                    suffix: "_GenKey",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003b801_u64),
                        end: ObjectRef::new(0x000000080003b801_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "K_AES_256_Range",
                    suffix: "_GenKey",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003d001_u64),
                        end: ObjectRef::new(0x000000080003d001_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Locking_Range",
                    suffix: "_Get_RangeStartToActiveKey",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003e001_u64),
                        end: ObjectRef::new(0x000000080003e001_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Locking_Range",
                    suffix: "_Set_RdLocked",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003e801_u64),
                        end: ObjectRef::new(0x000000080003e801_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Locking_Range",
                    suffix: "_Set_WrLocked",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000800044001_u64),
                        end: ObjectRef::new(0x0000000800044001_u64).add(256u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "User",
                    suffix: "_Set_CommonName",
                    display_base: 1u64,
                },
            ];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const ADMIN: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00010001_u64),
                end: ObjectRef::new(0x0000000b00010001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            pub const USER: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000b00030001_u64),
                end: ObjectRef::new(0x0000000b00030001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 0usize] = [];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 0usize] = [];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 2usize] = [
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00010001_u64),
                        end: ObjectRef::new(0x0000000b00010001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Admin",
                    suffix: "",
                    display_base: 1u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000b00030001_u64),
                        end: ObjectRef::new(0x0000000b00030001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "User",
                    suffix: "",
                    display_base: 1u64,
                },
            ];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod locking {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::LOCKING;
            pub const GLOBAL_RANGE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080200000001_u64);
            pub const RANGE: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000080200030001_u64),
                end: ObjectRef::new(0x0000080200030001_u64 + 2048u64 * 1u64),
                step: 1u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("GlobalRange", GLOBAL_RANGE)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(GLOBAL_RANGE, "GlobalRange")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000080200030001_u64),
                        end: ObjectRef::new(0x0000080200030001_u64).add(2048u64 as u32 * 1u64 as u32),
                        step: 1u64 as u32,
                    },
                    prefix: "Range",
                    suffix: "",
                    display_base: 1u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Locking",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 6usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::LOCKING, &locking::LOOKUP),
            (super::super::core::shared::table_id::K_AES_128, &k_aes_128::LOOKUP),
            (super::super::core::shared::table_id::K_AES_256, &k_aes_256::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Locking",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    const SP_LOOKUPS: [(Option<SpRef>, &crate::lookup::SecurityProviderLookup); 2usize] = [
        (Some(admin::sp::ADMIN), &admin::LOOKUP),
        (Some(admin::sp::LOCKING), &locking::LOOKUP),
    ];
    pub const LOOKUP: crate::lookup::FeatureLookup<'static> =
        crate::lookup::FeatureLookup { sp_lookups: crate::lookup::SortedMap { items: &SP_LOOKUPS } };
}
pub mod psid {
    use crate::lookup::SpRef;
    pub mod locking {
        use sed_packet::TableRef;
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 0usize] = [];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Locking",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    pub mod admin {
        use sed_packet::TableRef;
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const PSID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000090001ff01_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("PSID", PSID)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(PSID, "PSID")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod sp {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SP;
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000001_u64);
            pub const LOCKING: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000002_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] =
                [("Admin", ADMIN), ("Locking", LOCKING)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] =
                [(ADMIN, "Admin"), (LOCKING, "Locking")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SP",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod c_pin {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::C_PIN;
            pub const PSID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000b0001ff01_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("PSID", PSID)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(PSID, "PSID")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "C_PIN",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const C_PIN_GET_PSID_NO_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> =
                ObjectRef::new(0x00000008000100e1_u64);
            pub const SP_PSID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x00000008000100e0_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] = [
                ("C_PIN_Get_PSID_NoPIN", C_PIN_GET_PSID_NO_PIN),
                ("SP_PSID", SP_PSID),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] = [
                (SP_PSID, "SP_PSID"),
                (C_PIN_GET_PSID_NO_PIN, "C_PIN_Get_PSID_NoPIN"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 4usize] = [
            (super::super::core::shared::table_id::ACE, &ace::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::C_PIN, &c_pin::LOOKUP),
            (super::super::core::shared::table_id::SP, &sp::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Admin",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    const SP_LOOKUPS: [(Option<SpRef>, &crate::lookup::SecurityProviderLookup); 2usize] = [
        (Some(admin::sp::ADMIN), &admin::LOOKUP),
        (Some(admin::sp::LOCKING), &locking::LOOKUP),
    ];
    pub const LOOKUP: crate::lookup::FeatureLookup<'static> =
        crate::lookup::FeatureLookup { sp_lookups: crate::lookup::SortedMap { items: &SP_LOOKUPS } };
}
pub mod data_store {
    use crate::lookup::SpRef;
    pub mod locking {
        use sed_packet::TableRef;
        pub mod ace {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::ACE;
            pub const DATA_STORE_GET_ALL: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003fc00_u64),
                end: ObjectRef::new(0x000000080003fc00_u64 + 2048u64 * 2u64),
                step: 2u64 as u32,
            };
            pub const DATA_STORE_SET_ALL: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x000000080003fc01_u64),
                end: ObjectRef::new(0x000000080003fc01_u64 + 2048u64 * 2u64),
                step: 2u64 as u32,
            };
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 0usize] = [];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 0usize] = [];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 2usize] = [
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003fc00_u64),
                        end: ObjectRef::new(0x000000080003fc00_u64).add(2048u64 as u32 * 2u64 as u32),
                        step: 2u64 as u32,
                    },
                    prefix: "DataStore",
                    suffix: "_Get_All",
                    display_base: 0u64,
                },
                crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x000000080003fc01_u64),
                        end: ObjectRef::new(0x000000080003fc01_u64).add(2048u64 as u32 * 2u64 as u32),
                        step: 2u64 as u32,
                    },
                    prefix: "DataStore",
                    suffix: "_Set_All",
                    display_base: 0u64,
                },
            ];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "ACE",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 1usize] =
            [(super::super::core::shared::table_id::ACE, &ace::LOOKUP)];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Locking",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    pub mod admin {
        use sed_packet::TableRef;
        pub mod sp {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SP;
            pub const LOCKING: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000002_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020500000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 2usize] =
                [("Admin", ADMIN), ("Locking", LOCKING)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 2usize] =
                [(ADMIN, "Admin"), (LOCKING, "Locking")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SP",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 1usize] =
            [(super::super::core::shared::table_id::SP, &sp::LOOKUP)];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Admin",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    const SP_LOOKUPS: [(Option<SpRef>, &crate::lookup::SecurityProviderLookup); 2usize] = [
        (Some(admin::sp::ADMIN), &admin::LOOKUP),
        (Some(admin::sp::LOCKING), &locking::LOOKUP),
    ];
    pub const LOOKUP: crate::lookup::FeatureLookup<'static> =
        crate::lookup::FeatureLookup { sp_lookups: crate::lookup::SortedMap { items: &SP_LOOKUPS } };
}
pub mod core {
    use crate::lookup::SpRef;
    pub mod shared {
        use sed_packet::TableRef;
        pub mod table_id {
            use ::sed_packet::{TableRef, UidRange};
            pub const SP_TEMPLATES: TableRef = TableRef::new(0x0000000300000000_u64);
            pub const T_PER_INFO: TableRef = TableRef::new(0x0000020100000000_u64);
            pub const C_HMAC_512: TableRef = TableRef::new(0x0000001c00000000_u64);
            pub const CLOCK_TIME: TableRef = TableRef::new(0x0000040100000000_u64);
            pub const C_EC_233: TableRef = TableRef::new(0x0000001700000000_u64);
            pub const H_SHA_384: TableRef = TableRef::new(0x0000060300000000_u64);
            pub const LOCKING_INFO: TableRef = TableRef::new(0x0000080100000000_u64);
            pub const K_AES_128: TableRef = TableRef::new(0x0000080500000000_u64);
            pub const K_AES_256: TableRef = TableRef::new(0x0000080600000000_u64);
            pub const SP_INFO: TableRef = TableRef::new(0x0000000200000000_u64);
            pub const INVOKING_ID: TableRef = TableRef::new(0xfffffffe00000000_u64);
            pub const CRYPTO_SUITE: TableRef = TableRef::new(0x0000020300000000_u64);
            pub const TYPE: TableRef = TableRef::new(0x0000000500000000_u64);
            pub const ACE: TableRef = TableRef::new(0x0000000800000000_u64);
            pub const LOCKING: TableRef = TableRef::new(0x0000080200000000_u64);
            pub const COLUMN: TableRef = TableRef::new(0x0000000400000000_u64);
            pub const C_EC_160: TableRef = TableRef::new(0x0000001000000000_u64);
            pub const C_EC_256: TableRef = TableRef::new(0x0000001300000000_u64);
            pub const LOG: TableRef = TableRef::new(0x00000a0100000000_u64);
            pub const GENERAL: TableRef = TableRef::new(0xfffffffd00000000_u64);
            pub const AUTHORITY: TableRef = TableRef::new(0x0000000900000000_u64);
            pub const C_EC_163: TableRef = TableRef::new(0x0000001600000000_u64);
            pub const SECRET_PROTECT: TableRef = TableRef::new(0x0000001d00000000_u64);
            pub const H_SHA_256: TableRef = TableRef::new(0x0000060200000000_u64);
            pub const SP: TableRef = TableRef::new(0x0000020500000000_u64);
            pub const TABLE: TableRef = TableRef::new(0x0000000100000000_u64);
            pub const C_RSA_2048: TableRef = TableRef::new(0x0000000d00000000_u64);
            pub const CERTIFICATES: TableRef = TableRef::new(0x0000000a00000000_u64);
            pub const C_AES_128: TableRef = TableRef::new(0x0000000e00000000_u64);
            pub const ACCESS_CONTROL: TableRef = TableRef::new(0x0000000700000000_u64);
            pub const H_SHA_512: TableRef = TableRef::new(0x0000060400000000_u64);
            pub const METHOD_ID: TableRef = TableRef::new(0x0000000600000000_u64);
            pub const C_RSA_1024: TableRef = TableRef::new(0x0000000c00000000_u64);
            pub const TEMPLATE: TableRef = TableRef::new(0x0000020400000000_u64);
            pub const C_EC_192: TableRef = TableRef::new(0x0000001100000000_u64);
            pub const H_SHA_1: TableRef = TableRef::new(0x0000060100000000_u64);
            pub const LOG_LIST: TableRef = TableRef::new(0x00000a0200000000_u64);
            pub const C_HMAC_160: TableRef = TableRef::new(0x0000001900000000_u64);
            pub const SM_METHOD_ID: TableRef = TableRef::new(0xffffffff00000000_u64);
            pub const C_PIN: TableRef = TableRef::new(0x0000000b00000000_u64);
            pub const C_AES_256: TableRef = TableRef::new(0x0000000f00000000_u64);
            pub const C_EC_384: TableRef = TableRef::new(0x0000001400000000_u64);
            pub const C_EC_521: TableRef = TableRef::new(0x0000001500000000_u64);
            pub const C_HMAC_384: TableRef = TableRef::new(0x0000001b00000000_u64);
            pub const MBR_CONTROL: TableRef = TableRef::new(0x0000080300000000_u64);
            pub const C_EC_283: TableRef = TableRef::new(0x0000001800000000_u64);
            pub const C_HMAC_256: TableRef = TableRef::new(0x0000001a00000000_u64);
            pub const C_EC_224: TableRef = TableRef::new(0x0000001200000000_u64);
            pub const MBR: TableRef = TableRef::new(0x0000080400000000_u64);
            pub const DATA_STORE: UidRange<TableRef> = UidRange {
                start: TableRef::new(0x0000100100000000_u64),
                end: TableRef::new(0x0000100100000000_u64).add(256u64 as u32 * 2u64 as u32),
                step: 2u64 as u32,
            };
            const BY_NAME: [(&str, TableRef); 49usize] = [
                ("ACE", ACE),
                ("AccessControl", ACCESS_CONTROL),
                ("Authority", AUTHORITY),
                ("C_AES_128", C_AES_128),
                ("C_AES_256", C_AES_256),
                ("C_EC_160", C_EC_160),
                ("C_EC_163", C_EC_163),
                ("C_EC_192", C_EC_192),
                ("C_EC_224", C_EC_224),
                ("C_EC_233", C_EC_233),
                ("C_EC_256", C_EC_256),
                ("C_EC_283", C_EC_283),
                ("C_EC_384", C_EC_384),
                ("C_EC_521", C_EC_521),
                ("C_HMAC_160", C_HMAC_160),
                ("C_HMAC_256", C_HMAC_256),
                ("C_HMAC_384", C_HMAC_384),
                ("C_HMAC_512", C_HMAC_512),
                ("C_PIN", C_PIN),
                ("C_RSA_1024", C_RSA_1024),
                ("C_RSA_2048", C_RSA_2048),
                ("Certificates", CERTIFICATES),
                ("ClockTime", CLOCK_TIME),
                ("Column", COLUMN),
                ("CryptoSuite", CRYPTO_SUITE),
                ("General", GENERAL),
                ("H_SHA_1", H_SHA_1),
                ("H_SHA_256", H_SHA_256),
                ("H_SHA_384", H_SHA_384),
                ("H_SHA_512", H_SHA_512),
                ("InvokingID", INVOKING_ID),
                ("K_AES_128", K_AES_128),
                ("K_AES_256", K_AES_256),
                ("Locking", LOCKING),
                ("LockingInfo", LOCKING_INFO),
                ("Log", LOG),
                ("LogList", LOG_LIST),
                ("MBR", MBR),
                ("MBRControl", MBR_CONTROL),
                ("MethodID", METHOD_ID),
                ("SMMethodID", SM_METHOD_ID),
                ("SP", SP),
                ("SPInfo", SP_INFO),
                ("SPTemplates", SP_TEMPLATES),
                ("SecretProtect", SECRET_PROTECT),
                ("TPerInfo", T_PER_INFO),
                ("Table", TABLE),
                ("Template", TEMPLATE),
                ("Type", TYPE),
            ];
            const BY_UID: [(TableRef, &str); 49usize] = [
                (TABLE, "Table"),
                (SP_INFO, "SPInfo"),
                (SP_TEMPLATES, "SPTemplates"),
                (COLUMN, "Column"),
                (TYPE, "Type"),
                (METHOD_ID, "MethodID"),
                (ACCESS_CONTROL, "AccessControl"),
                (ACE, "ACE"),
                (AUTHORITY, "Authority"),
                (CERTIFICATES, "Certificates"),
                (C_PIN, "C_PIN"),
                (C_RSA_1024, "C_RSA_1024"),
                (C_RSA_2048, "C_RSA_2048"),
                (C_AES_128, "C_AES_128"),
                (C_AES_256, "C_AES_256"),
                (C_EC_160, "C_EC_160"),
                (C_EC_192, "C_EC_192"),
                (C_EC_224, "C_EC_224"),
                (C_EC_256, "C_EC_256"),
                (C_EC_384, "C_EC_384"),
                (C_EC_521, "C_EC_521"),
                (C_EC_163, "C_EC_163"),
                (C_EC_233, "C_EC_233"),
                (C_EC_283, "C_EC_283"),
                (C_HMAC_160, "C_HMAC_160"),
                (C_HMAC_256, "C_HMAC_256"),
                (C_HMAC_384, "C_HMAC_384"),
                (C_HMAC_512, "C_HMAC_512"),
                (SECRET_PROTECT, "SecretProtect"),
                (T_PER_INFO, "TPerInfo"),
                (CRYPTO_SUITE, "CryptoSuite"),
                (TEMPLATE, "Template"),
                (SP, "SP"),
                (CLOCK_TIME, "ClockTime"),
                (H_SHA_1, "H_SHA_1"),
                (H_SHA_256, "H_SHA_256"),
                (H_SHA_384, "H_SHA_384"),
                (H_SHA_512, "H_SHA_512"),
                (LOCKING_INFO, "LockingInfo"),
                (LOCKING, "Locking"),
                (MBR_CONTROL, "MBRControl"),
                (MBR, "MBR"),
                (K_AES_128, "K_AES_128"),
                (K_AES_256, "K_AES_256"),
                (LOG, "Log"),
                (LOG_LIST, "LogList"),
                (GENERAL, "General"),
                (INVOKING_ID, "InvokingID"),
                (SM_METHOD_ID, "SMMethodID"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<TableRef>; 1usize] = [crate::lookup::UidRangeEntry {
                range: UidRange {
                    start: TableRef::new(0x0000100100000000_u64),
                    end: TableRef::new(0x0000100100000000_u64).add(256u64 as u32 * 2u64 as u32),
                    step: 2u64 as u32,
                },
                prefix: "DataStore",
                suffix: "",
                display_base: 0u64,
            }];
            pub const LOOKUP: crate::lookup::TableIdLookup<'static> = crate::lookup::TableIdLookup {
                by_name: crate::lookup::SortedMap { items: &BY_NAME },
                by_uid: crate::lookup::SortedMap { items: &BY_UID },
                ranges: &RANGES,
            };
        }
        pub mod template {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::TEMPLATE;
            pub const LOCKING: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020400000006_u64);
            pub const BASE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020400000001_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020400000002_u64);
            pub const CRYPTO: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020400000004_u64);
            pub const LOG: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020400000005_u64);
            pub const RESERVED: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020400000007_u64);
            pub const CLOCK: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020400000003_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 7usize] = [
                ("Admin", ADMIN),
                ("Base", BASE),
                ("Clock", CLOCK),
                ("Crypto", CRYPTO),
                ("Locking", LOCKING),
                ("Log", LOG),
                ("Reserved", RESERVED),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 7usize] = [
                (BASE, "Base"),
                (ADMIN, "Admin"),
                (CLOCK, "Clock"),
                (CRYPTO, "Crypto"),
                (LOG, "Log"),
                (LOCKING, "Locking"),
                (RESERVED, "Reserved"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Template",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod t_per_info {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::T_PER_INFO;
            pub const T_PER_INFO: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000020100000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("TPerInfo", T_PER_INFO)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(T_PER_INFO, "TPerInfo")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "TPerInfo",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod locking_info {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::LOCKING_INFO;
            pub const LOCKING_INFO: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080100000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("LockingInfo", LOCKING_INFO)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(LOCKING_INFO, "LockingInfo")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "LockingInfo",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod sm_method_id {
            use ::sed_packet::{TableRef, Uid};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SM_METHOD_ID;
            pub const PROPERTIES: Uid = Uid::new(0x000000000000ff01_u64);
            pub const START_SESSION: Uid = Uid::new(0x000000000000ff02_u64);
            pub const SYNC_TRUSTED_SESSION: Uid = Uid::new(0x000000000000ff05_u64);
            pub const CLOSE_SESSION: Uid = Uid::new(0x000000000000ff06_u64);
            pub const START_TRUSTED_SESSION: Uid = Uid::new(0x000000000000ff04_u64);
            pub const SYNC_SESSION: Uid = Uid::new(0x000000000000ff03_u64);
            const BY_NAME: [(&str, Uid); 6usize] = [
                ("CloseSession", CLOSE_SESSION),
                ("Properties", PROPERTIES),
                ("StartSession", START_SESSION),
                ("StartTrustedSession", START_TRUSTED_SESSION),
                ("SyncSession", SYNC_SESSION),
                ("SyncTrustedSession", SYNC_TRUSTED_SESSION),
            ];
            const BY_UID: [(Uid, &str); 6usize] = [
                (PROPERTIES, "Properties"),
                (START_SESSION, "StartSession"),
                (SYNC_SESSION, "SyncSession"),
                (START_TRUSTED_SESSION, "StartTrustedSession"),
                (SYNC_TRUSTED_SESSION, "SyncTrustedSession"),
                (CLOSE_SESSION, "CloseSession"),
            ];
            pub const LOOKUP: crate::lookup::MetaTableLookup<'static> = crate::lookup::MetaTableLookup {
                name: "SMMethodID",
                by_name: crate::lookup::SortedMap { items: &BY_NAME },
                by_uid: crate::lookup::SortedMap { items: &BY_UID },
            };
        }
        pub mod general {
            use ::sed_packet::{TableRef, Uid};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::GENERAL;
            pub const CHAR_SET_UNRESTRICTED: Uid = Uid::new(0x0000000b00000001_u64);
            pub const NULL: Uid = Uid::new(0x0000000000000000_u64);
            const BY_NAME: [(&str, Uid); 2usize] = [
                ("CharSetUnrestricted", CHAR_SET_UNRESTRICTED),
                ("Null", NULL),
            ];
            const BY_UID: [(Uid, &str); 2usize] = [
                (NULL, "Null"),
                (CHAR_SET_UNRESTRICTED, "CharSetUnrestricted"),
            ];
            pub const LOOKUP: crate::lookup::MetaTableLookup<'static> = crate::lookup::MetaTableLookup {
                name: "General",
                by_name: crate::lookup::SortedMap { items: &BY_NAME },
                by_uid: crate::lookup::SortedMap { items: &BY_UID },
            };
        }
        pub mod sp_templates {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SP_TEMPLATES;
            pub const RESERVED: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000300000007_u64);
            pub const LOCKING: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000300000006_u64);
            pub const ADMIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000300000002_u64);
            pub const CLOCK: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000300000003_u64);
            pub const BASE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000300000001_u64);
            pub const CRYPTO: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000300000004_u64);
            pub const LOG: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000300000005_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 7usize] = [
                ("Admin", ADMIN),
                ("Base", BASE),
                ("Clock", CLOCK),
                ("Crypto", CRYPTO),
                ("Locking", LOCKING),
                ("Log", LOG),
                ("Reserved", RESERVED),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 7usize] = [
                (BASE, "Base"),
                (ADMIN, "Admin"),
                (CLOCK, "Clock"),
                (CRYPTO, "Crypto"),
                (LOG, "Log"),
                (LOCKING, "Locking"),
                (RESERVED, "Reserved"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SPTemplates",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod method_id {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::METHOD_ID;
            pub const VERIFY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000610_u64);
            pub const SET_LAG_LOW: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000406_u64);
            pub const GET_FREE_SPACE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000009_u64);
            pub const CREATE_TABLE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000002_u64);
            pub const DELETE_METHOD: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000000b_u64);
            pub const AUTHENTICATE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000001c_u64);
            pub const ISSUE_SP: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000201_u64);
            pub const SET_LAG_HIGH: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000404_u64);
            pub const SET_CLOCK_LOW: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000405_u64);
            pub const DECRYPT_INIT: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000603_u64);
            pub const DELETE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000003_u64);
            pub const CREATE_ROW: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000004_u64);
            pub const HASH_FINALIZE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000060e_u64);
            pub const HASH: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000060d_u64);
            pub const ADD_LOG: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000a01_u64);
            pub const CREATE_LOG: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000a02_u64);
            pub const GET_FREE_ROWS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000000a_u64);
            pub const ENCRYPT_INIT: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000606_u64);
            pub const SIGN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000060f_u64);
            pub const HMAC: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000060a_u64);
            pub const SET: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000017_u64);
            pub const INCREMENT_COUNTER: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000407_u64);
            pub const RANDOM: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000601_u64);
            pub const CLEAR_LOG: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000a03_u64);
            pub const FLUSH_LOG: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000a04_u64);
            pub const REVERT_SP: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000011_u64);
            pub const ACTIVATE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000203_u64);
            pub const SET_PACKAGE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000013_u64);
            pub const DECRYPT_FINALIZE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000605_u64);
            pub const REACTIVATE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000801_u64);
            pub const DELETE_ROW: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000005_u64);
            pub const ERASE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000803_u64);
            pub const OBSOLETE_0006: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000006_u64);
            pub const ADD_ACE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000000e_u64);
            pub const HMAC_INIT: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000609_u64);
            pub const ENCRYPT_FINALIZE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000608_u64);
            pub const HMAC_FINALIZE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000060b_u64);
            pub const REVERT: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000202_u64);
            pub const GET_ACL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000000d_u64);
            pub const GET_CLOCK: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000401_u64);
            pub const REMOVE_ACE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000000f_u64);
            pub const ENCRYPT: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000607_u64);
            pub const SET_CLOCK_HIGH: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000403_u64);
            pub const XOR: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000611_u64);
            pub const GET_PACKAGE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000012_u64);
            pub const NEXT: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000008_u64);
            pub const DECRYPT: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000604_u64);
            pub const GEN_KEY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000010_u64);
            pub const OBSOLETE_0007: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000007_u64);
            pub const DELETE_SP: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000001_u64);
            pub const OBSOLETE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000000c_u64);
            pub const GET: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000016_u64);
            pub const RESET_CLOCK: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000402_u64);
            pub const HASH_INIT: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000060000060c_u64);
            pub const SALT: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000600000602_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 55usize] = [
                ("Activate", ACTIVATE),
                ("AddACE", ADD_ACE),
                ("AddLog", ADD_LOG),
                ("Authenticate", AUTHENTICATE),
                ("ClearLog", CLEAR_LOG),
                ("CreateLog", CREATE_LOG),
                ("CreateRow", CREATE_ROW),
                ("CreateTable", CREATE_TABLE),
                ("Decrypt", DECRYPT),
                ("DecryptFinalize", DECRYPT_FINALIZE),
                ("DecryptInit", DECRYPT_INIT),
                ("Delete", DELETE),
                ("DeleteMethod", DELETE_METHOD),
                ("DeleteRow", DELETE_ROW),
                ("DeleteSP", DELETE_SP),
                ("Encrypt", ENCRYPT),
                ("EncryptFinalize", ENCRYPT_FINALIZE),
                ("EncryptInit", ENCRYPT_INIT),
                ("Erase", ERASE),
                ("FlushLog", FLUSH_LOG),
                ("GenKey", GEN_KEY),
                ("Get", GET),
                ("GetACL", GET_ACL),
                ("GetClock", GET_CLOCK),
                ("GetFreeRows", GET_FREE_ROWS),
                ("GetFreeSpace", GET_FREE_SPACE),
                ("GetPackage", GET_PACKAGE),
                ("HMAC", HMAC),
                ("HMACFinalize", HMAC_FINALIZE),
                ("HMACInit", HMAC_INIT),
                ("Hash", HASH),
                ("HashFinalize", HASH_FINALIZE),
                ("HashInit", HASH_INIT),
                ("IncrementCounter", INCREMENT_COUNTER),
                ("IssueSP", ISSUE_SP),
                ("Next", NEXT),
                ("OBSOLETE", OBSOLETE),
                ("OBSOLETE_0006", OBSOLETE_0006),
                ("OBSOLETE_0007", OBSOLETE_0007),
                ("Random", RANDOM),
                ("Reactivate", REACTIVATE),
                ("RemoveACE", REMOVE_ACE),
                ("ResetClock", RESET_CLOCK),
                ("Revert", REVERT),
                ("RevertSP", REVERT_SP),
                ("Salt", SALT),
                ("Set", SET),
                ("SetClockHigh", SET_CLOCK_HIGH),
                ("SetClockLow", SET_CLOCK_LOW),
                ("SetLagHigh", SET_LAG_HIGH),
                ("SetLagLow", SET_LAG_LOW),
                ("SetPackage", SET_PACKAGE),
                ("Sign", SIGN),
                ("Verify", VERIFY),
                ("XOR", XOR),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 55usize] = [
                (DELETE_SP, "DeleteSP"),
                (CREATE_TABLE, "CreateTable"),
                (DELETE, "Delete"),
                (CREATE_ROW, "CreateRow"),
                (DELETE_ROW, "DeleteRow"),
                (OBSOLETE_0006, "OBSOLETE_0006"),
                (OBSOLETE_0007, "OBSOLETE_0007"),
                (NEXT, "Next"),
                (GET_FREE_SPACE, "GetFreeSpace"),
                (GET_FREE_ROWS, "GetFreeRows"),
                (DELETE_METHOD, "DeleteMethod"),
                (OBSOLETE, "OBSOLETE"),
                (GET_ACL, "GetACL"),
                (ADD_ACE, "AddACE"),
                (REMOVE_ACE, "RemoveACE"),
                (GEN_KEY, "GenKey"),
                (REVERT_SP, "RevertSP"),
                (GET_PACKAGE, "GetPackage"),
                (SET_PACKAGE, "SetPackage"),
                (GET, "Get"),
                (SET, "Set"),
                (AUTHENTICATE, "Authenticate"),
                (ISSUE_SP, "IssueSP"),
                (REVERT, "Revert"),
                (ACTIVATE, "Activate"),
                (GET_CLOCK, "GetClock"),
                (RESET_CLOCK, "ResetClock"),
                (SET_CLOCK_HIGH, "SetClockHigh"),
                (SET_LAG_HIGH, "SetLagHigh"),
                (SET_CLOCK_LOW, "SetClockLow"),
                (SET_LAG_LOW, "SetLagLow"),
                (INCREMENT_COUNTER, "IncrementCounter"),
                (RANDOM, "Random"),
                (SALT, "Salt"),
                (DECRYPT_INIT, "DecryptInit"),
                (DECRYPT, "Decrypt"),
                (DECRYPT_FINALIZE, "DecryptFinalize"),
                (ENCRYPT_INIT, "EncryptInit"),
                (ENCRYPT, "Encrypt"),
                (ENCRYPT_FINALIZE, "EncryptFinalize"),
                (HMAC_INIT, "HMACInit"),
                (HMAC, "HMAC"),
                (HMAC_FINALIZE, "HMACFinalize"),
                (HASH_INIT, "HashInit"),
                (HASH, "Hash"),
                (HASH_FINALIZE, "HashFinalize"),
                (SIGN, "Sign"),
                (VERIFY, "Verify"),
                (XOR, "XOR"),
                (REACTIVATE, "Reactivate"),
                (ERASE, "Erase"),
                (ADD_LOG, "AddLog"),
                (CREATE_LOG, "CreateLog"),
                (CLEAR_LOG, "ClearLog"),
                (FLUSH_LOG, "FlushLog"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "MethodID",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod sp_info {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SP_INFO;
            pub const SP_INFO: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000200000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("SPInfo", SP_INFO)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(SP_INFO, "SPInfo")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SPInfo",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod table {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::TABLE;
            pub const DATA_STORE: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = UidRange {
                start: ObjectRef::new(0x0000000100001001_u64),
                end: ObjectRef::new(0x0000000100001001_u64 + 256u64 * 2u64),
                step: 2u64 as u32,
            };
            pub const SP_TEMPLATES: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000003_u64);
            pub const COLUMN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000004_u64);
            pub const ACE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000008_u64);
            pub const TEMPLATE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000204_u64);
            pub const C_EC_160: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000010_u64);
            pub const ACCESS_CONTROL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000007_u64);
            pub const C_HMAC_256: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000010000001a_u64);
            pub const C_EC_384: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000014_u64);
            pub const C_AES_256: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000010000000f_u64);
            pub const C_EC_192: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000011_u64);
            pub const CRYPTO_SUITE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000203_u64);
            pub const H_SHA_1: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000601_u64);
            pub const C_EC_233: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000017_u64);
            pub const TABLE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000001_u64);
            pub const LOG: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000a01_u64);
            pub const LOCKING_INFO: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000801_u64);
            pub const C_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000010000000b_u64);
            pub const SECRET_PROTECT: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000010000001d_u64);
            pub const C_EC_163: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000016_u64);
            pub const C_RSA_1024: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000010000000c_u64);
            pub const H_SHA_512: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000604_u64);
            pub const C_HMAC_384: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000010000001b_u64);
            pub const LOG_LIST: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000a02_u64);
            pub const LOCKING: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000802_u64);
            pub const CERTIFICATES: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000010000000a_u64);
            pub const C_HMAC_160: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000019_u64);
            pub const AUTHORITY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000009_u64);
            pub const C_EC_283: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000018_u64);
            pub const TYPE: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000005_u64);
            pub const K_AES_256: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000806_u64);
            pub const SP: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000205_u64);
            pub const H_SHA_384: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000603_u64);
            pub const C_EC_224: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000012_u64);
            pub const T_PER_INFO: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000201_u64);
            pub const METHOD_ID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000006_u64);
            pub const H_SHA_256: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000602_u64);
            pub const MBR_CONTROL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000803_u64);
            pub const C_RSA_2048: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000010000000d_u64);
            pub const C_EC_521: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000015_u64);
            pub const C_AES_128: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000010000000e_u64);
            pub const SP_INFO: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000002_u64);
            pub const CLOCK_TIME: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000401_u64);
            pub const MBR: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000804_u64);
            pub const C_EC_256: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000013_u64);
            pub const K_AES_128: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000100000805_u64);
            pub const C_HMAC_512: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x000000010000001c_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 46usize] = [
                ("ACE", ACE),
                ("AccessControl", ACCESS_CONTROL),
                ("Authority", AUTHORITY),
                ("C_AES_128", C_AES_128),
                ("C_AES_256", C_AES_256),
                ("C_EC_160", C_EC_160),
                ("C_EC_163", C_EC_163),
                ("C_EC_192", C_EC_192),
                ("C_EC_224", C_EC_224),
                ("C_EC_233", C_EC_233),
                ("C_EC_256", C_EC_256),
                ("C_EC_283", C_EC_283),
                ("C_EC_384", C_EC_384),
                ("C_EC_521", C_EC_521),
                ("C_HMAC_160", C_HMAC_160),
                ("C_HMAC_256", C_HMAC_256),
                ("C_HMAC_384", C_HMAC_384),
                ("C_HMAC_512", C_HMAC_512),
                ("C_PIN", C_PIN),
                ("C_RSA_1024", C_RSA_1024),
                ("C_RSA_2048", C_RSA_2048),
                ("Certificates", CERTIFICATES),
                ("ClockTime", CLOCK_TIME),
                ("Column", COLUMN),
                ("CryptoSuite", CRYPTO_SUITE),
                ("H_SHA_1", H_SHA_1),
                ("H_SHA_256", H_SHA_256),
                ("H_SHA_384", H_SHA_384),
                ("H_SHA_512", H_SHA_512),
                ("K_AES_128", K_AES_128),
                ("K_AES_256", K_AES_256),
                ("Locking", LOCKING),
                ("LockingInfo", LOCKING_INFO),
                ("Log", LOG),
                ("LogList", LOG_LIST),
                ("MBR", MBR),
                ("MBRControl", MBR_CONTROL),
                ("MethodID", METHOD_ID),
                ("SP", SP),
                ("SPInfo", SP_INFO),
                ("SPTemplates", SP_TEMPLATES),
                ("SecretProtect", SECRET_PROTECT),
                ("TPerInfo", T_PER_INFO),
                ("Table", TABLE),
                ("Template", TEMPLATE),
                ("Type", TYPE),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 46usize] = [
                (TABLE, "Table"),
                (SP_INFO, "SPInfo"),
                (SP_TEMPLATES, "SPTemplates"),
                (COLUMN, "Column"),
                (TYPE, "Type"),
                (METHOD_ID, "MethodID"),
                (ACCESS_CONTROL, "AccessControl"),
                (ACE, "ACE"),
                (AUTHORITY, "Authority"),
                (CERTIFICATES, "Certificates"),
                (C_PIN, "C_PIN"),
                (C_RSA_1024, "C_RSA_1024"),
                (C_RSA_2048, "C_RSA_2048"),
                (C_AES_128, "C_AES_128"),
                (C_AES_256, "C_AES_256"),
                (C_EC_160, "C_EC_160"),
                (C_EC_192, "C_EC_192"),
                (C_EC_224, "C_EC_224"),
                (C_EC_256, "C_EC_256"),
                (C_EC_384, "C_EC_384"),
                (C_EC_521, "C_EC_521"),
                (C_EC_163, "C_EC_163"),
                (C_EC_233, "C_EC_233"),
                (C_EC_283, "C_EC_283"),
                (C_HMAC_160, "C_HMAC_160"),
                (C_HMAC_256, "C_HMAC_256"),
                (C_HMAC_384, "C_HMAC_384"),
                (C_HMAC_512, "C_HMAC_512"),
                (SECRET_PROTECT, "SecretProtect"),
                (T_PER_INFO, "TPerInfo"),
                (CRYPTO_SUITE, "CryptoSuite"),
                (TEMPLATE, "Template"),
                (SP, "SP"),
                (CLOCK_TIME, "ClockTime"),
                (H_SHA_1, "H_SHA_1"),
                (H_SHA_256, "H_SHA_256"),
                (H_SHA_384, "H_SHA_384"),
                (H_SHA_512, "H_SHA_512"),
                (LOCKING_INFO, "LockingInfo"),
                (LOCKING, "Locking"),
                (MBR_CONTROL, "MBRControl"),
                (MBR, "MBR"),
                (K_AES_128, "K_AES_128"),
                (K_AES_256, "K_AES_256"),
                (LOG, "Log"),
                (LOG_LIST, "LogList"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 1usize] =
                [crate::lookup::UidRangeEntry {
                    range: UidRange {
                        start: ObjectRef::new(0x0000000100001001_u64),
                        end: ObjectRef::new(0x0000000100001001_u64).add(256u64 as u32 * 2u64 as u32),
                        step: 2u64 as u32,
                    },
                    prefix: "DataStore",
                    suffix: "",
                    display_base: 0u64,
                }];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Table",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod mbr_control {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::MBR_CONTROL;
            pub const MBR_CONTROL: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000080300000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 1usize] = [("MBRControl", MBR_CONTROL)];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 1usize] = [(MBR_CONTROL, "MBRControl")];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "MBRControl",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod authority {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::AUTHORITY;
            pub const MAKER_SYM_K: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000004_u64);
            pub const ANYBODY: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000001_u64);
            pub const ADMINS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000002_u64);
            pub const SID: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000006_u64);
            pub const SERVERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000204_u64);
            pub const RESERVE2: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000207_u64);
            pub const MAKERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000003_u64);
            pub const EDITORS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000202_u64);
            pub const ISSUERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000201_u64);
            pub const T_PER_SIGN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000007_u64);
            pub const RESERVE0: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000205_u64);
            pub const RESERVE3: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000208_u64);
            pub const DELETERS: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000203_u64);
            pub const MAKER_PU_K: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000005_u64);
            pub const ADMIN_EXCH: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000009_u64);
            pub const T_PER_EXCH: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000008_u64);
            pub const RESERVE1: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000000900000206_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 17usize] = [
                ("AdminExch", ADMIN_EXCH),
                ("Admins", ADMINS),
                ("Anybody", ANYBODY),
                ("Deleters", DELETERS),
                ("Editors", EDITORS),
                ("Issuers", ISSUERS),
                ("MakerPuK", MAKER_PU_K),
                ("MakerSymK", MAKER_SYM_K),
                ("Makers", MAKERS),
                ("Reserve0", RESERVE0),
                ("Reserve1", RESERVE1),
                ("Reserve2", RESERVE2),
                ("Reserve3", RESERVE3),
                ("SID", SID),
                ("Servers", SERVERS),
                ("TPerExch", T_PER_EXCH),
                ("TPerSign", T_PER_SIGN),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 17usize] = [
                (ANYBODY, "Anybody"),
                (ADMINS, "Admins"),
                (MAKERS, "Makers"),
                (MAKER_SYM_K, "MakerSymK"),
                (MAKER_PU_K, "MakerPuK"),
                (SID, "SID"),
                (T_PER_SIGN, "TPerSign"),
                (T_PER_EXCH, "TPerExch"),
                (ADMIN_EXCH, "AdminExch"),
                (ISSUERS, "Issuers"),
                (EDITORS, "Editors"),
                (DELETERS, "Deleters"),
                (SERVERS, "Servers"),
                (RESERVE0, "Reserve0"),
                (RESERVE1, "Reserve1"),
                (RESERVE2, "Reserve2"),
                (RESERVE3, "Reserve3"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "Authority",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        pub mod invoking_id {
            use ::sed_packet::{TableRef, Uid};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::INVOKING_ID;
            pub const SESSION_MANAGER: Uid = Uid::new(0x00000000000000ff_u64);
            pub const THIS_SP: Uid = Uid::new(0x0000000000000001_u64);
            const BY_NAME: [(&str, Uid); 2usize] = [("SessionManager", SESSION_MANAGER), ("ThisSP", THIS_SP)];
            const BY_UID: [(Uid, &str); 2usize] = [(THIS_SP, "ThisSP"), (SESSION_MANAGER, "SessionManager")];
            pub const LOOKUP: crate::lookup::MetaTableLookup<'static> = crate::lookup::MetaTableLookup {
                name: "InvokingID",
                by_name: crate::lookup::SortedMap { items: &BY_NAME },
                by_uid: crate::lookup::SortedMap { items: &BY_UID },
            };
        }
        pub mod secret_protect {
            use ::sed_packet::{ObjectRef, TableRef, UidRange};
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::SECRET_PROTECT;
            pub const C_AES_128: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000000e_u64);
            pub const C_EC_384: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000014_u64);
            pub const K_AES_256: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000001e_u64);
            pub const C_RSA_1024_9: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000005_u64);
            pub const C_HMAC_160: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000019_u64);
            pub const C_EC_256: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000013_u64);
            pub const C_HMAC_512: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000001c_u64);
            pub const C_RSA_1024_A: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000006_u64);
            pub const C_HMAC_384: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000001b_u64);
            pub const C_AES_256: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000000f_u64);
            pub const C_EC_224: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000012_u64);
            pub const C_RSA_2048_7: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000009_u64);
            pub const C_EC_283: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000018_u64);
            pub const C_RSA_1024_B: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000007_u64);
            pub const C_EC_163: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000016_u64);
            pub const C_EC_160: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000010_u64);
            pub const C_RSA_1024_6: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000002_u64);
            pub const C_RSA_2048_6: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000008_u64);
            pub const C_RSA_2048_8: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000000a_u64);
            pub const C_RSA_2048_9: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000000b_u64);
            pub const C_EC_233: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000017_u64);
            pub const C_EC_521: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000015_u64);
            pub const K_AES_128: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000001d_u64);
            pub const C_RSA_2048_B: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000000d_u64);
            pub const C_HMAC_256: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000001a_u64);
            pub const C_RSA_1024_8: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000004_u64);
            pub const C_RSA_2048_A: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d0000000c_u64);
            pub const C_EC_192: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000011_u64);
            pub const C_RSA_1024_7: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000003_u64);
            pub const C_PIN: ObjectRef<{ THIS_TABLE.to_u64() }> = ObjectRef::new(0x0000001d00000001_u64);
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); 30usize] = [
                ("C_AES_128", C_AES_128),
                ("C_AES_256", C_AES_256),
                ("C_EC_160", C_EC_160),
                ("C_EC_163", C_EC_163),
                ("C_EC_192", C_EC_192),
                ("C_EC_224", C_EC_224),
                ("C_EC_233", C_EC_233),
                ("C_EC_256", C_EC_256),
                ("C_EC_283", C_EC_283),
                ("C_EC_384", C_EC_384),
                ("C_EC_521", C_EC_521),
                ("C_HMAC_160", C_HMAC_160),
                ("C_HMAC_256", C_HMAC_256),
                ("C_HMAC_384", C_HMAC_384),
                ("C_HMAC_512", C_HMAC_512),
                ("C_PIN", C_PIN),
                ("C_RSA_1024_6", C_RSA_1024_6),
                ("C_RSA_1024_7", C_RSA_1024_7),
                ("C_RSA_1024_8", C_RSA_1024_8),
                ("C_RSA_1024_9", C_RSA_1024_9),
                ("C_RSA_1024_A", C_RSA_1024_A),
                ("C_RSA_1024_B", C_RSA_1024_B),
                ("C_RSA_2048_6", C_RSA_2048_6),
                ("C_RSA_2048_7", C_RSA_2048_7),
                ("C_RSA_2048_8", C_RSA_2048_8),
                ("C_RSA_2048_9", C_RSA_2048_9),
                ("C_RSA_2048_A", C_RSA_2048_A),
                ("C_RSA_2048_B", C_RSA_2048_B),
                ("K_AES_128", K_AES_128),
                ("K_AES_256", K_AES_256),
            ];
            const BY_UID: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); 30usize] = [
                (C_PIN, "C_PIN"),
                (C_RSA_1024_6, "C_RSA_1024_6"),
                (C_RSA_1024_7, "C_RSA_1024_7"),
                (C_RSA_1024_8, "C_RSA_1024_8"),
                (C_RSA_1024_9, "C_RSA_1024_9"),
                (C_RSA_1024_A, "C_RSA_1024_A"),
                (C_RSA_1024_B, "C_RSA_1024_B"),
                (C_RSA_2048_6, "C_RSA_2048_6"),
                (C_RSA_2048_7, "C_RSA_2048_7"),
                (C_RSA_2048_8, "C_RSA_2048_8"),
                (C_RSA_2048_9, "C_RSA_2048_9"),
                (C_RSA_2048_A, "C_RSA_2048_A"),
                (C_RSA_2048_B, "C_RSA_2048_B"),
                (C_AES_128, "C_AES_128"),
                (C_AES_256, "C_AES_256"),
                (C_EC_160, "C_EC_160"),
                (C_EC_192, "C_EC_192"),
                (C_EC_224, "C_EC_224"),
                (C_EC_256, "C_EC_256"),
                (C_EC_384, "C_EC_384"),
                (C_EC_521, "C_EC_521"),
                (C_EC_163, "C_EC_163"),
                (C_EC_233, "C_EC_233"),
                (C_EC_283, "C_EC_283"),
                (C_HMAC_160, "C_HMAC_160"),
                (C_HMAC_256, "C_HMAC_256"),
                (C_HMAC_384, "C_HMAC_384"),
                (C_HMAC_512, "C_HMAC_512"),
                (K_AES_128, "K_AES_128"),
                (K_AES_256, "K_AES_256"),
            ];
            const RANGES: [crate::lookup::UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; 0usize] = [];
            pub const LOOKUP: crate::lookup::ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> =
                crate::lookup::ObjectTableLookup {
                    name: "SecretProtect",
                    by_name: crate::lookup::SortedMap { items: &BY_NAME },
                    by_uid: crate::lookup::SortedMap { items: &BY_UID },
                    ranges: &RANGES,
                };
        }
        const TABLE_LOOKUPS: [(TableRef, &dyn crate::lookup::TableLookup); 13usize] = [
            (super::super::core::shared::table_id::TABLE, &table::LOOKUP),
            (super::super::core::shared::table_id::SP_INFO, &sp_info::LOOKUP),
            (super::super::core::shared::table_id::SP_TEMPLATES, &sp_templates::LOOKUP),
            (super::super::core::shared::table_id::METHOD_ID, &method_id::LOOKUP),
            (super::super::core::shared::table_id::AUTHORITY, &authority::LOOKUP),
            (super::super::core::shared::table_id::SECRET_PROTECT, &secret_protect::LOOKUP),
            (super::super::core::shared::table_id::T_PER_INFO, &t_per_info::LOOKUP),
            (super::super::core::shared::table_id::TEMPLATE, &template::LOOKUP),
            (super::super::core::shared::table_id::LOCKING_INFO, &locking_info::LOOKUP),
            (super::super::core::shared::table_id::MBR_CONTROL, &mbr_control::LOOKUP),
            (super::super::core::shared::table_id::GENERAL, &general::LOOKUP),
            (super::super::core::shared::table_id::INVOKING_ID, &invoking_id::LOOKUP),
            (super::super::core::shared::table_id::SM_METHOD_ID, &sm_method_id::LOOKUP),
        ];
        pub const LOOKUP: crate::lookup::SecurityProviderLookup<'static> = crate::lookup::SecurityProviderLookup {
            name: "Shared",
            table_ids: &super::super::core::shared::table_id::LOOKUP,
            table_lookups: crate::lookup::SortedMap { items: &TABLE_LOOKUPS },
        };
    }
    const SP_LOOKUPS: [(Option<SpRef>, &crate::lookup::SecurityProviderLookup); 1usize] = [(None, &shared::LOOKUP)];
    pub const LOOKUP: crate::lookup::FeatureLookup<'static> =
        crate::lookup::FeatureLookup { sp_lookups: crate::lookup::SortedMap { items: &SP_LOOKUPS } };
}
