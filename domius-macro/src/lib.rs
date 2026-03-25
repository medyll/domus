//! Procedural macros for Domius.

mod codegen;
mod parser;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

/// Parse RSX into an AST.
///
/// Supports both Rust-style (`div { }`) and HTML-style (`<div></div>`) syntax.
/// Transforms macro input into a reactive component representation.
#[proc_macro]
pub fn domius(input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input);
    match parser::parse_rsx(input2) {
        Ok(ast) => {
            // For now, return a debug representation
            // Later, this will codegen component initialization
            let debug_output = format!("/* RSX AST: {:?} */", ast);
            debug_output.parse().unwrap_or_else(|_| TokenStream::new())
        }
        Err(e) => e.to_compile_error().into(),
    }
}
