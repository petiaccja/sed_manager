//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, LitInt, PathArguments, Type, parse_quote};

use crate::semantic_ast;

/// The identifier for a module, e.g. `table_id` for `TableID`.
fn module_ident(name: &str) -> Ident {
    format_ident!("{}", name.to_snake_case())
}

/// The identifier for a constant, e.g. `TABLE_ID` for `TableID`.
fn const_ident(name: &str) -> Ident {
    format_ident!("{}", name.to_shouty_snake_case())
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
    /// The exact type of the UID, e.g. `ObjectRef<...>`.
    pub ty: Type,
    pub value: u64,
}

impl ToTokens for Uid {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ty: uid_type, value } = self;
        let ty = strip_generic_arguments(uid_type);
        let hex = LitInt::new(&format!("0x{value:016x}_u64"), Span::call_site());
        tokens.extend(quote! {
            #ty::new(#hex)
        });
    }
}

/// A UID range literal, with an exclusive `end`.
#[derive(Debug, Clone)]
pub struct UidRange {
    /// The exact type of the UIDs in the range, e.g. `ObjectRef<...>`.
    pub uid_ty: Type,
    pub start: u64,
    pub end: u64,
    /// The distance between consecutive UIDs, in the units of the table.
    pub step: u32,
}

impl UidRange {
    pub fn new(uid_ty: Type, range: &semantic_ast::UidRangeEntry) -> Self {
        Self { uid_ty, start: range.start, end: range.start + range.count * range.stride, step: range.step }
    }
}

impl ToTokens for UidRange {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { uid_ty, start, end, step } = self;
        let start = Uid { ty: uid_ty.clone(), value: *start };
        let end = Uid { ty: uid_ty.clone(), value: *end };
        tokens.extend(quote! {
            UidRange { start: #start, end: #end, step: #step }
        });
    }
}

/// A public constant for a single UID.
#[derive(Debug)]
pub struct UidConstant {
    pub ident: Ident,
    pub uid: Uid,
}

impl ToTokens for UidConstant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, uid } = self;
        let uid_ty = &uid.ty;
        tokens.extend(quote! {
            pub const #ident: #uid_ty = #uid;
        });
    }
}

/// A public constant for a UID range.
#[derive(Debug)]
pub struct UidRangeConstant {
    pub ident: Ident,
    pub uid_range: UidRange,
}

impl ToTokens for UidRangeConstant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, uid_range } = self;
        let uid_ty = &uid_range.uid_ty;
        tokens.extend(quote! {
            pub const #ident: UidRange<#uid_ty> = #uid_range;
        });
    }
}

#[derive(Debug)]
pub struct LookupByName {
    pub uid_ty: Type,
    pub lookup: BTreeMap<String, Ident>,
}

impl ToTokens for LookupByName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let uid_ty = &self.uid_ty;
        let len = self.lookup.len();
        let items = self.lookup.iter().map(|(name, ident)| quote! { (#name, #ident) });
        tokens.extend(quote! {
            const BY_NAME: [(&str, #uid_ty); #len] = [
                #(#items,)*
            ];
        });
    }
}

#[derive(Debug)]
pub struct LookupByUid {
    pub uid_ty: Type,
    pub lookup: BTreeMap<u64, (Ident, String)>,
}

impl ToTokens for LookupByUid {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let uid_ty = &self.uid_ty;
        let len = self.lookup.len();
        let items = self.lookup.values().map(|(ident, name)| quote! { (#ident, #name) });
        tokens.extend(quote! {
            const BY_UID: [(#uid_ty, &str); #len] = [
                #(#items,)*
            ];
        });
    }
}

#[derive(Debug)]
pub struct UidRangeLookupEntry {
    pub uid_range: UidRange,
    pub prefix: String,
    pub suffix: String,
    pub display_start: u64,
}

impl ToTokens for UidRangeLookupEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { uid_range, prefix, suffix, display_start } = self;
        tokens.extend(quote! {
            UidRangeLookupEntry {
                range: #uid_range,
                prefix: #prefix,
                suffix: #suffix,
                display_start: #display_start,
            }
        });
    }
}

#[derive(Debug)]
pub struct LookupRanges {
    pub uid_ty: Type,
    pub lookup: Vec<UidRangeLookupEntry>,
}

impl ToTokens for LookupRanges {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let uid_ty = &self.uid_ty;
        let len = self.lookup.len();
        let items = &self.lookup;
        tokens.extend(quote! {
            const RANGES: [UidRangeLookupEntry<#uid_ty>; #len] = [
                #(#items,)*
            ];
        });
    }
}

/// The public `LOOKUP` constant of a table.
#[derive(Debug)]
pub struct LookupConstant {
    /// The type of the constant, e.g. `RefTableLookup<'static, ObjectRef<{ THIS_TABLE.to_u64() }>>`.
    pub lookup_type: Type,
    /// The value of the `name` field, if the lookup has one.
    pub name: Option<String>,
}

impl ToTokens for LookupConstant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { lookup_type, name } = self;
        let constructor = strip_generic_arguments(lookup_type);
        let name = name.as_ref().map(|name| quote! { name: #name, });
        tokens.extend(quote! {
            pub const LOOKUP: #lookup_type = #constructor {
                #name
                by_name: SortedMap { items: &BY_NAME },
                by_uid: SortedMap { items: &BY_UID },
                ranges: &RANGES,
            };
        });
    }
}

/// The module of a table, containing its UID constants and lookup.
#[derive(Debug)]
pub struct Table {
    pub name: String,
    /// The `THIS_TABLE` constant with the UID of the table, or `None` for the `TableID` table.
    pub uid: Option<UidConstant>,
    pub uid_constants: Vec<UidConstant>,
    pub uid_range_constants: Vec<UidRangeConstant>,
    pub lookup_by_name: LookupByName,
    pub lookup_by_uid: LookupByUid,
    pub lookup_ranges: LookupRanges,
    pub lookup: LookupConstant,
}

impl Table {
    pub fn new_object_table(source: semantic_ast::Table) -> Self {
        let uid_ty = parse_quote! { ObjectRef<{ THIS_TABLE.to_u64() }> };
        let lookup_type = parse_quote! { RefTableLookup<'static, #uid_ty> };
        let name = Some(source.name.clone());
        Self::new(source, uid_ty, LookupConstant { lookup_type, name })
    }

    pub fn new_table_id_table(source: semantic_ast::Table) -> Self {
        let uid_ty = parse_quote! { TableRef };
        let lookup_type = parse_quote! { TableIdLookup<'static> };
        Self::new(source, uid_ty, LookupConstant { lookup_type, name: None })
    }

    pub fn new_special_table(source: semantic_ast::Table) -> Self {
        let uid_ty = parse_quote! { Uid };
        let lookup_type = parse_quote! { RefTableLookup<'static, #uid_ty> };
        let name = Some(source.name.clone());
        Self::new(source, uid_ty, LookupConstant { lookup_type, name })
    }

    fn new(source: semantic_ast::Table, uid_ty: Type, lookup: LookupConstant) -> Self {
        let uid = source.uid.map(|value| UidConstant {
            ident: format_ident!("THIS_TABLE"),
            uid: Uid { ty: parse_quote! { TableRef }, value },
        });

        let mut uid_constants = Vec::new();
        let mut lookup_by_name = LookupByName { uid_ty: uid_ty.clone(), lookup: BTreeMap::new() };
        let mut lookup_by_uid = LookupByUid { uid_ty: uid_ty.clone(), lookup: BTreeMap::new() };
        for semantic_ast::UidSingleEntry { name, value } in source.uid_entries {
            let ident = const_ident(&name);
            let uid = Uid { ty: uid_ty.clone(), value };
            uid_constants.push(UidConstant { ident: ident.clone(), uid });
            lookup_by_name.lookup.insert(name.clone(), ident.clone());
            lookup_by_uid.lookup.insert(value, (ident, name));
        }

        let mut uid_range_constants = Vec::new();
        let mut lookup_ranges = LookupRanges { uid_ty: uid_ty.clone(), lookup: Vec::new() };
        for range in source.uid_range_entries {
            let uid_range = UidRange::new(uid_ty.clone(), &range);
            let semantic_ast::UidRangeEntry { prefix, suffix, display_start, .. } = range;
            let ident = const_ident(&format!("{prefix}_{suffix}"));
            uid_range_constants.push(UidRangeConstant { ident, uid_range: uid_range.clone() });
            lookup_ranges.lookup.push(UidRangeLookupEntry { uid_range, prefix, suffix, display_start });
        }

        Self {
            name: source.name,
            uid,
            uid_constants,
            uid_range_constants,
            lookup_by_name,
            lookup_by_uid,
            lookup_ranges,
            lookup,
        }
    }
}

impl ToTokens for Table {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            uid,
            uid_constants,
            uid_range_constants,
            lookup_by_name,
            lookup_by_uid,
            lookup_ranges,
            lookup,
        } = self;
        let module = module_ident(name);
        tokens.extend(quote! {
            pub mod #module {
                use ::sed_packet::{ObjectRef, TableRef, Uid, UidRange};
                use crate::lookup::{UidRangeLookupEntry, SortedMap, RefTableLookup, TableIdLookup};

                #uid

                #(#uid_constants)*
                #(#uid_range_constants)*

                #lookup_by_name
                #lookup_by_uid
                #lookup_ranges

                #lookup
            }
        });
    }
}

pub struct SecurityProvider {
    pub name: String,
    pub uid: Option<Uid>,
    pub table_id: Option<Table>,
    pub special_tables: Vec<Table>,
    pub object_tables: Vec<Table>,
}

impl From<semantic_ast::SecurityProvider> for SecurityProvider {
    fn from(value: semantic_ast::SecurityProvider) -> Self {
        Self {
            name: value.name,
            uid: value.uid.map(|value| Uid { ty: parse_quote! { SpRef }, value }),
            table_id: value.table_id.map(Table::new_table_id_table),
            special_tables: value.special_tables.into_iter().map(Table::new_special_table).collect(),
            object_tables: value.object_tables.into_iter().map(Table::new_object_table).collect(),
        }
    }
}

impl ToTokens for SecurityProvider {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, uid: _, table_id, special_tables, object_tables } = self;
        let module = module_ident(name);

        let table_id_lookups: Vec<_> = table_id
            .as_ref()
            // First look up in the SP's `TableID` special table.
            .map(|_| quote! { &table_id::LOOKUP })
            .into_iter()
            // Then look up in the Core's Shared SP's `TableID`. This might be
            // a duplicate of the previous one, but that's not a problem.
            .chain(std::iter::once(quote! { &super::super::core::shared::table_id::LOOKUP }))
            .collect();

        let num_table_id_lookups = table_id_lookups.len();

        let mut sp_tables: Vec<_> = special_tables.iter().chain(object_tables).collect();
        // The tables must be sorted by UID for `SortedMap`.
        sp_tables.sort_by_key(|table| table.uid.as_ref().map(|this_table| this_table.uid.value));

        let sp_table_entries: Vec<_> = sp_tables
            .into_iter()
            .map(|table| {
                let module = module_ident(&table.name);
                quote! { (#module::THIS_TABLE, &#module::LOOKUP) }
            })
            .collect();
        let num_table_entries = sp_table_entries.len();

        tokens.extend(quote! {
            pub mod #module {
                use ::sed_packet::{TableRef};
                use crate::lookup::{TableLookup, TableIdLookup, SecurityProviderLookup, SortedMap};

                #table_id
                #(#special_tables)*
                #(#object_tables)*

                const TABLE_LOOKUPS: [(TableRef, &dyn TableLookup); #num_table_entries] = [
                    #(#sp_table_entries,)*
                ];

                const TABLE_ID_LOOKUPS: [&TableIdLookup; #num_table_id_lookups] = [
                    #(#table_id_lookups,)*
                ];

                pub const LOOKUP: SecurityProviderLookup<'static> = SecurityProviderLookup {
                    name: #name,
                    table_id_lookups: &TABLE_ID_LOOKUPS,
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
        let module = module_ident(name);
        let mut lookup_security_providers: Vec<_> = security_providers.iter().collect();
        // The SPs must be sorted by UID for `SortedMap`. The shared SP (uid = `None`) comes first.
        lookup_security_providers.sort_by_key(|security_provider| security_provider.uid.as_ref().map(|uid| uid.value));
        let num_security_providers = lookup_security_providers.len();
        let lookup_items = lookup_security_providers.into_iter().map(|security_provider| {
            let sp_module = module_ident(&security_provider.name);
            match &security_provider.uid {
                Some(uid) => quote! { (Some(#uid), &#sp_module::LOOKUP) },
                None => quote! { (None, &#sp_module::LOOKUP) },
            }
        });

        tokens.extend(quote! {
            pub mod #module {
                use crate::lookup::{SecurityProviderLookup, FeatureLookup, SortedMap, SpRef};

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

pub struct UidRegistry {
    pub features: Vec<Feature>,
}

impl From<semantic_ast::UidRegistry> for UidRegistry {
    fn from(value: semantic_ast::UidRegistry) -> Self {
        Self { features: value.features.into_iter().map(Into::into).collect() }
    }
}

impl ToTokens for UidRegistry {
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
    #[case::lifetime_and_type(parse_quote! { RefTableLookup<'static, Uid> }, parse_quote! { RefTableLookup })]
    #[case::nested_path(parse_quote! { a::B<u8>::C<u16> }, parse_quote! { a::B::C })]
    #[case::not_generic(parse_quote! { TableRef }, parse_quote! { TableRef })]
    #[case::not_a_path(parse_quote! { &'static Uid }, parse_quote! { &'static Uid })]
    fn strip_generic_arguments_(#[case] ty: Type, #[case] expected: Type) {
        assert_that!(strip_generic_arguments(&ty), eq(&expected));
    }
}
