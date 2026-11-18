//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use heck::ToSnakeCase as _;

use crate::semantic_ast::{
    Feature, FeatureKind, SecurityProvider, SemanticError, Table, UidRangeEntry, UidRegistry, check_overlaps,
    check_uniqueness,
};

/// The name of the generated feature that contains the UIDs common to SSCs.
pub const COMMON_FEATURE: &str = "Common";

/// Add the ["Common"](COMMON_FEATURE) to the registry, which collects the UIDs that
/// are identical in all SSCs that define them.
///
/// A UID is included when:
/// - It's defined by at least 2 SSCs.
/// - It has the same SP (by name), table (by name & UID), name, and value in all SSCs.
pub fn add_common_feature(mut registry: UidRegistry) -> Result<UidRegistry, SemanticError> {
    let reserved_module = COMMON_FEATURE.to_snake_case();
    if let Some(feature) = registry.features.iter().find(|feature| feature.name.to_snake_case() == reserved_module) {
        return Err(SemanticError::ReservedFeatureName(feature.name.clone()));
    }

    let sscs = registry.features.iter().filter(|feature| feature.kind == FeatureKind::Ssc);
    let security_providers = group_by_name(sscs.flat_map(|ssc| &ssc.security_providers), |sp| sp.name.clone())
        .into_iter()
        .filter_map(|(name, definitions)| common_security_provider_subset(name, &definitions).transpose())
        .collect::<Result<_, _>>()?;

    registry
        .features
        .push(Feature { name: COMMON_FEATURE.to_owned(), kind: FeatureKind::Common, security_providers });
    Ok(registry)
}

/// Returns the common subset of the `definitions` of the same SP by different
/// SSCs, or `None` if they have nothing in common.
fn common_security_provider_subset(
    name: String,
    definitions: &[&SecurityProvider],
) -> Result<Option<SecurityProvider>, SemanticError> {
    let uid = common_value(definitions.iter().map(|sp| &sp.uid)).flatten();
    let table_id = common_table_subset(&definitions.iter().filter_map(|sp| sp.table_id.as_ref()).collect::<Vec<_>>())?;
    let special_tables = common_table_list_subset(definitions.iter().flat_map(|sp| &sp.special_tables))?;
    let object_tables = common_table_list_subset(definitions.iter().flat_map(|sp| &sp.object_tables))?;

    if table_id.is_none() && special_tables.is_empty() && object_tables.is_empty() {
        return Ok(None);
    }
    Ok(Some(SecurityProvider { name, uid, table_id, special_tables, object_tables }))
}

/// Groups the `tables` by name, and returns the common subset of each group.
fn common_table_list_subset<'a>(tables: impl IntoIterator<Item = &'a Table>) -> Result<Vec<Table>, SemanticError> {
    group_by_name(tables, |table| table.name.clone())
        .into_values()
        .filter_map(|definitions| common_table_subset(&definitions).transpose())
        .collect()
}

/// Returns the common subset of the `definitions` of the same table by
/// different SSCs, or `None` if they have nothing in common.
fn common_table_subset(definitions: &[&Table]) -> Result<Option<Table>, SemanticError> {
    let Some(first) = definitions.first() else {
        return Ok(None);
    };
    let Some(uid) = common_value(definitions.iter().map(|table| &table.uid)) else {
        return Ok(None);
    };

    let uid_entries: Vec<_> =
        group_by_name(definitions.iter().flat_map(|table| &table.uid_entries), |entry| entry.name.clone())
            .into_values()
            .filter_map(common_value)
            .collect();
    let uid_range_entries: Vec<_> =
        group_by_name(definitions.iter().flat_map(|table| &table.uid_range_entries), UidRangeEntry::name)
            .into_values()
            .filter_map(common_value)
            .collect();

    if uid_entries.is_empty() && uid_range_entries.is_empty() {
        return Ok(None);
    }
    // The entries are valid within each SSC, but entries that come from
    // different SSCs may still collide.
    check_uniqueness(&uid_entries)?;
    check_overlaps(&uid_entries, &uid_range_entries)?;
    Ok(Some(Table { name: first.name.clone(), uid, uid_entries, uid_range_entries }))
}

/// Returns the value if there are at least two `definitions`, and they are all
/// equal.
fn common_value<'a, T: PartialEq + Clone + 'a>(definitions: impl IntoIterator<Item = &'a T>) -> Option<T> {
    let mut definitions = definitions.into_iter();
    let first = definitions.next()?;
    let mut count = 1;
    for definition in definitions {
        if definition != first {
            return None;
        }
        count += 1;
    }
    (count >= 2).then(|| first.clone())
}

fn group_by_name<'a, T>(
    items: impl IntoIterator<Item = &'a T>,
    name: impl Fn(&T) -> String,
) -> BTreeMap<String, Vec<&'a T>> {
    let mut groups = BTreeMap::<_, Vec<_>>::new();
    for item in items {
        groups.entry(name(item)).or_default().push(item);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    use googletest::prelude::*;

    use crate::schema_ast;

    const CORE: &str = r#""Core": { "kind": "core", "Shared": { "TableID": {
        "SP": "0000020500000000",
        "TableA": "0000000900000000"
    } } }"#;

    /// A feature with a `Locking` SP of the given UID, whose SP contains the
    /// given `tables` JSON.
    fn feature(name: &str, kind: &str, locking_uid: &str, tables: &str) -> String {
        format!(
            r#""{name}": {{
                "kind": "{kind}",
                "Admin": {{ "SP": {{ "Admin": "0000020500000001", "Locking": "{locking_uid}" }} }},
                "Locking": {{ {tables} }}
            }}"#
        )
    }

    fn ssc(name: &str, tables: &str) -> String {
        feature(name, "ssc", "0000020500000002", tables)
    }

    fn generate(features: &[String]) -> std::result::Result<UidRegistry, SemanticError> {
        let json = format!("{{ {CORE}, {} }}", features.join(", "));
        let registry: schema_ast::UidRegistry = serde_json::from_str(&json).unwrap();
        add_common_feature(UidRegistry::try_from(registry).unwrap())
    }

    fn common(features: &[String]) -> Feature {
        let registry = generate(features).unwrap();
        registry.features.into_iter().find(|feature| feature.name == COMMON_FEATURE).unwrap()
    }

    /// The `SP::Table::Entry` paths of all UIDs in the feature.
    fn paths(feature: &Feature) -> Vec<String> {
        let mut paths = Vec::new();
        for sp in &feature.security_providers {
            for table in sp.table_id.iter().chain(&sp.special_tables).chain(&sp.object_tables) {
                let names = table.uid_entries.iter().map(|entry| entry.name.clone());
                let range_names = table.uid_range_entries.iter().map(UidRangeEntry::name);
                for name in names.chain(range_names) {
                    paths.push(format!("{}::{}::{}", sp.name, table.name, name));
                }
            }
        }
        paths
    }

    fn security_provider<'a>(feature: &'a Feature, name: &str) -> Option<&'a SecurityProvider> {
        feature.security_providers.iter().find(|sp| sp.name == name)
    }

    #[test]
    fn identical_entries_are_common() {
        let tables = r#"
            "TableID": { "Own": "0000100100000000" },
            "Own": { "OwnObject": "0000100100000001" },
            "TableA": {
                "Object": "0000000900000001",
                "Range{n}": { "start": "0000000900000010", "count": 4 }
            }
        "#;
        let common = common(&[ssc("X", tables), ssc("Y", tables)]);
        assert_that!(
            paths(&common),
            unordered_elements_are![
                eq("Admin::SP::Admin"),
                eq("Admin::SP::Locking"),
                eq("Locking::TableID::Own"),
                eq("Locking::Own::OwnObject"),
                eq("Locking::TableA::Object"),
                eq("Locking::TableA::Range{n}"),
            ]
        );
        assert_that!(
            security_provider(&common, "Locking"),
            some(field!(SecurityProvider.uid, some(eq(&0x0000020500000002))))
        );
    }

    #[test]
    fn entries_of_a_single_ssc_are_not_common() {
        let common = common(&[
            ssc("X", r#""TableA": { "Object": "0000000900000001", "OnlyX": "0000000900000002" }"#),
            ssc("Y", r#""TableA": { "Object": "0000000900000001" }"#),
        ]);
        assert_that!(paths(&common), not(contains(eq("Locking::TableA::OnlyX"))));
        assert_that!(paths(&common), contains(eq("Locking::TableA::Object")));
    }

    #[test]
    fn differing_entries_are_not_common() {
        let common = common(&[
            ssc(
                "X",
                r#""TableA": {
                    "Object": "0000000900000001",
                    "Range{n}": { "start": "0000000900000010", "count": 4 },
                    "Same": "0000000900000003"
                }"#,
            ),
            ssc(
                "Y",
                r#""TableA": {
                    "Object": "0000000900000002",
                    "Range{n}": { "start": "0000000900000010", "count": 5 },
                    "Same": "0000000900000003"
                }"#,
            ),
            ssc("Z", r#""TableA": { "Object": "0000000900000001" }"#),
        ]);
        assert_that!(paths(&common), not(contains(eq("Locking::TableA::Object"))));
        assert_that!(paths(&common), not(contains(eq("Locking::TableA::Range{n}"))));
        assert_that!(paths(&common), contains(eq("Locking::TableA::Same")));
    }

    #[test]
    fn non_ssc_features_are_ignored() {
        let tables = r#""TableA": { "Object": "0000000900000001" }"#;
        let common = common(&[
            ssc("X", tables),
            feature("F", "feature", "0000020500000002", tables),
        ]);
        assert_that!(paths(&common), is_empty());
    }

    #[test]
    fn tables_with_differing_uids_are_not_common() {
        let common = common(&[
            ssc("X", r#""TableID": { "Own": "0000100100000000" }, "Own": { "Object": "0000000900000001" }"#),
            ssc("Y", r#""TableID": { "Own": "0000800100000000" }, "Own": { "Object": "0000000900000001" }"#),
        ]);
        assert_that!(paths(&common), not(contains(eq("Locking::Own::Object"))));
        assert_that!(paths(&common), not(contains(eq("Locking::TableID::Own"))));
    }

    #[test]
    fn security_provider_uid_is_dropped_when_differing() {
        let tables = r#""TableA": { "Object": "0000000900000001" }"#;
        let common = common(&[
            feature("X", "ssc", "0000020500000002", tables),
            feature("Y", "ssc", "0000020500010001", tables),
        ]);
        assert_that!(security_provider(&common, "Locking"), some(field!(SecurityProvider.uid, none())));
        assert_that!(paths(&common), contains(eq("Locking::TableA::Object")));
    }

    #[test]
    fn empty_security_providers_are_omitted() {
        let common = common(&[
            ssc("X", r#""TableA": { "Object": "0000000900000001" }"#),
            ssc("Y", r#""TableA": { "Object": "0000000900000002" }"#),
        ]);
        assert_that!(security_provider(&common, "Locking"), none());
    }

    #[test]
    fn colliding_entries_from_different_sscs() {
        // Each SSC is valid, but `Object` from X and Y falls inside `Range{n}`
        // from Z and W.
        let with_object = r#""TableA": { "Object": "0000000900000011" }"#;
        let with_range = r#""TableA": { "Range{n}": { "start": "0000000900000010", "count": 4 } }"#;
        let result = generate(&[
            ssc("X", with_object),
            ssc("Y", with_object),
            ssc("Z", with_range),
            ssc("W", with_range),
        ]);
        assert_that!(
            result,
            err(matches_pattern!(SemanticError::Overlap { first: eq("Range{n}"), second: eq("Object") }))
        );
    }

    #[test]
    fn reserved_feature_name() {
        let tables = r#""TableA": {}"#;
        assert_that!(
            generate(&[ssc("COMMON", tables)]),
            err(matches_pattern!(SemanticError::ReservedFeatureName(eq("COMMON"))))
        );
    }
}
