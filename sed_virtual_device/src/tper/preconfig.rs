//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_spec::{
    ssc::Ssc,
    uid_registry::{core, data_store, enterprise, kpio, opal_2, opalite, psid, pyrite_2, ruby},
};
use serde_json::{Map, Value};

#[derive(Debug, thiserror::Error)]
pub enum NameResolutionError {
    #[error("the path `{path}` could not be resolved")]
    PathNotFound { path: String },
}

/// Resolves paths to UIDs in the JSON document.
///
/// Each path is prefixed with a `$`, like `"$SomeTable::SomeObject"`. The names
/// are replaced by the corresponding UID, e.g. `"0x0000_0001_0000_0001"`
pub fn resolve_paths(value: Value, ssc: Ssc) -> Result<Value, NameResolutionError> {
    Ok(match value {
        Value::String(raw) => Value::String(resolve_if_path(raw, ssc)?),
        Value::Array(items) => {
            Value::Array(items.into_iter().map(|item| resolve_paths(item, ssc)).collect::<Result<Vec<_>, _>>()?)
        }
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, value)| Ok((resolve_if_path(key, ssc)?, resolve_paths(value, ssc)?)))
                .collect::<Result<Map<_, _>, _>>()?,
        ),
        Value::Null | Value::Bool(_) | Value::Number(_) => value,
    })
}

/// Resolves the string if it's a path (i.e. starts with `$`), otherwise returns it unchanged.
fn resolve_if_path(raw: String, ssc: Ssc) -> Result<String, NameResolutionError> {
    match raw.strip_prefix('$') {
        Some(path) => resolve(path, ssc),
        None => Ok(raw),
    }
}

fn resolve(path: &str, ssc: Ssc) -> Result<String, NameResolutionError> {
    let ssc_lookup = match ssc {
        Ssc::Enterprise => enterprise::LOOKUP,
        Ssc::OpalV1 => opal_2::LOOKUP, // Checked and aligns with Opal 2.
        Ssc::OpalV2 => opal_2::LOOKUP,
        Ssc::Opalite => opalite::LOOKUP,
        Ssc::PyriteV1 => pyrite_2::LOOKUP, // Checked and aligns with Pyrite 2.
        Ssc::PyriteV2 => pyrite_2::LOOKUP,
        Ssc::Ruby => ruby::LOOKUP,
        Ssc::KeyPerIO => kpio::LOOKUP,
    };
    // Order matters: the SSC first, then the features, and finally core.
    let lookups = [ssc_lookup, data_store::LOOKUP, psid::LOOKUP, core::LOOKUP];
    for lookup in lookups {
        if let Some(uid) = lookup.by_path(path) {
            return Ok(uid.to_string());
        }
    }
    Err(NameResolutionError::PathNotFound { path: path.to_owned() })
}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use rstest::rstest;
    use serde_json::json;

    use sed_packet::Uid;
    use sed_spec::uid_registry::{self, data_store, opal_2, psid};

    use super::{NameResolutionError, Ssc, resolve_paths};

    #[rstest]
    #[case::ssc("$Admin::C_PIN::SID", opal_2::admin::c_pin::SID.to_uid())]
    #[case::feature_psid("$Admin::Authority::PSID", psid::admin::authority::PSID.to_uid())]
    #[case::feature_data_store(
        "$Locking::ACE::DataStore2_Get_All",
        data_store::locking::ace::DATA_STORE_GET_ALL.get(0).unwrap().to_uid()
    )]
    #[case::ssc_table("$Locking::Table::DataStore", opal_2::locking::table::DATA_STORE.to_uid())]
    #[case::core("$Authority::Anybody", uid_registry::core::shared::authority::ANYBODY.to_uid())]
    fn resolve_string_value(#[case] path: &str, #[case] uid: Uid) {
        let resolved = resolve_paths(json!(path), Ssc::OpalV2).unwrap();
        assert_that!(resolved, eq(&json!(uid.to_string())));
    }

    #[test]
    fn leave_non_paths_unchanged() {
        let value = json!({
            "string": "C_PIN::SID",
            "number": 42,
            "bool": true,
            "null": null,
            "array": ["Authority::Anybody", 1, false],
        });
        let resolved = resolve_paths(value.clone(), Ssc::OpalV2).unwrap();
        assert_that!(resolved, eq(&value));
    }

    #[test]
    fn resolve_nested() {
        let sid = opal_2::admin::c_pin::SID.to_uid().to_string();
        let anybody = uid_registry::core::shared::authority::ANYBODY.to_uid().to_string();
        let value = json!({
            "c_pin": {
                "$Admin::C_PIN::SID": {
                    "uid": "$Admin::C_PIN::SID",
                    "authorities": ["$Authority::Anybody", "Authority::Anybody"],
                },
            },
        });
        let expected = json!({
            "c_pin": {
                sid.clone(): {
                    "uid": sid,
                    "authorities": [anybody, "Authority::Anybody"],
                },
            },
        });
        let resolved = resolve_paths(value, Ssc::OpalV2).unwrap();
        assert_that!(resolved, eq(&expected));
    }

    #[rstest]
    #[case::unknown_object("$Admin::C_PIN::NoSuchObject")]
    #[case::missing_sp("$C_PIN::SID")]
    #[case::unknown_table("$NoSuchTable::SID")]
    #[case::wrong_ssc("$Locking::LockingInfo::LockingInfo")]
    fn unresolved_path_value(#[case] path: &str) {
        let result = resolve_paths(json!([path]), Ssc::Enterprise);
        assert_that!(result, err(matches_pattern!(NameResolutionError::PathNotFound { path: eq(&path[1..]) })));
    }

    #[test]
    fn unresolved_path_key() {
        let result = resolve_paths(json!({ "$Admin::C_PIN::NoSuchObject": 0 }), Ssc::OpalV2);
        assert_that!(
            result,
            err(matches_pattern!(NameResolutionError::PathNotFound { path: eq("Admin::C_PIN::NoSuchObject") }))
        );
    }
}
