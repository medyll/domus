use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::cell::Cell;
use syn::{visit_mut, visit_mut::VisitMut, Expr};

use crate::parser::{RsxAttr, RsxNode};

// ---------------------------------------------------------------------------
// Counter for unique variable names
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Generate a TokenStream that builds DOM from an RsxNode.
/// Handles both static elements and dynamic signal bindings.
pub fn generate(node: &RsxNode) -> TokenStream {
    reset_counter();
    gen_node(node, None)
}

/// Backward-compat alias used by static-only tests.
pub fn generate_static(node: &RsxNode) -> TokenStream {
    generate(node)
}

// ---------------------------------------------------------------------------
// Node generation
// ---------------------------------------------------------------------------

/// Generate code for a node.
/// `parent_var`: if Some, dynamic children append themselves to it directly.
fn gen_node(node: &RsxNode, parent_var: Option<&Ident>) -> TokenStream {
    match node {
        RsxNode::Element { tag, attrs, children } => gen_element(tag, attrs, children),
        RsxNode::Text(content) => {
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
        RsxNode::Dynamic(expr) => {
            if let Some(pv) = parent_var {
                // Inline dynamic binding: append text node + create_effect
                gen_dynamic_text_inline(expr, pv)
            } else {
                quote! { compile_error!("Dynamic node must be inside an element") }
            }
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

    // Event handler statements (on_* attributes)
    let event_stmts: Vec<TokenStream> = attrs
        .iter()
        .filter(|(name, _)| name.starts_with("on_"))
        .filter_map(|(name, val)| {
            if let RsxAttr::Dynamic(expr) = val {
                Some(gen_event_handler(expr, &el_var, name))
            } else {
                None
            }
        })
        .collect();

    // Static attribute-setting statements
    let static_attr_stmts: Vec<TokenStream> = attrs
        .iter()
        .filter(|(name, _)| !name.starts_with("on_"))
        .filter(|(_, v)| !matches!(v, RsxAttr::Dynamic(_)))
        .map(|(name, val)| {
            let attr_val = match val {
                RsxAttr::String(s) => {
                    let stripped = s.trim_matches('"');
                    quote! { #stripped }
                }
                RsxAttr::Ident(id) => quote! { #id },
                RsxAttr::Dynamic(_) => unreachable!(),
            };
            quote! {
                #el_var.set_attribute(#name, #attr_val)
                    .expect(concat!("failed to set attribute ", #name));
            }
        })
        .collect();

    // Dynamic attribute-binding effects (non-event on_ attrs)
    let dynamic_attr_stmts: Vec<TokenStream> = attrs
        .iter()
        .filter(|(name, _)| !name.starts_with("on_"))
        .filter_map(|(name, val)| {
            if let RsxAttr::Dynamic(expr) = val {
                Some(gen_dynamic_attr_effect(expr, &el_var, name))
            } else {
                None
            }
        })
        .collect();

    // Child statements — dynamic children are handled specially
    let child_stmts: Vec<TokenStream> = children
        .iter()
        .map(|child| {
            if let RsxNode::Dynamic(expr) = child {
                // Dynamic text: inline — appends its own node
                gen_dynamic_text_inline(expr, &el_var)
            } else {
                let child_ts = gen_node(child, Some(&el_var));
                quote! {
                    {
                        let __child = #child_ts;
                        #el_var.append_child(&__child)
                            .expect("failed to append child");
                    }
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

            #(#static_attr_stmts)*
            #(#dynamic_attr_stmts)*
            #(#event_stmts)*
            #(#child_stmts)*

            #el_var
        }
    }
}

// ---------------------------------------------------------------------------
// Dynamic binding helpers
// ---------------------------------------------------------------------------

/// Create an empty text node, append to `parent_var`, then wire up a
/// `create_effect` that keeps it in sync with the signal expression.
fn gen_dynamic_text_inline(expr: &Expr, parent_var: &Ident) -> TokenStream {
    let dyn_id = next_id();
    let dyn_var: Ident = format_ident!("__dyn_{}", dyn_id);
    let dyn_clone: Ident = format_ident!("__dyn_{}_c", dyn_id);

    let idents = analyze_expr(expr);
    let clone_stmts = generate_clones(&idents);
    let cloned_expr = substitute_clones(expr, &idents);

    quote! {
        let #dyn_var = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document")
            .create_text_node("");
        #parent_var.append_child(&#dyn_var)
            .expect("failed to append dynamic text node");
        #clone_stmts
        {
            let #dyn_clone = #dyn_var.clone();
            domius_core::effect::create_effect(move || {
                let __value = (#cloned_expr).to_string();
                #dyn_clone.set_text_content(Some(&__value));
            });
        }
    }
}

/// Create an `create_effect` that keeps a DOM attribute in sync with a signal.
fn gen_dynamic_attr_effect(expr: &Expr, el_var: &Ident, attr_name: &str) -> TokenStream {
    let idents = analyze_expr(expr);
    let clone_stmts = generate_clones(&idents);
    let cloned_expr = substitute_clones(expr, &idents);
    let el_clone: Ident = format_ident!("{}_ac", el_var);

    quote! {
        #clone_stmts
        {
            let #el_clone = #el_var.clone();
            domius_core::effect::create_effect(move || {
                let __value = (#cloned_expr).to_string();
                #el_clone.set_attribute(#attr_name, &__value)
                    .expect(concat!("failed to set dynamic attribute ", #attr_name));
            });
        }
    }
}

/// Generate a wasm-bindgen Closure for an event handler attribute.
/// `on_click` → `set_onclick(MouseEvent)`, `on_input` → `set_oninput(InputEvent)`, etc.
fn gen_event_handler(handler_expr: &Expr, el_var: &Ident, attr_name: &str) -> TokenStream {
    let handler_id = next_id();
    let handler_var: Ident = format_ident!("__handler_{}", handler_id);
    let el_clone: Ident = format_ident!("{}_ec", el_var);

    // Map on_X → (web_sys event type, setter method)
    let event_name = attr_name.trim_start_matches("on_");
    let (event_type_str, setter_str) = event_type_and_setter(event_name);
    let event_type: proc_macro2::TokenStream = event_type_str.parse().unwrap();
    let setter_ident: Ident = format_ident!("{}", setter_str);

    let idents = analyze_expr(handler_expr);
    let clone_stmts = generate_clones(&idents);
    let cloned_handler = substitute_clones(handler_expr, &idents);

    quote! {
        {
            #clone_stmts
            let #el_clone = #el_var.clone();
            let #handler_var = wasm_bindgen::closure::Closure::<dyn Fn(#event_type)>::new(
                #cloned_handler
            );
            #el_clone.#setter_ident(Some(
                wasm_bindgen::JsCast::unchecked_ref(#handler_var.as_ref())
            ));
            #handler_var.forget();
        }
    }
}

/// Map event name to `(web_sys type path, HtmlElement setter method)`.
fn event_type_and_setter(event: &str) -> (String, String) {
    let setter = format!("set_on{}", event);
    let ty = match event {
        "click" | "dblclick" | "mousedown" | "mouseup" | "mouseover" | "mouseout" => {
            "web_sys::MouseEvent"
        }
        "input" => "web_sys::InputEvent",
        "keydown" | "keyup" | "keypress" => "web_sys::KeyboardEvent",
        "submit" => "web_sys::SubmitEvent",
        "focus" | "blur" => "web_sys::FocusEvent",
        _ => "web_sys::Event",
    };
    (ty.to_string(), setter)
}

// ---------------------------------------------------------------------------
// Expression analysis and clone substitution
// ---------------------------------------------------------------------------

/// Collect all simple identifiers used in `expr` (excluding method names).
pub fn analyze_expr(expr: &Expr) -> Vec<String> {
    let mut vars = Vec::new();
    collect_idents(expr, &mut vars);
    vars.sort();
    vars.dedup();
    vars
}

fn collect_idents(expr: &Expr, vars: &mut Vec<String>) {
    match expr {
        Expr::Path(p) => {
            if let Some(ident) = p.path.get_ident() {
                let name = ident.to_string();
                // Skip Rust keywords / boolean literals
                if !matches!(name.as_str(), "true" | "false" | "None" | "Some") {
                    vars.push(name);
                }
            }
        }
        Expr::MethodCall(m) => {
            collect_idents(&m.receiver, vars);
            // Intentionally skip method name and args to avoid cloning method idents
        }
        Expr::Call(c) => {
            collect_idents(&c.func, vars);
            for arg in &c.args {
                collect_idents(arg, vars);
            }
        }
        Expr::Field(f) => collect_idents(&f.base, vars),
        Expr::Unary(u) => collect_idents(&u.expr, vars),
        Expr::Binary(b) => {
            collect_idents(&b.left, vars);
            collect_idents(&b.right, vars);
        }
        Expr::Paren(p) => collect_idents(&p.expr, vars),
        Expr::Reference(r) => collect_idents(&r.expr, vars),
        Expr::Closure(c) => collect_idents(&c.body, vars),
        Expr::Block(b) => {
            for stmt in &b.block.stmts {
                if let syn::Stmt::Expr(e) = stmt {
                    collect_idents(e, vars);
                }
            }
        }
        _ => {}
    }
}

/// Generate `let x_clone = x.clone();` for each ident in `vars`.
pub fn generate_clones(vars: &[String]) -> TokenStream {
    let stmts = vars.iter().map(|var| {
        let clone_var = format_ident!("{}_clone", var);
        let orig_var = format_ident!("{}", var);
        quote! { let #clone_var = #orig_var.clone(); }
    });
    quote! { #(#stmts)* }
}

/// Rewrite `expr` so every identifier in `idents` gets a `_clone` suffix.
pub fn substitute_clones(expr: &Expr, idents: &[String]) -> Expr {
    let mut out = expr.clone();
    CloneSubstituter { idents }.visit_expr_mut(&mut out);
    out
}

/// `syn::VisitMut` impl that renames listed identifiers to `{name}_clone`.
struct CloneSubstituter<'a> {
    idents: &'a [String],
}

impl VisitMut for CloneSubstituter<'_> {
    fn visit_ident_mut(&mut self, node: &mut syn::Ident) {
        if self.idents.contains(&node.to_string()) {
            *node = format_ident!("{}_clone", node);
        }
        visit_mut::visit_ident_mut(self, node);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_rsx;
    use quote::quote;
    use syn::parse_quote;

    fn ts(ts: TokenStream) -> String {
        ts.to_string()
    }

    // --- Static tests (unchanged) ---

    #[test]
    fn test_static_element_gen_is_valid_tokens() {
        let ast = parse_rsx(quote! { div(class: "btn") { "Click" } }).unwrap();
        let s = ts(generate_static(&ast));
        assert!(s.contains("create_element"));
        assert!(s.contains("set_attribute"));
        assert!(s.contains("create_text_node"));
    }

    #[test]
    fn test_element_no_attrs_no_children() {
        let ast = parse_rsx(quote! { div { } }).unwrap();
        let s = ts(generate_static(&ast));
        assert!(s.contains("create_element"));
        assert!(!s.contains("set_attribute"));
        assert!(!s.contains("create_text_node"));
    }

    #[test]
    fn test_text_node_gen() {
        let ast = parse_rsx(quote! { span { "hello world" } }).unwrap();
        let s = ts(generate_static(&ast));
        assert!(s.contains("create_text_node"));
        assert!(s.contains("hello world"));
    }

    #[test]
    fn test_multiple_attributes_gen() {
        let ast = parse_rsx(quote! { input(type: "text", placeholder: "Enter text") }).unwrap();
        let s = ts(generate_static(&ast));
        assert!(s.contains("set_attribute"));
        assert!(s.contains("type"));
        assert!(s.contains("placeholder"));
    }

    #[test]
    fn test_nested_element_gen() {
        let ast = parse_rsx(quote! { div { span { "inner" } } }).unwrap();
        let s = ts(generate_static(&ast));
        assert!(s.contains("append_child"));
        assert_eq!(s.matches("create_element").count(), 2);
    }

    #[test]
    fn test_variable_hygiene() {
        let ast = parse_rsx(quote! { div { span { } p { } } }).unwrap();
        let s = ts(generate_static(&ast));
        assert!(s.contains("__el_0"));
        assert!(s.contains("__el_1"));
        assert!(s.contains("__el_2"));
    }

    #[test]
    fn test_html_style_gen() {
        let ast = parse_rsx(quote! { <section> content </section> }).unwrap();
        let s = ts(generate_static(&ast));
        assert!(s.contains("create_element"));
        assert!(s.contains("section"));
    }

    // --- Dynamic binding tests ---

    #[test]
    fn test_analyze_expr_simple_ident() {
        let expr: Expr = parse_quote! { count };
        let vars = analyze_expr(&expr);
        assert_eq!(vars, vec!["count"]);
    }

    #[test]
    fn test_analyze_expr_method_call() {
        let expr: Expr = parse_quote! { signal.get() };
        let vars = analyze_expr(&expr);
        assert_eq!(vars, vec!["signal"]);
    }

    #[test]
    fn test_analyze_expr_binary() {
        let expr: Expr = parse_quote! { a + b };
        let vars = analyze_expr(&expr);
        assert!(vars.contains(&"a".to_string()));
        assert!(vars.contains(&"b".to_string()));
    }

    #[test]
    fn test_analyze_expr_no_clone_for_literals() {
        let expr: Expr = parse_quote! { "hello" };
        let vars = analyze_expr(&expr);
        assert!(vars.is_empty());
    }

    #[test]
    fn test_analyze_expr_no_clone_for_booleans() {
        let expr: Expr = parse_quote! { true };
        let vars = analyze_expr(&expr);
        assert!(vars.is_empty());
    }

    #[test]
    fn test_generate_clones_output() {
        let vars = vec!["count".to_string(), "name".to_string()];
        let s = ts(generate_clones(&vars));
        assert!(s.contains("count_clone"));
        assert!(s.contains("name_clone"));
        assert!(s.contains("clone ()"));
    }

    #[test]
    fn test_substitute_clones_renames_idents() {
        let expr: Expr = parse_quote! { count.get() };
        let idents = vec!["count".to_string()];
        let subst = substitute_clones(&expr, &idents);
        let s = quote! { #subst }.to_string();
        assert!(s.contains("count_clone"));
        assert!(!s.contains("count.get"));
    }

    #[test]
    fn test_dynamic_text_generates_create_effect() {
        let ast = parse_rsx(quote! { span { {count} } }).unwrap();
        let s = ts(generate(&ast));
        assert!(s.contains("create_effect"));
        assert!(s.contains("set_text_content"));
        assert!(s.contains("count_clone"));
        assert!(s.contains("create_text_node"));
    }

    #[test]
    fn test_dynamic_attr_generates_create_effect() {
        let ast = parse_rsx(quote! { div(class: {theme}) { } }).unwrap();
        let s = ts(generate(&ast));
        assert!(s.contains("create_effect"));
        assert!(s.contains("set_attribute"));
        assert!(s.contains("theme_clone"));
    }

    #[test]
    fn test_multiple_dynamics_each_get_effect() {
        let ast = parse_rsx(quote! { div { {first} {last} } }).unwrap();
        let s = ts(generate(&ast));
        assert_eq!(s.matches("create_effect").count(), 2);
        assert!(s.contains("first_clone"));
        assert!(s.contains("last_clone"));
    }

    #[test]
    fn test_mixed_static_and_dynamic() {
        let ast = parse_rsx(quote! { div { "Static " {dynamic_val} } }).unwrap();
        let s = ts(generate(&ast));
        assert!(s.contains("create_text_node"));
        assert!(s.contains("create_effect"));
        assert!(s.contains("dynamic_val_clone"));
    }

    #[test]
    fn test_static_generates_zero_effects() {
        let ast = parse_rsx(quote! { div(class: "x") { "text" } }).unwrap();
        let s = ts(generate(&ast));
        assert!(!s.contains("create_effect"));
    }

    // --- Event handler tests ---

    #[test]
    fn test_click_handler_generates_closure() {
        let ast = parse_rsx(quote! {
            button(on_click: |_| signal.set(42)) { "Click" }
        })
        .unwrap();
        let s = ts(generate(&ast));
        assert!(s.contains("Closure"));
        assert!(s.contains("set_onclick"));
        assert!(s.contains("forget"));
    }

    #[test]
    fn test_input_handler_uses_input_event_type() {
        let ast = parse_rsx(quote! {
            input(on_input: |_| count.set(1))
        })
        .unwrap();
        let s = ts(generate(&ast));
        assert!(s.contains("set_oninput"));
        assert!(s.contains("InputEvent"));
    }

    #[test]
    fn test_multiple_handlers_on_same_element() {
        let ast = parse_rsx(quote! {
            div(on_click: |_| a.set(1), on_mouseout: |_| b.set(2)) { }
        })
        .unwrap();
        let s = ts(generate(&ast));
        assert!(s.contains("set_onclick"));
        assert!(s.contains("set_onmouseout"));
        assert_eq!(s.matches("Closure").count(), 2);
    }

    #[test]
    fn test_handler_captures_state_via_clone() {
        let ast = parse_rsx(quote! {
            button(on_click: |_| signal.set(99)) { "Go" }
        })
        .unwrap();
        let s = ts(generate(&ast));
        assert!(s.contains("signal_clone"));
        assert!(s.contains("clone ()"));
    }

    #[test]
    fn test_event_handler_does_not_generate_set_attribute() {
        let ast = parse_rsx(quote! {
            button(on_click: |_| {}) { "x" }
        })
        .unwrap();
        let s = ts(generate(&ast));
        assert!(!s.contains("set_attribute"));
        assert!(s.contains("set_onclick"));
    }
}
