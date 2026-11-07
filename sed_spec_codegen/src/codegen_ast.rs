//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;
use std::str::FromStr as _;

use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, PathArguments, Type, parse_quote};

use crate::semantic_ast;

pub struct Hex {
    pub value: u64,
}

impl ToTokens for Hex {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = proc_macro2::Literal::from_str(&format!("0x{:016x}_u64", self.value))
            .expect("hex literal valid by construction");
        value.to_tokens(tokens);
    }
}

/// Returns the type without generic arguments, e.g. `ObjectRef` for `ObjectRef<{ THIS_TABLE.to_u64() }>`.
///
/// Types that are not paths (e.g. references or tuples) are returned verbatim.
fn strip_generic_arguments(ty: &Type) -> Type {
    let mut ty = ty.clone();
    if let Type::Path(type_path) = &mut ty {
        for segment in &mut type_path.path.segments {
            segment.arguments = PathArguments::None;
        }
    }
    ty
}

/// A UID literal, e.g. `ObjectRef::new(0x0000000900000001_u64)`.
#[derive(Debug, Clone)]
pub struct Uid {
    /// The type of the UID, e.g. `ObjectRef<{ THIS_TABLE.to_u64() }>`.
    pub value_type: Type,
    pub value: u64,
}

impl ToTokens for Uid {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { value_type, value } = self;
        let value_type = strip_generic_arguments(value_type);
        let hex = Hex { value: *value };
        tokens.extend(quote! {
            #value_type::new(#hex)
        });
    }
}

/// A UID range literal, with an exclusive `end`.
#[derive(Debug, Clone)]
pub struct UidRange {
    /// The type of the UIDs in the range, e.g. `ObjectRef<{ THIS_TABLE.to_u64() }>`.
    pub value_type: Type,
    pub start: u64,
    pub end: u64,
    /// The distance between consecutive UIDs, in the units of the table.
    pub step: u32,
}

impl UidRange {
    pub fn new(value_type: Type, start: u64, count: u64, step: u64, stride: u64) -> Self {
        Self {
            value_type,
            start,
            end: start + count * stride,
            step: u32::try_from(step).expect("UID range step must fit into a u32"),
        }
    }
}

impl ToTokens for UidRange {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { value_type, start, end, step } = self;
        let start = Uid { value_type: value_type.clone(), value: *start };
        let end = Uid { value_type: value_type.clone(), value: *end };
        tokens.extend(quote! {
            UidRange { start: #start, end: #end, step: #step }
        });
    }
}

/// A public constant for a single UID.
#[derive(Debug)]
pub struct UidConst {
    pub ident: Ident,
    pub uid: Uid,
}

impl ToTokens for UidConst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, uid } = self;
        let value_type = &uid.value_type;
        tokens.extend(quote! {
            pub const #ident: #value_type = #uid;
        });
    }
}

/// A public constant for a UID range.
#[derive(Debug)]
pub struct UidRangeConst {
    pub ident: Ident,
    pub uid_range: UidRange,
}

impl ToTokens for UidRangeConst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, uid_range } = self;
        let value_type = &uid_range.value_type;
        tokens.extend(quote! {
            pub const #ident: UidRange<#value_type> = #uid_range;
        });
    }
}

#[derive(Debug)]
pub struct LookupByName {
    pub value_type: Type,
    pub lookup: BTreeMap<String, Ident>,
}

impl ToTokens for LookupByName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value_type = &self.value_type;
        let len = self.lookup.len();
        let items = self.lookup.iter().map(|(name, ident)| quote! { (#name, #ident) });
        tokens.extend(quote! {
            const BY_NAME: [(&str, #value_type); #len] = [
                #(#items,)*
            ];
        });
    }
}

#[derive(Debug)]
pub struct LookupByValue {
    pub value_type: Type,
    pub lookup: BTreeMap<(u64, Ident), String>,
}

impl ToTokens for LookupByValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value_type = &self.value_type;
        let len = self.lookup.len();
        let items = self.lookup.iter().map(|((_, ident), name)| quote! { (#ident, #name) });
        tokens.extend(quote! {
            const BY_VALUE: [(#value_type, &str); #len] = [
                #(#items,)*
            ];
        });
    }
}

#[derive(Debug)]
pub struct RangeLookupEntry {
    pub uid_range: UidRange,
    pub prefix: String,
    pub suffix: String,
    pub start_display: u64,
}

impl ToTokens for RangeLookupEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { uid_range, prefix, suffix, start_display } = self;
        tokens.extend(quote! {
            UidRangeEntry {
                range: #uid_range,
                prefix: #prefix,
                suffix: #suffix,
                display_base: #start_display,
            }
        });
    }
}

#[derive(Debug)]
pub struct LookupRanges {
    pub value_type: Type,
    pub lookup: Vec<RangeLookupEntry>,
}

impl ToTokens for LookupRanges {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value_type = &self.value_type;
        let len = self.lookup.len();
        let items = &self.lookup;
        tokens.extend(quote! {
            const RANGES: [UidRangeEntry<#value_type>; #len] = [
                #(#items,)*
            ];
        });
    }
}

/// The `use` declarations at the top of a table's module.
#[derive(Debug)]
pub struct Imports {
    /// Items imported from `::sed_packet`.
    pub sed_packet: Vec<Ident>,
    /// Items imported from `crate::lookup`.
    pub lookup: Vec<Ident>,
}

impl ToTokens for Imports {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { sed_packet, lookup } = self;
        tokens.extend(quote! {
            use ::sed_packet::{#(#sed_packet),*};
            use crate::lookup::{#(#lookup),*};
        });
    }
}

/// The constant that refers to the UID of the table itself.
#[derive(Debug)]
pub struct ThisTable {
    /// The identifier of the table's constant in the `TableID` table.
    pub table: Ident,
}

impl ToTokens for ThisTable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let table = &self.table;
        tokens.extend(quote! {
            const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::#table;
        });
    }
}

/// The public `LOOKUP` constant of a table.
#[derive(Debug)]
pub struct LookupConst {
    /// The type of the constant, e.g. `ObjectTableLookup<'static, { THIS_TABLE.to_u64() }>`.
    pub lookup_type: Type,
    /// The value of the `name` field, if the lookup has one.
    pub name: Option<String>,
    /// Whether the lookup has a `ranges` field.
    pub has_ranges: bool,
}

impl ToTokens for LookupConst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { lookup_type, name, has_ranges } = self;
        let constructor = strip_generic_arguments(lookup_type);
        let name = name.as_ref().map(|name| quote! { name: #name, });
        let ranges = has_ranges.then(|| quote! { ranges: &RANGES, });
        tokens.extend(quote! {
            pub const LOOKUP: #lookup_type = #constructor {
                #name
                by_name: SortedMap { items: &BY_NAME },
                by_uid: SortedMap { items: &BY_VALUE },
                #ranges
            };
        });
    }
}

/// The module of a table, containing its UID constants and lookup.
#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub imports: Imports,
    pub this_table: Option<ThisTable>,
    pub uids: Vec<UidConst>,
    pub uid_ranges: Vec<UidRangeConst>,
    pub lookup_by_name: LookupByName,
    pub lookup_by_value: LookupByValue,
    pub lookup_ranges: LookupRanges,
    pub lookup: LookupConst,
}

impl Table {
    /// A regular table, whose rows are objects.
    pub fn object(source: semantic_ast::Table) -> Self {
        let this_table = ThisTable { table: ident_upper(&source.name) };
        let lookup = LookupConst {
            lookup_type: parse_quote! { ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> },
            name: Some(source.name.clone()),
            has_ranges: true,
        };
        Self::new(
            source,
            parse_quote! { ObjectRef<{ THIS_TABLE.to_u64() }> },
            Imports {
                sed_packet: vec![
                    parse_quote!(ObjectRef),
                    parse_quote!(TableRef),
                    parse_quote!(UidRange),
                ],
                lookup: vec![
                    parse_quote!(UidRangeEntry),
                    parse_quote!(SortedMap),
                    parse_quote!(ObjectTableLookup),
                ],
            },
            Some(this_table),
            lookup,
        )
    }

    /// The `TableID` table, whose rows are the UIDs of tables.
    pub fn table_id(source: semantic_ast::Table) -> Self {
        let lookup = LookupConst { lookup_type: parse_quote! { TableIdLookup<'static> }, name: None, has_ranges: true };
        Self::new(
            source,
            parse_quote! { TableRef },
            Imports {
                sed_packet: vec![parse_quote!(TableRef), parse_quote!(UidRange)],
                lookup: vec![
                    parse_quote!(UidRangeEntry),
                    parse_quote!(SortedMap),
                    parse_quote!(TableIdLookup),
                ],
            },
            None,
            lookup,
        )
    }

    /// A table of special-purpose UIDs, which are not objects.
    pub fn special(source: semantic_ast::Table) -> Self {
        let this_table = ThisTable { table: ident_upper(&source.name) };
        let lookup = LookupConst {
            lookup_type: parse_quote! { MetaTableLookup<'static> },
            name: Some(source.name.clone()),
            has_ranges: false,
        };
        Self::new(
            source,
            parse_quote! { Uid },
            Imports {
                sed_packet: vec![
                    parse_quote!(TableRef),
                    parse_quote!(Uid),
                    parse_quote!(UidRange),
                ],
                lookup: vec![
                    parse_quote!(UidRangeEntry),
                    parse_quote!(SortedMap),
                    parse_quote!(MetaTableLookup),
                ],
            },
            Some(this_table),
            lookup,
        )
    }

    /// Builds the UID constants and lookups, which are the same for all kinds of tables
    /// except for the types of the UIDs.
    fn new(
        source: semantic_ast::Table,
        value_type: Type,
        imports: Imports,
        this_table: Option<ThisTable>,
        lookup: LookupConst,
    ) -> Self {
        let mut uids = Vec::new();
        let mut lookup_by_name = LookupByName { value_type: value_type.clone(), lookup: BTreeMap::new() };
        let mut lookup_by_value = LookupByValue { value_type: value_type.clone(), lookup: BTreeMap::new() };
        for semantic_ast::UidObject { name, value } in source.uid_objects {
            let ident = ident_upper(&name);
            let uid = Uid { value_type: value_type.clone(), value };
            uids.push(UidConst { ident: ident.clone(), uid });
            lookup_by_name.lookup.insert(name.clone(), ident.clone());
            lookup_by_value.lookup.insert((value, ident), name);
        }

        let mut uid_ranges = Vec::new();
        let mut lookup_ranges = LookupRanges { value_type: value_type.clone(), lookup: Vec::new() };
        for semantic_ast::UidRangeObject { prefix, suffix, start, count, step, start_display, stride } in
            source.uid_range_objects
        {
            let ident = ident_upper(&format!("{prefix}_{suffix}"));
            let uid_range = UidRange::new(value_type.clone(), start, count, step, stride);
            uid_ranges.push(UidRangeConst { ident, uid_range: uid_range.clone() });
            lookup_ranges.lookup.push(RangeLookupEntry { uid_range, prefix, suffix, start_display });
        }

        Self {
            name: source.name,
            imports,
            this_table,
            uids,
            uid_ranges,
            lookup_by_name,
            lookup_by_value,
            lookup_ranges,
            lookup,
        }
    }

    pub fn ident_lower(&self) -> Ident {
        format_ident!("{}", self.name.to_snake_case())
    }

    pub fn ident_upper(&self) -> Ident {
        ident_upper(&self.name)
    }
}

fn ident_upper(name: &str) -> Ident {
    format_ident!("{}", name.to_shouty_snake_case())
}

impl ToTokens for Table {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name: _,
            imports,
            this_table,
            uids,
            uid_ranges,
            lookup_by_name,
            lookup_by_value,
            lookup_ranges,
            lookup,
        } = self;
        let ident_lower = self.ident_lower();
        tokens.extend(quote! {
            pub mod #ident_lower {
                #imports

                #this_table

                #(#uids)*
                #(#uid_ranges)*

                #lookup_by_name
                #lookup_by_value
                #lookup_ranges

                #lookup
            }
        });
    }
}

pub struct SecurityProvider {
    pub name: String,
    pub table_id: Option<Table>,
    pub special_tables: Vec<Table>,
    pub object_tables: Vec<Table>,
}

impl From<semantic_ast::SecurityProvider> for SecurityProvider {
    fn from(value: semantic_ast::SecurityProvider) -> Self {
        Self {
            name: value.name,
            table_id: value.table_id.map(Table::table_id),
            special_tables: value.special_tables.into_iter().map(Table::special).collect(),
            object_tables: value.object_tables.into_iter().map(Table::object).collect(),
        }
    }
}

impl ToTokens for SecurityProvider {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, table_id, special_tables, object_tables } = self;
        let ident_lower = format_ident!("{}", name.to_snake_case());

        // The `TableID` table is referenced directly through `table_ids`, it's not a lookup table.
        fn lookup_item(table: &Table) -> TokenStream {
            let ident_lower = table.ident_lower();
            let ident_upper = table.ident_upper();
            quote! { (super::super::core::shared::table_id::#ident_upper, &#ident_lower::LOOKUP) }
        }
        let lookup_items: Vec<_> =
            special_tables.iter().map(lookup_item).chain(object_tables.iter().map(lookup_item)).collect();
        let num_tables = lookup_items.len();

        tokens.extend(quote! {
            pub mod #ident_lower {
                use ::sed_packet::{TableRef};
                use crate::lookup::{TableLookup, SecurityProviderLookup, SortedMap};

                #table_id
                #(#special_tables)*
                #(#object_tables)*

                const TABLE_LOOKUPS: [(TableRef, &dyn TableLookup); #num_tables] = [
                    #(#lookup_items,)*
                ];

                pub const LOOKUP: SecurityProviderLookup<'static> = SecurityProviderLookup {
                    name: #name,
                    table_ids: &super::super::core::shared::table_id::LOOKUP,
                    table_lookups: SortedMap { items: &TABLE_LOOKUPS },
                };
            }
        });
    }
}

pub struct Feature {
    pub name: String,
    pub security_providers: Vec<SecurityProvider>,
}

impl From<semantic_ast::Feature> for Feature {
    fn from(value: semantic_ast::Feature) -> Self {
        Self { name: value.name, security_providers: value.security_providers.into_iter().map(Into::into).collect() }
    }
}

impl ToTokens for Feature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, security_providers } = self;
        let ident_lower = format_ident!("{}", name.to_snake_case());
        let num_security_providers = security_providers.len();
        let lookup_items = security_providers.iter().map(|security_provider| {
            let ident_lower = format_ident!("{}", security_provider.name.to_snake_case());
            let ident_upper = format_ident!("{}", security_provider.name.to_snake_case());
            quote! { (admin::sp::#ident_upper, &#ident_lower::LOOKUP) }
        });

        tokens.extend(quote! {
            pub mod #ident_lower {
                use ::sed_packet::{TableRef};
                use crate::lookup::{SecurityProviderLookup, FeatureLookup, SortedMap};

                #(#security_providers)*

                const SP_LOOKUPS: [(Option<SpRef>, &SecurityProviderLookup); #num_security_providers] = [
                    #(#lookup_items,)*
                ];

                pub const LOOKUP: FeatureLookup<'static> = FeatureLookup {
                    sp_lookups: SortedMap { items: &SP_LOOKUPS },
                };
            }
        });
    }
}

pub struct Database {
    pub features: Vec<Feature>,
}

impl From<semantic_ast::Database> for Database {
    fn from(value: semantic_ast::Database) -> Self {
        Self { features: value.features.into_iter().map(Into::into).collect() }
    }
}

impl ToTokens for Database {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! { const GENERATED_MARKER: () = (); });
        for feature in &self.features {
            feature.to_tokens(tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::const_generic(parse_quote! { ObjectRef<{ THIS_TABLE.to_u64() }> }, parse_quote! { ObjectRef })]
    #[case::lifetime_and_const(parse_quote! { ObjectTableLookup<'static, 9> }, parse_quote! { ObjectTableLookup })]
    #[case::nested_path(parse_quote! { a::B<u8>::C<u16> }, parse_quote! { a::B::C })]
    #[case::not_generic(parse_quote! { TableRef }, parse_quote! { TableRef })]
    #[case::not_a_path(parse_quote! { &'static Uid }, parse_quote! { &'static Uid })]
    fn strip_generic_arguments_cases(#[case] ty: Type, #[case] expected: Type) {
        assert_that!(strip_generic_arguments(&ty), eq(&expected));
    }
}
