//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_spec_codegen::generate_uid_registry;

const TEST_REGISTRY: &str = include_str!("test_uid_registry.json");

#[test]
fn generate_test_uid_registry() {
    let generated = generate_uid_registry(TEST_REGISTRY).expect("test registry must be valid");
    insta::assert_snapshot!(generated);
}
