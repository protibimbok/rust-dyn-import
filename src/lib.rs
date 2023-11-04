extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::fs;
use syn::{parse_macro_input, LitStr};

#[macro_use]
mod macros;

#[proc_macro]
pub fn import_mods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();

    let mut from_dir = get_cwd!().expect("Couldn't get the CWD");

    for part in input.split(&['/', '\\'][..]) {
        from_dir = from_dir.join(part);
    }

    if !from_dir.exists() {
        panic!("Path doesn't exist: {:?}", from_dir)
    }

    let mut generated = quote! {};

    for entry in fs::read_dir(from_dir).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let entry_path = entry.path();

            if entry_path.is_file() {
                if entry_path.extension().unwrap() != "rs"
                    || entry_path.file_name().unwrap() == "mod.rs"
                {
                    continue;
                }
            } else {
                let file_path = entry_path.join("mod.rs");
                if !file_path.exists() {
                    continue;
                }
            }
            let filename = entry_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let mod_name = format_ident!("{}", filename);
            let mod_des = quote! {
                pub mod #mod_name;
            };
            generated.extend(mod_des);
        }
    }
    generated.into()
}
