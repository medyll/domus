use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::cell::Cell;

use crate::parser::{RsxAttr, RsxNode};

thread_local! {
    static COUNTER: Cell<usize> = Cell::new(0);
}

fn next_id() -> usize {
    COUNTER.with(|c| {
        let v = c.get();
        c.set(v + 1);
        v
    })
}

fn reset_counter() {
    COUNTER.with(|c| c.set(0));
}

/// Generate a TokenStream that builds static DOM from an RsxNode.
/// Dynamic nodes (S2-03) are not yet handled.
pub fn generate_static(node: &RsxNode) -> TokenStream {
    reset_counter();
    gen_node(node)
}

fn gen_node(node: &RsxNode) -> TokenStream {
    match node {
        RsxNode::Element { tag, attrs, children } => gen_element(tag, attrs, children),
        RsxNode::Text(content) => gen_text(content),
        RsxNode::Dynamic(_) => {
            // Dynamic bindings handled in S2-03
            quote! { compile_error!("dynamic bindings require S2-03 effect integration") }
        }
        RsxNode::Component { name, .. } => {
            let msg = format!("component <{}> not yet supported", name);
            quote! { compile_error!(#msg) }
        }
    }
}

fn gen_element(tag: &str, attrs: &[(String, RsxAttr)], children: &[RsxNode]) -> TokenStream {
    let el_id = next_id();
    let el_var: Ident = format_ident!("__el_{}", el_id);

    // Attribute-setting statements
    let attr_stmts: Vec<TokenStream> = attrs
        .iter()
        .map(|(name, val)| {
            let attr_val = match val {
                RsxAttr::String(s) => {
                    // Strip surrounding quotes if present (from Rust literal)
                    let stripped = s.trim_matches('"');
                    quote! { #stripped }
                }
                RsxAttr::Ident(id) => quote! { #id },
                RsxAttr::Dynamic(expr) => quote! { &(#expr).to_string() },
            };
            quote! {
                #el_var.set_attribute(#name, #attr_val)
                    .expect(concat!("failed to set attribute ", #name));
            }
        })
        .collect();

    // Child-appending statements
    let child_stmts: Vec<TokenStream> = children
        .iter()
        .map(|child| {
            let child_ts = gen_node(child);
            quote! {
                {
                    let __child = #child_ts;
                    #el_var.append_child(&__child)
                        .expect("failed to append child");
                }
            }
        })
        .collect();

    quote! {
        {
            let #el_var: web_sys::Element = web_sys::window()
                .expect("no window")
                .document()
                .expect("no document")
                .create_element(#tag)
                .expect(concat!("failed to create element: ", #tag));

            #(#attr_stmts)*
            #(#child_stmts)*

            #el_var
        }
    }
}

fn gen_text(content: &str) -> TokenStream {
    // Strip surrounding quotes from Rust string literals
    let text = content.trim_matches('"');
    quote! {
        web_sys::window()
            .expect("no window")
            .document()
            .expect("no document")
            .create_text_node(#text)
            .dyn_into::<web_sys::Node>()
            .expect("text node as Node")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_rsx;
    use quote::quote;

    fn token_string(ts: TokenStream) -> String {
        ts.to_string()
    }

    #[test]
    fn test_static_element_gen_is_valid_tokens() {
        let ast = parse_rsx(quote! {
            div(class: "btn") { "Click" }
        })
        .unwrap();

        let generated = generate_static(&ast);
        let s = token_string(generated);
        // Should contain web_sys DOM calls
        assert!(s.contains("create_element"));
        assert!(s.contains("set_attribute"));
        assert!(s.contains("create_text_node"));
    }

    #[test]
    fn test_element_no_attrs_no_children() {
        let ast = parse_rsx(quote! { div { } }).unwrap();
        let s = token_string(generate_static(&ast));
        assert!(s.contains("create_element"));
        assert!(!s.contains("set_attribute"));
        assert!(!s.contains("create_text_node"));
    }

    #[test]
    fn test_text_node_gen() {
        let ast = parse_rsx(quote! { span { "hello world" } }).unwrap();
        let s = token_string(generate_static(&ast));
        assert!(s.contains("create_text_node"));
        assert!(s.contains("hello world"));
    }

    #[test]
    fn test_multiple_attributes_gen() {
        let ast = parse_rsx(quote! {
            input(type: "text", placeholder: "Enter text")
        })
        .unwrap();
        let s = token_string(generate_static(&ast));
        assert!(s.contains("set_attribute"));
        assert!(s.contains("type"));
        assert!(s.contains("placeholder"));
    }

    #[test]
    fn test_nested_element_gen() {
        let ast = parse_rsx(quote! {
            div {
                span { "inner" }
            }
        })
        .unwrap();
        let s = token_string(generate_static(&ast));
        assert!(s.contains("append_child"));
        // Two create_element calls: div and span
        assert_eq!(s.matches("create_element").count(), 2);
    }

    #[test]
    fn test_variable_hygiene() {
        let ast = parse_rsx(quote! {
            div {
                span { }
                p { }
            }
        })
        .unwrap();
        let s = token_string(generate_static(&ast));
        // Variables should use __el_ prefix
        assert!(s.contains("__el_0"));
        assert!(s.contains("__el_1"));
        assert!(s.contains("__el_2"));
    }

    #[test]
    fn test_html_style_gen() {
        let ast = parse_rsx(quote! {
            <section> content </section>
        })
        .unwrap();
        let s = token_string(generate_static(&ast));
        assert!(s.contains("create_element"));
        assert!(s.contains("section"));
    }
}
