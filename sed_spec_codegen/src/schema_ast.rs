//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

/// A single UID or a range of UIDs inside a table of the JSON.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum UidEntry {
    /// A single UID in the JSON.
    Single(#[serde(deserialize_with = "deserialize_uid")] u64),
    /// A contiguous range of UIDs, named `<prefix>{n}<suffix>` in the JSON.
    Range {
        /// The first UID of the range.
        #[serde(deserialize_with = "deserialize_uid")]
        start: u64,
        /// The number of UIDs in the range.
        count: u64,
        #[serde(default = "default_value_one")]
        /// Only every `step`th UID is part of the range.
        step: u32,
        /// When the UID is rendered as a string, `display_start` will be
        /// displayed as the index of the 0th UID in this range.
        #[serde(default = "default_value_one")]
        display_start: u64,
    },
}

/// A generic table that might contains table IDs, special purpose UIDs, or
/// object UIDs.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Table {
    #[serde(flatten)]
    pub entries: BTreeMap<String, UidEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct SecurityProvider {
    /// The table that lists the UIDs of tables.
    #[serde(rename = "TableID")]
    pub table_id: Option<Table>,
    /// The table of special-purpose UIDs that aren't further categorized.
    #[serde(rename = "General")]
    pub general: Option<Table>,
    /// The table of special purpose UIDs used as the invoking ID.
    #[serde(rename = "InvokingID")]
    pub invoking_id: Option<Table>,
    /// The table of session manager method special-purpose UIDs.
    #[serde(rename = "SMMethodID")]
    pub sm_method_id: Option<Table>,
    /// All remaining tables, which are object tables.
    #[serde(flatten)]
    pub tables: BTreeMap<String, Table>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Feature {
    #[serde(flatten)]
    pub security_providers: BTreeMap<String, SecurityProvider>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UidRegistry {
    #[serde(flatten)]
    pub features: BTreeMap<String, Feature>,
}

fn deserialize_uid<'de, D>(de: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    u64::from_str_radix(&String::deserialize(de)?, 16).map_err(|_| D::Error::custom("could not parse UID"))
}

fn default_value_one<T: From<u8>>() -> T {
    T::from(1)
}

#[cfg(test)]
mod tests {
    use googletest::{assert_that, matchers::*};

    use super::*;

    #[test]
    fn verify_loaded_content() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_uid_registry.json"));

        let shared = SecurityProvider {
            table_id: Some(Table {
                entries: [
                    ("InvokingID".to_owned(), UidEntry::Single(0xFFFFFFFE00000000)),
                    ("SP".to_owned(), UidEntry::Single(0x0000020500000000)),
                    ("TableW".to_owned(), UidEntry::Single(0x0000000A00000000)),
                    ("TableZ".to_owned(), UidEntry::Single(0x0000000900000000)),
                    (
                        "DataStore{n}".to_owned(),
                        UidEntry::Range { start: 0x0000100100000000, count: 4, step: 2, display_start: 0 },
                    ),
                ]
                .into(),
            }),
            invoking_id: Some(Table {
                entries: [
                    ("ThisSP".to_owned(), UidEntry::Single(0x0000000000000001)),
                    ("SessionManager".to_owned(), UidEntry::Single(0x00000000000000FF)),
                ]
                .into(),
            }),
            ..Default::default()
        };
        let admin = SecurityProvider {
            tables: [(
                "SP".to_owned(),
                Table {
                    entries: [
                        ("Admin".to_owned(), UidEntry::Single(0x0000020500000002)),
                        ("SPY".to_owned(), UidEntry::Single(0x0000020500000001)),
                    ]
                    .into(),
                },
            )]
            .into(),
            ..Default::default()
        };
        let spy = SecurityProvider {
            tables: [
                (
                    "TableW".to_owned(),
                    Table { entries: [("ObjectD".to_owned(), UidEntry::Single(0x0000000A00000001))].into() },
                ),
                (
                    "TableZ".to_owned(),
                    Table {
                        entries: [
                            ("ObjectA".to_owned(), UidEntry::Single(0x0000000900000001)),
                            (
                                "ObjectB{n}".to_owned(),
                                UidEntry::Range { start: 0x0000000900000002, count: 100, step: 2, display_start: 0 },
                            ),
                            // `step` and `display_start` are omitted in the JSON.
                            (
                                "ObjectC{n}_Suffix".to_owned(),
                                UidEntry::Range { start: 0x0000000900001000, count: 8, step: 1, display_start: 1 },
                            ),
                        ]
                        .into(),
                    },
                ),
            ]
            .into(),
            ..Default::default()
        };
        let expected = UidRegistry {
            features: [
                ("Core".to_owned(), Feature { security_providers: [("Shared".to_owned(), shared)].into() }),
                (
                    "FeatureX".to_owned(),
                    Feature { security_providers: [("Admin".to_owned(), admin), ("SPY".to_owned(), spy)].into() },
                ),
            ]
            .into(),
        };

        let registry: UidRegistry = serde_json::from_str(input).unwrap();
        assert_that!(registry, eq(&expected));
    }
}
