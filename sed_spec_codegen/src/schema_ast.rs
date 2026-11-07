//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

/// A contiguous range of UIDs, named `<prefix>{n}<suffix>` in the JSON.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UidRange {
    #[serde(deserialize_with = "deserialize_uid", rename = "base")]
    pub start: u64,
    pub count: u64,
    #[serde(default = "default_value_one")]
    pub step: u32,
    #[serde(default = "default_value_one", rename = "display_base")]
    pub start_display: u64,
}

/// An entry of a table: either a single UID or a range of UIDs.
///
/// The interpretation of the entry (e.g. object, table, or special-purpose UID)
/// depends on the table, and is decided in the semantic AST.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum Entry {
    Uid(#[serde(deserialize_with = "deserialize_uid")] u64),
    UidRange(UidRange),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Table {
    #[serde(flatten)]
    pub entries: BTreeMap<String, Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct SecurityProvider {
    /// The table that lists the UIDs of tables.
    #[serde(rename = "TableID")]
    pub table_id: Option<Table>,
    /// Tables of special-purpose UIDs (see Core Specification, Table 239 "Special Purpose UIDs").
    ///
    /// These tables don't exist in the TCG specifications, they only group the
    /// special-purpose UIDs so that they can be looked up the same way as objects.
    #[serde(rename = "General")]
    pub general: Option<Table>,
    #[serde(rename = "InvokingID")]
    pub invoking_id: Option<Table>,
    #[serde(rename = "SMMethodID")]
    pub sm_method_id: Option<Table>,
    /// All tables not captured by the fields above, whose rows are objects.
    #[serde(flatten)]
    pub tables: BTreeMap<String, Table>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Feature {
    #[serde(flatten)]
    pub security_providers: BTreeMap<String, SecurityProvider>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Database {
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
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_database.json"));

        let shared = SecurityProvider {
            table_id: Some(Table {
                entries: [
                    ("InvokingID".to_owned(), Entry::Uid(0xFFFFFFFE00000000)),
                    ("SP".to_owned(), Entry::Uid(0x0000020500000000)),
                    ("TableW".to_owned(), Entry::Uid(0x0000000A00000000)),
                    ("TableZ".to_owned(), Entry::Uid(0x0000000900000000)),
                    (
                        "DataStore{n}".to_owned(),
                        Entry::UidRange(UidRange { start: 0x0000100100000000, count: 4, step: 2, start_display: 0 }),
                    ),
                ]
                .into(),
            }),
            invoking_id: Some(Table {
                entries: [
                    ("ThisSP".to_owned(), Entry::Uid(0x0000000000000001)),
                    ("SessionManager".to_owned(), Entry::Uid(0x00000000000000FF)),
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
                        ("Admin".to_owned(), Entry::Uid(0x0000020500000002)),
                        ("SPY".to_owned(), Entry::Uid(0x0000020500000001)),
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
                    Table { entries: [("ObjectD".to_owned(), Entry::Uid(0x0000000A00000001))].into() },
                ),
                (
                    "TableZ".to_owned(),
                    Table {
                        entries: [
                            ("ObjectA".to_owned(), Entry::Uid(0x0000000900000001)),
                            (
                                "ObjectB{n}".to_owned(),
                                Entry::UidRange(UidRange {
                                    start: 0x0000000900000002,
                                    count: 100,
                                    step: 2,
                                    start_display: 0,
                                }),
                            ),
                            // `step` and `display_base` are omitted in the JSON.
                            (
                                "ObjectC{n}_Suffix".to_owned(),
                                Entry::UidRange(UidRange {
                                    start: 0x0000000900001000,
                                    count: 8,
                                    step: 1,
                                    start_display: 1,
                                }),
                            ),
                        ]
                        .into(),
                    },
                ),
            ]
            .into(),
            ..Default::default()
        };
        let expected = Database {
            features: [
                ("Core".to_owned(), Feature { security_providers: [("Shared".to_owned(), shared)].into() }),
                (
                    "FeatureX".to_owned(),
                    Feature { security_providers: [("Admin".to_owned(), admin), ("SPY".to_owned(), spy)].into() },
                ),
            ]
            .into(),
        };

        let database: Database = serde_json::from_str(input).unwrap();
        assert_that!(database, eq(&expected));
    }
}
