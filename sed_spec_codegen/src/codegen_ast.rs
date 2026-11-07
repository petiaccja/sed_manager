use std::{collections::BTreeMap, str::FromStr as _};

use heck::{ToShoutySnakeCase, ToSnakeCase};
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

use crate::semantic_ast;

pub struct Hex {
    pub value: u64,
}

impl ToTokens for Hex {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let value = proc_macro2::Literal::from_str(&format!("0x{:016x}_u64", self.value))
            .expect("hex literal valid by construction");
        value.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uid {
    pub value: u64,
}

impl ToTokens for Uid {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let hex = Hex { value: self.value };
        tokens.extend(quote! {
            ObjectRef::new(#hex)
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UidRange {
    pub start: u64,
    pub count: u64,
    pub step: u64,
}

impl ToTokens for UidRange {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let start = Uid { value: self.start };
        let end = Uid { value: self.start + self.count * self.step };
        let step = self.step as u32;
        tokens.extend(quote! {
            UidRange { start: #start, end: #end, step: #step }
        });
    }
}

#[derive(Debug)]
pub struct UidConst {
    pub ident: Ident,
    pub uid: Uid,
}

impl ToTokens for UidConst {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { ident, uid } = self;
        tokens.extend(quote! {
            pub const #ident: ObjectRef<{ THIS_TABLE.to_u64() }> = #uid;
        });
    }
}

#[derive(Debug)]
pub struct UidRangeConst {
    pub ident: Ident,
    pub uid_range: UidRange,
}

impl ToTokens for UidRangeConst {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { ident, uid_range } = self;
        tokens.extend(quote! {
            pub const #ident: UidRange<ObjectRef<{ THIS_TABLE.to_u64() }>> = #uid_range;
        });
    }
}

#[derive(Debug)]
pub struct LookupByName {
    pub lookup: BTreeMap<String, Ident>,
}

impl ToTokens for LookupByName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let len = self.lookup.len();
        let items = self.lookup.iter().map(|(name, ident)| quote! { (#name, #ident) });
        tokens.extend(quote! {
            const BY_NAME: [(&str, ObjectRef<{ THIS_TABLE.to_u64() }>); #len] = [
                #(#items,)*
            ];
        });
    }
}

#[derive(Debug)]
pub struct LookupByValue {
    pub lookup: BTreeMap<(Uid, Ident), String>,
}

impl ToTokens for LookupByValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let len = self.lookup.len();
        let items = self.lookup.iter().map(|((_, ident), name)| quote! { (#ident, #name) });
        tokens.extend(quote! {
            const BY_VALUE: [(ObjectRef<{ THIS_TABLE.to_u64() }>, &str); #len] = [
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
    pub lookup: Vec<RangeLookupEntry>,
}

impl ToTokens for LookupRanges {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let len = self.lookup.len();
        let items = &self.lookup;
        tokens.extend(quote! {
            const RANGES: [UidRangeEntry<ObjectRef<{ THIS_TABLE.to_u64() }>>; #len] = [
                #(#items,)*
            ];
        });
    }
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub uids: Vec<UidConst>,
    pub uid_ranges: Vec<UidRangeConst>,
    pub lookup_by_name: LookupByName,
    pub lookup_by_value: LookupByValue,
    pub lookup_ranges: LookupRanges,
}

impl Table {
    pub fn ident_lower(&self) -> Ident {
        format_ident!("{}", self.name.to_snake_case())
    }

    pub fn ident_upper(&self) -> Ident {
        format_ident!("{}", self.name.to_shouty_snake_case())
    }
}

impl From<semantic_ast::Table> for Table {
    fn from(value: semantic_ast::Table) -> Self {
        let mut uids = Vec::new();
        let mut lookup_by_name = LookupByName { lookup: BTreeMap::new() };
        let mut lookup_by_value = LookupByValue { lookup: BTreeMap::new() };
        for semantic_ast::UidObject { name, value } in value.uid_objects {
            let ident = format_ident!("{}", name.to_shouty_snake_case());
            let uid = Uid { value };
            uids.push(UidConst { ident: ident.clone(), uid: uid.clone() });
            lookup_by_name.lookup.insert(name.clone(), ident.clone());
            lookup_by_value.lookup.insert((uid, ident.clone()), name);
        }

        let mut uid_ranges = Vec::new();
        let mut lookup_ranges = LookupRanges { lookup: Vec::new() };
        for semantic_ast::UidRangeObject { prefix, suffix, start, count, step, start_display } in
            value.uid_range_objects
        {
            let ident = format_ident!("{}", format!("{prefix}_{suffix}").to_shouty_snake_case());
            let uid_range = UidRange { start, count, step };
            uid_ranges.push(UidRangeConst { ident, uid_range: uid_range.clone() });
            lookup_ranges.lookup.push(RangeLookupEntry { uid_range, prefix, suffix, start_display });
        }

        Self { name: value.name, uids, uid_ranges, lookup_by_name, lookup_by_value, lookup_ranges }
    }
}

impl ToTokens for Table {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { name, uids, uid_ranges, lookup_by_name, lookup_by_value, lookup_ranges } = self;
        let ident_lower = self.ident_lower();
        let ident_upper = self.ident_upper();
        tokens.extend(quote! {
            pub mod #ident_lower {
                use ::sed_packet::{ObjectRef, TableRef, UidRange};
                use crate::lookup::{UidRangeEntry, SortedMap, ObjectTableLookup};

                const THIS_TABLE: TableRef = super::super::super::core::shared::table_id::#ident_upper;

                #(#uids)*
                #(#uid_ranges)*

                #lookup_by_name
                #lookup_by_value
                #lookup_ranges

                pub const LOOKUP: ObjectTableLookup<'static, { THIS_TABLE.to_u64() }> = ObjectTableLookup {
                    name: #name,
                    by_name: SortedMap { items: &BY_NAME },
                    by_uid: SortedMap { items: &BY_VALUE },
                    ranges: &RANGES,
                };
            }
        });
    }
}

pub struct SecurityProvider {
    pub name: String,
    pub tables: Vec<Table>,
}

impl From<semantic_ast::SecurityProvider> for SecurityProvider {
    fn from(value: semantic_ast::SecurityProvider) -> Self {
        Self { name: value.name, tables: value.tables.into_iter().map(Into::into).collect() }
    }
}

impl ToTokens for SecurityProvider {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { name, tables } = self;
        let ident_lower = format_ident!("{}", name.to_snake_case());
        let num_tables = tables.len();
        let lookup_items = tables.iter().map(|table| {
            let ident_lower = table.ident_lower();
            let ident_upper = table.ident_upper();
            quote! { (super::super::core::shared::table_id::#ident_upper, &#ident_lower::LOOKUP) }
        });

        tokens.extend(quote! {
            pub mod #ident_lower {
                use ::sed_packet::{TableRef};
                use crate::lookup::{TableLookup, SecurityProviderLookup, SortedMap};

                #(#tables)*

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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote! { const GENERATED_MARKER: () = (); });
        for feature in &self.features {
            feature.to_tokens(tokens);
        }
    }
}
