//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::fs::File;
use std::io::{Read as _, Write as _};
use std::path::PathBuf;

fn main() -> Result<(), ()> {
    let registry_path = "uid_registry.json";
    let mut registry_file = File::open(registry_path).unwrap();
    let mut registry_json = String::new();
    registry_file.read_to_string(&mut registry_json).unwrap();

    let generated_registry = sed_spec_codegen::generate_uid_registry(&registry_json).unwrap();

    let mut out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    out_path.push("uid_registry.rs");
    let mut out_file = File::create(out_path).unwrap();
    out_file.write_all(generated_registry.as_bytes()).unwrap();
    println!("cargo::rerun-if-changed={registry_path}");
    Ok(())
}
