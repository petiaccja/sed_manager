//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::HashMap;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum Object {
    Uid(#[serde(deserialize_with = "deserialize_uid")] u64),
    UidRange {
        #[serde(deserialize_with = "deserialize_uid", rename = "base")]
        start: u64,
        count: u64,
        step: u64,
        #[serde(default = "default_value_one", rename = "display_base")]
        start_display: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Table {
    #[serde(flatten)]
    pub objects: HashMap<String, Object>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SecurityProvider {
    #[serde(flatten)]
    pub tables: HashMap<String, Table>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Feature {
    #[serde(flatten)]
    pub security_providers: HashMap<String, SecurityProvider>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Database {
    #[serde(flatten)]
    pub features: HashMap<String, Feature>,
}

fn deserialize_uid<'de, D>(de: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    u64::from_str_radix(&String::deserialize(de)?, 16).map_err(|_| D::Error::custom("could not parse UID"))
}

fn default_value_one() -> u64 {
    1
}

#[cfg(test)]
mod tests {
    use googletest::{assert_that, matchers::*};

    use super::*;

    #[test]
    fn verify_loaded_content() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_database.json"));

        let expected = Database {
            features: [(
                "FeatureX".to_owned(),
                Feature {
                    security_providers: [(
                        "SPY".to_owned(),
                        SecurityProvider {
                            tables: [(
                                "TableZ".to_owned(),
                                Table {
                                    objects: [
                                        ("ObjectA".to_owned(), Object::Uid(0x0000000900000001)),
                                        (
                                            "ObjectB{n}".to_owned(),
                                            Object::UidRange {
                                                start: 0x0000000900000002,
                                                count: 100,
                                                step: 2,
                                                start_display: 0,
                                            },
                                        ),
                                    ]
                                    .into(),
                                },
                            )]
                            .into(),
                        },
                    )]
                    .into(),
                },
            )]
            .into(),
        };

        let database: Database = serde_json::from_str(input).unwrap();
        assert_that!(database, eq(&expected));
    }
}
