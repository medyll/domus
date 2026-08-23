//! Procedural macros for Domius.
#![allow(clippy::missing_const_for_thread_local)]
#![allow(clippy::large_enum_variant)]
#![allow(dead_code)]

mod codegen;
mod parser;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

fn expand(input: TokenStream2) -> TokenStream2 {
    match parser::parse_rsx(input) {
        Ok(ast) => codegen::generate(&ast),
        Err(error) => error.to_compile_error(),
    }
}

/// Build a live DOM tree from Rust-style or HTML-style RSX.
#[proc_macro]
pub fn domus(input: TokenStream) -> TokenStream {
    expand(input.into()).into()
}

/// Compatibility alias for the original macro name.
#[proc_macro]
pub fn domius(input: TokenStream) -> TokenStream {
    domus(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn public_expansion_builds_dom_instead_of_returning_nothing() {
        let output = expand(quote! { div(class: "card") { "Hello" } }).to_string();

        assert!(output.contains("create_element"));
        assert!(output.contains("set_attribute"));
        assert!(output.contains("create_text_node"));
        assert!(!output.contains("RSX AST"));
    }

    #[test]
    fn public_expansion_preserves_parser_errors() {
        let output = expand(TokenStream2::new()).to_string();

        assert!(output.contains("compile_error"));
        assert!(output.contains("empty macro input"));
    }
}
