//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::HashMap;
use std::ops::Range;

use crate::schema_ast;

#[derive(Debug, thiserror::Error)]
pub enum SemanticError {
    #[error("the value {value:016x} (name: `{name}`) is defined for multiple UIDs or UID ranges")]
    ValueCollision { value: u64, name: String },
    #[error("the name `{name}` (value: {value:016x}) is defined for multiple UIDs or UID ranges")]
    NameCollision { name: String, value: u64 },
    #[error("the UID range `{first}` overlaps with `{second}`")]
    Overlap { first: String, second: String },
    #[error("the UID range `{0}` has an invalid name; names must be `<prefix>{{<index>}}<suffix>")]
    InvalidRangeName(String),
}

pub struct UidObject {
    pub name: String,
    pub value: u64,
}

pub struct UidRangeObject {
    pub prefix: String,
    pub suffix: String,
    pub start: u64,
    pub count: u64,
    /// The distance between consecutive UIDs, in the units of the table (see [`OBJECT_STEP_UNIT`] and [`TABLE_STEP_UNIT`]).
    pub step: u64,
    pub start_display: u64,
    /// The distance between consecutive UIDs as raw 64-bit values.
    pub stride: u64,
}

impl UidRangeObject {
    pub fn name(&self) -> String {
        format!("{}{{n}}{}", self.prefix, self.suffix)
    }

    pub fn check_contains(&self, uid_object: &UidObject) -> Result<(), SemanticError> {
        let range = self.to_range();
        if range.contains(&uid_object.value) && (uid_object.value - range.start).is_multiple_of(self.stride) {
            Err(SemanticError::Overlap { first: self.name(), second: uid_object.name.clone() })
        } else {
            Ok(())
        }
    }

    /// Returns if the two UID ranges share elements. False positives are
    /// possible when the ranges overlap, but do not actually have a common
    /// element due to their finite length.
    pub fn check_overlaps(&self, other: &UidRangeObject) -> Result<(), SemanticError> {
        let mut ranges = [self.to_range(), other.to_range()];
        ranges.sort_by_key(|range| range.start);
        let [first_range, second_range] = ranges;

        let shift = second_range.start - first_range.start;
        let stride_gcd = num_integer::gcd(self.stride, other.stride);

        if first_range.end <= second_range.start {
            Ok(())
        } else if shift.is_multiple_of(stride_gcd) {
            Err(SemanticError::Overlap { first: self.name(), second: other.name() })
        } else {
            Ok(())
        }
    }

    fn to_range(&self) -> Range<u64> {
        let end = if self.count == 0 {
            self.start
        } else {
            self.start + self.count * self.stride + 1 - self.stride
        };
        self.start..end
    }
}

/// The raw 64-bit distance corresponding to a `step` of 1 in tables whose rows
/// are objects or special-purpose UIDs.
pub const OBJECT_STEP_UNIT: u64 = 1;

/// The raw 64-bit distance corresponding to a `step` of 1 in the `TableID` table.
///
/// Table UIDs differ in their upper half, so a step of 1 means the next table.
pub const TABLE_STEP_UNIT: u64 = 1 << 32;

pub struct Table {
    pub name: String,
    pub uid_objects: Vec<UidObject>,
    pub uid_range_objects: Vec<UidRangeObject>,
}

impl Table {
    /// The `step_unit` is the raw distance between UIDs for a `step` of 1 in the
    /// table's UID ranges, either [`OBJECT_STEP_UNIT`] or [`TABLE_STEP_UNIT`].
    pub fn try_from(name: String, source: schema_ast::Table, step_unit: u64) -> Result<Self, SemanticError> {
        let (uid_objects, uid_range_objects) = convert_objects(source, step_unit)?;
        check_uniqueness(&uid_objects)?;
        for (index, uid_range_object) in uid_range_objects.iter().enumerate() {
            for uid_object in &uid_objects {
                uid_range_object.check_contains(uid_object)?;
            }
            for other_range in &uid_range_objects[index + 1..] {
                uid_range_object.check_overlaps(other_range)?;
            }
        }
        Ok(Self { name, uid_objects, uid_range_objects })
    }
}

fn convert_objects(
    source: schema_ast::Table,
    step_unit: u64,
) -> Result<(Vec<UidObject>, Vec<UidRangeObject>), SemanticError> {
    let mut uid_objects = Vec::new();
    let mut uid_range_objects = Vec::new();
    for (name, object) in source.objects {
        match object {
            schema_ast::Entry::Uid(value) => uid_objects.push(UidObject { name, value }),
            schema_ast::Entry::UidRange(schema_ast::UidRange { start, count, step, start_display }) => {
                let parts: Vec<_> = name.split(|ch| ch == '{' || ch == '}').collect();
                let (prefix, suffix) = match parts.as_slice() {
                    [prefix, _index, suffix] => ((*prefix).to_owned(), (*suffix).to_owned()),
                    _ => return Err(SemanticError::InvalidRangeName(name)),
                };
                let stride = step * step_unit;
                uid_range_objects.push(UidRangeObject { prefix, suffix, start, count, step, start_display, stride })
            }
        }
    }
    Ok((uid_objects, uid_range_objects))
}

fn check_uniqueness(uid_objects: &[UidObject]) -> Result<(), SemanticError> {
    let mut by_name = HashMap::new();
    let mut by_value = HashMap::new();
    for UidObject { name, value } in uid_objects {
        if by_name.insert(name, value).is_some() {
            return Err(SemanticError::NameCollision { value: *value, name: name.clone() });
        }
        if by_value.insert(value, name).is_some() {
            return Err(SemanticError::ValueCollision { value: *value, name: name.clone() });
        }
    }
    Ok(())
}

pub struct SecurityProvider {
    pub name: String,
    /// The `TableID` table, whose rows are the UIDs of tables.
    pub table_id: Option<Table>,
    /// Tables of special-purpose UIDs, which are not objects.
    pub special_tables: Vec<Table>,
    /// Regular tables, whose rows are objects.
    pub object_tables: Vec<Table>,
}

impl SecurityProvider {
    pub fn try_from(name: String, source: schema_ast::SecurityProvider) -> Result<Self, SemanticError> {
        let schema_ast::SecurityProvider { table_id, general, invoking_id, sm_method_id, tables } = source;
        // These names must match the `rename` attributes of `schema_ast::SecurityProvider`.
        let special_tables = [
            ("General", general),
            ("InvokingID", invoking_id),
            ("SMMethodID", sm_method_id),
        ];
        Ok(Self {
            name,
            table_id: table_id
                .map(|table| Table::try_from("TableID".to_owned(), table, TABLE_STEP_UNIT))
                .transpose()?,
            special_tables: special_tables
                .into_iter()
                .filter_map(|(name, table)| {
                    table.map(|table| Table::try_from(name.to_owned(), table, OBJECT_STEP_UNIT))
                })
                .collect::<Result<_, _>>()?,
            object_tables: tables
                .into_iter()
                .map(|(name, table)| Table::try_from(name, table, OBJECT_STEP_UNIT))
                .collect::<Result<_, _>>()?,
        })
    }
}

pub struct Feature {
    pub name: String,
    pub security_providers: Vec<SecurityProvider>,
}

impl Feature {
    pub fn try_from(name: String, source: schema_ast::Feature) -> Result<Self, SemanticError> {
        Ok(Self {
            name,
            security_providers: source
                .security_providers
                .into_iter()
                .map(|(name, security_provider)| SecurityProvider::try_from(name, security_provider))
                .collect::<Result<_, _>>()?,
        })
    }
}

pub struct Database {
    pub features: Vec<Feature>,
}

impl Database {
    pub fn try_from(source: schema_ast::Database) -> Result<Self, SemanticError> {
        Ok(Self {
            features: source
                .features
                .into_iter()
                .map(|(name, feature)| Feature::try_from(name, feature))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use googletest::prelude::*;
    use rstest::rstest;

    fn uid(name: &str, value: u64) -> UidObject {
        UidObject { name: name.to_owned(), value }
    }

    fn range(prefix: &str, start: u64, count: u64, step: u64) -> UidRangeObject {
        UidRangeObject {
            prefix: prefix.to_owned(),
            suffix: String::new(),
            start,
            count,
            step,
            start_display: 1,
            stride: step,
        }
    }

    // The range used for `check_contains` contains 0x10, 0x12, 0x14, 0x16.
    #[rstest]
    #[case::before_start(0x0F)]
    #[case::after_end(0x18)]
    #[case::off_stride(0x13)]
    fn check_contains_disjoint(#[case] value: u64) {
        let range = range("Range", 0x10, 4, 2);
        assert_that!(range.check_contains(&uid("Object", value)), ok(anything()));
    }

    #[rstest]
    #[case::first(0x10)]
    #[case::last(0x16)]
    fn check_contains_overlapping(#[case] value: u64) {
        let range = range("Range", 0x10, 4, 2);
        assert_that!(
            range.check_contains(&uid("Object", value)),
            err(matches_pattern!(SemanticError::Overlap { first: eq("Range{n}"), second: eq("Object") }))
        );
    }

    #[rstest]
    #[case::adjacent((0, 5, 2), (9, 5, 1))]
    #[case::interleaved_different_steps((0, 10, 4), (1, 10, 6))]
    fn check_overlaps_disjoint(#[case] first: (u64, u64, u64), #[case] second: (u64, u64, u64)) {
        let first = range("First", first.0, first.1, first.2);
        let second = range("Second", second.0, second.1, second.2);
        assert_that!(first.check_overlaps(&second), ok(anything()));
        assert_that!(second.check_overlaps(&first), ok(anything()));
    }

    #[rstest]
    #[case::shared_last_first((0, 5, 2), (8, 5, 2))]
    #[case::different_steps((0, 10, 4), (2, 10, 6))]
    // False positive: an empty range has no elements, but it is still flagged when it starts inside another range.
    #[case::empty_inside((5, 0, 1), (0, 10, 1))]
    fn check_overlaps_overlapping(#[case] first: (u64, u64, u64), #[case] second: (u64, u64, u64)) {
        let first = range("First", first.0, first.1, first.2);
        let second = range("Second", second.0, second.1, second.2);
        assert_that!(
            first.check_overlaps(&second),
            err(matches_pattern!(SemanticError::Overlap { first: eq("First{n}"), second: eq("Second{n}") }))
        );
        assert_that!(
            second.check_overlaps(&first),
            err(matches_pattern!(SemanticError::Overlap { first: eq("Second{n}"), second: eq("First{n}") }))
        );
    }

    #[test]
    fn check_uniqueness_unique() {
        let objects = [uid("A", 1), uid("B", 2), uid("C", 3)];
        assert_that!(check_uniqueness(&objects), ok(anything()));
    }

    #[test]
    fn check_uniqueness_name_collision() {
        let objects = [uid("A", 1), uid("B", 2), uid("A", 3)];
        assert_that!(
            check_uniqueness(&objects),
            err(matches_pattern!(SemanticError::NameCollision { name: eq("A"), value: eq(&3) }))
        );
    }

    #[test]
    fn check_uniqueness_value_collision() {
        let objects = [uid("A", 1), uid("B", 2), uid("C", 1)];
        assert_that!(
            check_uniqueness(&objects),
            err(matches_pattern!(SemanticError::ValueCollision { name: eq("C"), value: eq(&1) }))
        );
    }

    #[test]
    fn table_id_ranges_step_by_tables() {
        // Contains tables 0x1001, 0x1003, 0x1005, and 0x1007.
        let range = schema_ast::UidRange { start: 0x0000100100000000, count: 4, step: 2, start_display: 0 };
        let table = |uid: u64| schema_ast::Table {
            objects: [
                ("DataStore{n}".to_owned(), schema_ast::Entry::UidRange(range.clone())),
                ("Table".to_owned(), schema_ast::Entry::Uid(uid)),
            ]
            .into(),
        };
        assert_that!(
            Table::try_from("TableID".to_owned(), table(0x0000100400000000), TABLE_STEP_UNIT).map(|_| ()),
            ok(anything())
        );
        assert_that!(
            Table::try_from("TableID".to_owned(), table(0x0000100300000000), TABLE_STEP_UNIT).map(|_| ()),
            err(matches_pattern!(SemanticError::Overlap { first: eq("DataStore{n}"), second: eq("Table") }))
        );
    }
}
