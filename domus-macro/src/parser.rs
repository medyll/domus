use proc_macro2::{TokenStream, TokenTree, Delimiter, Group, Span};
use syn::{parse2, Expr, Error, Ident, LitStr, Result as SynResult};
use std::fmt;

/// RSX AST node representing parsed macro input.
#[derive(Clone)]
pub enum RsxNode {
    /// An element node with tag, attributes, and children.
    Element {
        tag: String,
        attrs: Vec<(String, RsxAttr)>,
        children: Vec<RsxNode>,
    },
    /// A static text node.
    Text(String),
    /// A dynamic expression node (e.g., `{signal}`).
    Dynamic(Expr),
    /// A component reference with props.
    Component {
        name: String,
        props: Vec<(String, RsxAttr)>,
    },
}

impl std::fmt::Debug for RsxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element { tag, attrs, children } => {
                f.debug_struct("Element")
                    .field("tag", tag)
                    .field("attrs", attrs)
                    .field("children", children)
                    .finish()
            }
            Self::Text(s) => f.debug_tuple("Text").field(s).finish(),
            Self::Dynamic(_) => f.debug_tuple("Dynamic").field(&"<expr>").finish(),
            Self::Component { name, props } => {
                f.debug_struct("Component")
                    .field("name", name)
                    .field("props", props)
                    .finish()
            }
        }
    }
}

/// RSX attribute value representation.
#[derive(Clone)]
pub enum RsxAttr {
    /// Static string value: `class: "btn"`.
    String(String),
    /// Dynamic expression: `on_click: {handler}`.
    Dynamic(Expr),
    /// Identifier: `some_value`.
    Ident(String),
}

impl std::fmt::Debug for RsxAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => f.debug_tuple("String").field(s).finish(),
            Self::Dynamic(_) => f.debug_tuple("Dynamic").field(&"<expr>").finish(),
            Self::Ident(s) => f.debug_tuple("Ident").field(s).finish(),
        }
    }
}

/// Parse RSX macro input and return the AST.
pub fn parse_rsx(input: TokenStream) -> SynResult<RsxNode> {
    let tokens: Vec<TokenTree> = input.into_iter().collect();

    if tokens.is_empty() {
        return Err(Error::new(Span::call_site(), "empty macro input"));
    }

    // Detect syntax style by looking for '<' token anywhere near the start
    // (accounting for leading whitespace in token stream)
    let is_html_style = tokens
        .iter()
        .take(3)
        .any(|t| matches!(t, TokenTree::Punct(p) if p.as_char() == '<'));

    if is_html_style {
        parse_html_style(&tokens)
    } else {
        parse_rust_style(&tokens)
    }
}

/// Parse Rust-style RSX: `div(attrs) { children }`.
fn parse_rust_style(tokens: &[TokenTree]) -> SynResult<RsxNode> {
    let mut parser = RustStyleParser::new(tokens);
    parser.parse_element()
}

/// Parse HTML-style RSX: `<div attrs>children</div>`.
fn parse_html_style(tokens: &[TokenTree]) -> SynResult<RsxNode> {
    // Use space separator so adjacent idents don't merge (e.g. `div` + `class` → `divclass`)
    let source = tokens
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let mut parser = HtmlStyleParser::new(&source);
    parser.parse_element()
}

/// Rust-style parser using syn's parsing infrastructure.
struct RustStyleParser<'a> {
    tokens: &'a [TokenTree],
    pos: usize,
}

impl<'a> RustStyleParser<'a> {
    fn new(tokens: &'a [TokenTree]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&'a TokenTree> {
        self.tokens.get(self.pos)
    }

    fn peek(&self) -> Option<&'a TokenTree> {
        self.tokens.get(self.pos + 1)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse_element(&mut self) -> SynResult<RsxNode> {
        // Parse tag name (must be an identifier)
        let tag = match self.current() {
            Some(TokenTree::Ident(id)) => {
                let tag = id.to_string();
                self.advance();
                tag
            }
            _ => return Err(Error::new(Span::call_site(), "expected element tag")),
        };

        // Check for attributes or children
        let mut attrs = Vec::new();

        // Parse attributes if present (inside parentheses)
        if let Some(TokenTree::Group(g)) = self.current() {
            if g.delimiter() == Delimiter::Parenthesis {
                self.advance();
                attrs = self.parse_attributes(g.stream())?;
            }
        }

        // Parse children if present (inside braces)
        let children = if let Some(TokenTree::Group(g)) = self.current() {
            if g.delimiter() == Delimiter::Brace {
                self.advance();
                self.parse_children(g.stream())?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        Ok(RsxNode::Element { tag, attrs, children })
    }

    fn parse_attributes(&mut self, stream: TokenStream) -> SynResult<Vec<(String, RsxAttr)>> {
        let tokens: Vec<TokenTree> = stream.into_iter().collect();
        let mut attrs = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            // Expect key (identifier)
            let key = match &tokens[i] {
                TokenTree::Ident(id) => id.to_string(),
                _ => {
                    i += 1;
                    continue; // Skip non-identifier tokens
                }
            };
            i += 1;

            // Expect colon
            if let Some(TokenTree::Punct(p)) = tokens.get(i) {
                if p.as_char() == ':' {
                    i += 1;
                } else {
                    continue;
                }
            } else {
                continue;
            }

            // Parse value
            let value = match tokens.get(i) {
                Some(TokenTree::Literal(lit)) => {
                    let val_str = lit.to_string();
                    i += 1;
                    RsxAttr::String(val_str)
                }
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
                    let expr: Expr = parse2(g.stream())?;
                    i += 1;
                    RsxAttr::Dynamic(expr)
                }
                Some(TokenTree::Ident(id)) => {
                    let ident = id.to_string();
                    i += 1;
                    RsxAttr::Ident(ident)
                }
                _ => {
                    i += 1;
                    continue;
                }
            };

            attrs.push((key, value));

            // Skip comma if present
            if let Some(TokenTree::Punct(p)) = tokens.get(i) {
                if p.as_char() == ',' {
                    i += 1;
                }
            }
        }

        Ok(attrs)
    }

    fn parse_children(&mut self, stream: TokenStream) -> SynResult<Vec<RsxNode>> {
        let tokens: Vec<TokenTree> = stream.into_iter().collect();
        let mut children = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            match &tokens[i] {
                TokenTree::Literal(lit) => {
                    children.push(RsxNode::Text(lit.to_string()));
                    i += 1;
                }
                TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                    let expr: Expr = parse2(g.stream())?;
                    children.push(RsxNode::Dynamic(expr));
                    i += 1;
                }
                TokenTree::Ident(_) | TokenTree::Punct(_) => {
                    // Try to parse as nested element
                    let elem_tokens = &tokens[i..];
                    let mut elem_parser = RustStyleParser::new(elem_tokens);
                    if let Ok(elem) = elem_parser.parse_element() {
                        children.push(elem);
                        i += elem_parser.pos;
                    } else {
                        i += 1;
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }

        Ok(children)
    }
}

/// HTML-style parser (handwritten recursive descent).
struct HtmlStyleParser {
    source: String,
    pos: usize,
}

impl HtmlStyleParser {
    fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            pos: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        self.source[self.pos..]
            .chars()
            .nth(offset)
    }

    fn advance(&mut self, count: usize) {
        for _ in 0..count {
            self.pos += self.current_char().map_or(1, |c| c.len_utf8());
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    fn parse_element(&mut self) -> SynResult<RsxNode> {
        self.skip_whitespace();

        // Expect opening `<`
        if self.current_char() != Some('<') {
            return Err(Error::new(Span::call_site(), "expected '<'"));
        }
        self.advance(1);
        self.skip_whitespace(); // tokens are space-joined, so `< div` has a space

        // Parse tag name
        let tag = self.parse_tag_name()?;

        // Parse attributes
        let mut attrs = Vec::new();
        self.skip_whitespace();

        while self.current_char() != Some('>') && self.current_char().is_some() {
            if self.current_char() == Some('/') {
                // Self-closing element (`/ >` with space after space-join)
                self.advance(1);
                self.skip_whitespace();
                if self.current_char() == Some('>') {
                    self.advance(1);
                    return Ok(RsxNode::Element {
                        tag,
                        attrs,
                        children: Vec::new(),
                    });
                }
            } else {
                let attr = self.parse_attribute()?;
                attrs.push(attr);
            }
            self.skip_whitespace();
        }

        // Consume closing `>`
        if self.current_char() == Some('>') {
            self.advance(1);
        }

        // Parse children
        let children = self.parse_children(&tag)?;

        // Expect closing tag
        self.skip_whitespace();
        if self.current_char() == Some('<') {
            self.advance(1);
            self.skip_whitespace();
            if self.current_char() == Some('/') {
                self.advance(1);
                self.skip_whitespace();
                let closing_tag = self.parse_tag_name()?;
                if closing_tag != tag {
                    return Err(Error::new(
                        Span::call_site(),
                        format!("mismatched closing tag: expected '{}', found '{}'", tag, closing_tag),
                    ));
                }
                self.skip_whitespace();
                if self.current_char() == Some('>') {
                    self.advance(1);
                }
            }
        }

        Ok(RsxNode::Element { tag, attrs, children })
    }

    fn parse_tag_name(&mut self) -> SynResult<String> {
        let mut name = String::new();
        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                name.push(c);
                self.advance(1);
            } else {
                break;
            }
        }

        if name.is_empty() {
            Err(Error::new(Span::call_site(), "expected tag name"))
        } else {
            Ok(name)
        }
    }

    fn parse_attribute(&mut self) -> SynResult<(String, RsxAttr)> {
        self.skip_whitespace();

        // Parse attribute name
        let mut key = String::new();
        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                key.push(c);
                self.advance(1);
            } else {
                break;
            }
        }

        if key.is_empty() {
            return Err(Error::new(Span::call_site(), "expected attribute name"));
        }

        self.skip_whitespace();

        // Expect '='
        if self.current_char() != Some('=') {
            return Err(Error::new(Span::call_site(), "expected '=' after attribute name"));
        }
        self.advance(1);
        self.skip_whitespace();

        // Parse value
        let value = if self.current_char() == Some('"') {
            self.advance(1);
            let mut val = String::new();
            while let Some(c) = self.current_char() {
                if c == '"' {
                    self.advance(1);
                    break;
                }
                val.push(c);
                self.advance(1);
            }
            RsxAttr::String(val)
        } else if self.current_char() == Some('\'') {
            self.advance(1);
            let mut val = String::new();
            while let Some(c) = self.current_char() {
                if c == '\'' {
                    self.advance(1);
                    break;
                }
                val.push(c);
                self.advance(1);
            }
            RsxAttr::String(val)
        } else {
            let mut val = String::new();
            while let Some(c) = self.current_char() {
                if c.is_whitespace() || c == '>' || c == '/' {
                    break;
                }
                val.push(c);
                self.advance(1);
            }
            RsxAttr::String(val)
        };

        Ok((key, value))
    }

    fn parse_children(&mut self, _closing_tag: &str) -> SynResult<Vec<RsxNode>> {
        let mut children = Vec::new();
        let mut text = String::new();

        while self.pos < self.source.len() {
            if self.current_char() == Some('<') {
                // Check if it's a closing tag (tokens are space-joined so `</` → `< /`)
                let saved_pos = self.pos;
                self.advance(1);
                self.skip_whitespace();
                if self.current_char() == Some('/') {
                    // Closing tag found, restore position and break
                    self.pos = saved_pos;
                    break;
                }
                // Not a closing tag, restore and parse as element
                self.pos = saved_pos;

                if !text.is_empty() {
                    children.push(RsxNode::Text(text.clone()));
                    text.clear();
                }

                let elem = self.parse_element()?;
                children.push(elem);
            } else if self.current_char() == Some('{') {
                // Dynamic expression
                if !text.is_empty() {
                    children.push(RsxNode::Text(text.clone()));
                    text.clear();
                }

                self.advance(1);
                let mut expr_str = String::new();
                let mut brace_count = 1;

                while brace_count > 0 && self.current_char().is_some() {
                    match self.current_char() {
                        Some('{') => {
                            brace_count += 1;
                            expr_str.push('{');
                        }
                        Some('}') => {
                            brace_count -= 1;
                            if brace_count > 0 {
                                expr_str.push('}');
                            }
                        }
                        Some(c) => {
                            expr_str.push(c);
                        }
                        None => break,
                    }
                    self.advance(1);
                }

                let expr: Expr = syn::parse_str(&expr_str)?;
                children.push(RsxNode::Dynamic(expr));
            } else if let Some(c) = self.current_char() {
                text.push(c);
                self.advance(1);
            } else {
                break;
            }
        }

        if !text.is_empty() {
            children.push(RsxNode::Text(text));
        }

        Ok(children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    // --- Rust-style parsing tests ---

    #[test]
    fn test_simple_rust_element() {
        let input = quote! {
            div { "hello" }
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
        let node = result.unwrap();
        match node {
            RsxNode::Element { tag, attrs, children } => {
                assert_eq!(tag, "div");
                assert!(attrs.is_empty());
                assert_eq!(children.len(), 1);
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_rust_style_named_attributes() {
        let input = quote! {
            div(class: "container", id: "main") { }
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
        let node = result.unwrap();
        match node {
            RsxNode::Element { tag, attrs, .. } => {
                assert_eq!(tag, "div");
                assert_eq!(attrs.len(), 2);
                // Verify first attribute is class
                if let (key, RsxAttr::String(val)) = &attrs[0] {
                    assert_eq!(key, "class");
                    assert!(val.contains("container"));
                }
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_rust_style_dynamic_attribute() {
        let input = quote! {
            button(on_click: {move || { count.set(1) }}) { "Click" }
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
        let node = result.unwrap();
        match node {
            RsxNode::Element { tag, attrs, .. } => {
                assert_eq!(tag, "button");
                assert!(attrs.iter().any(|(k, _)| k == "on_click"));
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_rust_style_text_node() {
        let input = quote! {
            span { "static text" }
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
        let node = result.unwrap();
        match node {
            RsxNode::Element { children, .. } => {
                assert_eq!(children.len(), 1);
                assert!(matches!(children[0], RsxNode::Text(_)));
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_rust_style_dynamic_text() {
        let input = quote! {
            span { "Count: " {count} }
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
        let node = result.unwrap();
        match node {
            RsxNode::Element { children, .. } => {
                assert!(children.iter().any(|c| matches!(c, RsxNode::Text(_))));
                assert!(children.iter().any(|c| matches!(c, RsxNode::Dynamic(_))));
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_rust_style_nested_elements() {
        let input = quote! {
            div {
                span { "inner" }
            }
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
        let node = result.unwrap();
        match node {
            RsxNode::Element { tag, children, .. } => {
                assert_eq!(tag, "div");
                assert!(children.iter().any(|c| {
                    matches!(c, RsxNode::Element { tag, .. } if tag == "span")
                }));
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_rust_style_multiple_nesting_levels() {
        let input = quote! {
            div {
                section {
                    article {
                        p { "deep" }
                    }
                }
            }
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
    }

    // --- HTML-style parsing tests ---

    #[test]
    fn test_html_style_simple_element() {
        let input = quote! {
            <div> hello </div>
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
        let node = result.unwrap();
        match node {
            RsxNode::Element { tag, .. } => {
                assert_eq!(tag, "div");
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_html_style_with_attributes() {
        let input = quote! {
            <div class="container" id="main"> content </div>
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
        let node = result.unwrap();
        match node {
            RsxNode::Element { tag, attrs, .. } => {
                assert_eq!(tag, "div");
                assert!(attrs.iter().any(|(k, _)| k == "class"));
                assert!(attrs.iter().any(|(k, _)| k == "id"));
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_html_style_self_closing() {
        let input = quote! {
            <input />
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
        let node = result.unwrap();
        match node {
            RsxNode::Element { tag, children, .. } => {
                assert_eq!(tag, "input");
                assert!(children.is_empty());
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_html_style_dynamic_text() {
        let input = quote! {
            <span> Count: {count} </span>
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_html_style_nested() {
        let input = quote! {
            <div>
                <span>nested</span>
            </div>
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
    }

    // --- Bidirectional equivalence ---

    #[test]
    fn test_rust_and_html_produce_similar_ast() {
        let rust_input = quote! {
            div(class: "btn") { "Click" }
        };
        let html_input = quote! {
            <div class="btn"> Click </div>
        };

        let rust_result = parse_rsx(rust_input).unwrap();
        let html_result = parse_rsx(html_input).unwrap();

        // Both should be Elements with same tag
        match (&rust_result, &html_result) {
            (
                RsxNode::Element { tag: t1, .. },
                RsxNode::Element { tag: t2, .. },
            ) => {
                assert_eq!(t1, t2);
                assert_eq!(t1, "div");
            }
            _ => panic!("both should be Elements"),
        }
    }

    #[test]
    fn test_complex_component_structure() {
        let input = quote! {
            div(class: "form") {
                "Form Title"
                input(type: "text", placeholder: "Enter name")
                button(on_click: {|_| handle_submit()}) { "Submit" }
            }
        };
        let result = parse_rsx(input);
        assert!(result.is_ok());
        let node = result.unwrap();
        match node {
            RsxNode::Element { children, .. } => {
                assert!(children.len() >= 2);
            }
            _ => panic!("expected Element"),
        }
    }
}
