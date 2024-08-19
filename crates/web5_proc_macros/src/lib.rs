extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::process::Command;

#[proc_macro]
pub fn git_sha(_input: TokenStream) -> TokenStream {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to execute git command");

    let git_hash = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 sequence")
        .trim()
        .to_string();

    let expanded = quote! {
        #git_hash
    };

    TokenStream::from(expanded)
}
