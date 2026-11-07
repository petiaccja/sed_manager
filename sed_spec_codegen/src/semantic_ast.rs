use std::{collections::HashMap, ops::Range};

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
    pub step: u64,
    pub start_display: u64,
}

impl UidRangeObject {
    pub fn name(&self) -> String {
        format!("{}{{n}}{}", self.prefix, self.suffix)
    }

    pub fn check_contains(&self, uid_object: &UidObject) -> Result<(), SemanticError> {
        let range = self.to_range();
        if range.contains(&uid_object.value) && (uid_object.value - range.start).is_multiple_of(self.step) {
            Err(SemanticError::Overlap { first: self.name(), second: uid_object.name.clone() })
        } else {
            Ok(())
        }
    }

    pub fn check_overlaps(&self, other: &UidRangeObject) -> Result<(), SemanticError> {
        let mut ranges = [self.to_range(), other.to_range()];
        ranges.sort_by_key(|range| range.start);
        let [first_range, second_range] = ranges;

        let shift = second_range.start - first_range.start;
        let step_gcd = num_integer::gcd(self.step, other.step);

        if first_range.end <= second_range.start {
            Ok(())
        } else if shift.is_multiple_of(step_gcd) {
            Err(SemanticError::Overlap { first: self.name(), second: other.name() })
        } else {
            Ok(())
        }
    }

    fn to_range(&self) -> Range<u64> {
        self.start..(self.start + self.count * self.step + 1 - self.step)
    }
}

pub struct Table {
    pub name: String,
    pub uid_objects: Vec<UidObject>,
    pub uid_range_objects: Vec<UidRangeObject>,
}

impl Table {
    pub fn try_from(name: String, source: schema_ast::Table) -> Result<Self, SemanticError> {
        let (uid_objects, uid_range_objects) = convert_objects(source)?;
        check_uniqueness(&uid_objects)?;
        for uid_range_object in &uid_range_objects {
            for uid_object in &uid_objects {
                uid_range_object.check_contains(uid_object)?;
            }
            for other_range in &uid_range_objects {
                uid_range_object.check_overlaps(other_range)?;
            }
        }
        Ok(Self { name, uid_objects, uid_range_objects })
    }
}

fn convert_objects(source: schema_ast::Table) -> Result<(Vec<UidObject>, Vec<UidRangeObject>), SemanticError> {
    let mut uid_objects = Vec::new();
    let mut uid_range_objects = Vec::new();
    for (name, object) in source.objects {
        match object {
            schema_ast::Object::Uid(value) => uid_objects.push(UidObject { name, value }),
            schema_ast::Object::UidRange { start, count, step, start_display } => {
                let parts: Vec<_> = name.split(|ch| ch == '{' || ch == '}').collect();
                let (prefix, suffix) = match parts.as_slice() {
                    [prefix, _index, suffix] => ((*prefix).to_owned(), (*suffix).to_owned()),
                    _ => return Err(SemanticError::InvalidRangeName(name)),
                };

                uid_range_objects.push(UidRangeObject { prefix, suffix, start, count, step, start_display })
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
    pub tables: Vec<Table>,
}

impl SecurityProvider {
    pub fn try_from(name: String, source: schema_ast::SecurityProvider) -> Result<Self, SemanticError> {
        Ok(Self {
            name,
            tables: source
                .tables
                .into_iter()
                .map(|(name, table)| Table::try_from(name, table))
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
