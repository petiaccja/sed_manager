//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_spec_codegen::generate_database_module;

const TEST_DATABASE: &str = include_str!("test_database.json");

#[test]
fn generate_test_database() {
    let generated = generate_database_module(TEST_DATABASE).expect("test database must be valid");
    insta::assert_snapshot!(generated);
}
