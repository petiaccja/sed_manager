//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use quote::ToTokens as _;

pub use crate::semantic_ast::SemanticError;

mod codegen_ast;
mod schema_ast;
mod semantic_ast;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Parsing(#[from] serde_json::Error),
    #[error(transparent)]
    Semantic(#[from] SemanticError),
}

/// Parse the UID registry from the JSON file and generate the corresponding Rust
/// module that defines the same UIDs.
pub fn generate_uid_registry(registry_json: &str) -> Result<String, Error> {
    let registry: schema_ast::UidRegistry = serde_json::from_str(registry_json)?;
    let registry = semantic_ast::UidRegistry::try_from(registry)?;
    let registry = codegen_ast::UidRegistry::from(registry);
    let file = syn::parse2(registry.to_token_stream()).expect("must generate syntactically correct code");
    Ok(prettyplease::unparse(&file))
}
