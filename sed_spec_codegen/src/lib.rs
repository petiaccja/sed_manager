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

pub fn generate_database_module(database_json: &str) -> Result<String, Error> {
    let database: schema_ast::Database = serde_json::from_str(database_json)?;
    let database = semantic_ast::Database::try_from(database)?;
    let database = codegen_ast::Database::from(database);
    let file = syn::parse2(database.to_token_stream()).expect("must generate syntactically correct code");
    Ok(prettyplease::unparse(&file))
}
