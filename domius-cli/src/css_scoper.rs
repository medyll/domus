//! Scoped CSS transformer for Domius components.
//!
//! Each component gets a unique `data-domius-{hash}` attribute. This module:
//! 1. Generates a short deterministic hash from (path, content)
//! 2. Transforms CSS selectors to scoped versions
//!
//! **Input:**
//! ```css
//! .btn { color: red; }
//! .icon { width: 16px; }
//! ```
//!
//! **Output** (with hash `"a3f2"`):
//! ```css
//! [data-domius="a3f2"] .btn { color: red; }
//! [data-domius="a3f2"] .icon { width: 16px; }
//! ```

// ---------------------------------------------------------------------------
// Hash generation (FNV-1a, no external crates)
// ---------------------------------------------------------------------------

/// Generate a short deterministic hex hash from `path` and `content`.
///
/// Uses FNV-1a 64-bit hash for speed and good distribution.
/// Returns the first 8 hex characters (32 bits of entropy — sufficient for
/// scoping uniqueness within a typical component tree).
pub fn generate_scope_hash(path: &str, content: &str) -> String {
    const FNV_OFFSET: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;

    let mut hash: u64 = FNV_OFFSET;
    for byte in path.bytes().chain(b"|".iter().copied()).chain(content.bytes()) {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    // Return first 8 hex chars (truncate to 32 bits for brevity)
    format!("{:08x}", (hash & 0xFFFF_FFFF) as u32)
}

// ---------------------------------------------------------------------------
// CSS scoping
// ---------------------------------------------------------------------------

/// Scope attribute selector inserted before each rule's selectors.
pub fn scope_attr(hash: &str) -> String {
    format!("[data-domius=\"{}\"]", hash)
}

/// Transform `css` so every rule is scoped to `scope_hash`.
///
/// Handles:
/// - Single and multiple rules
/// - Multiple comma-separated selectors: `.a, .b { }` → `[data-domius=…] .a, [data-domius=…] .b { }`
/// - `@media` / `@keyframes` blocks (passed through unchanged — not scoped)
/// - Comments stripped to avoid misparses
///
/// Does **not** support:
/// - Nested CSS (CSS Nesting spec) — not needed for MVP
/// - `:root`, `:host` rewriting — left as-is
pub fn scope_css(css: &str, scope_hash: &str) -> String {
    let attr = scope_attr(scope_hash);
    let mut output = String::new();
    let mut chars = css.chars().peekable();
    let mut depth: usize = 0;
    let mut current_selector = String::new();
    let mut in_at_rule = false;

    while let Some(ch) = chars.next() {
        match ch {
            // Skip /* ... */ comments
            '/' if chars.peek() == Some(&'*') => {
                chars.next(); // consume '*'
                loop {
                    match chars.next() {
                        Some('*') if chars.peek() == Some(&'/') => {
                            chars.next(); // consume '/'
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                }
            }

            // At-rule start
            '@' if depth == 0 => {
                in_at_rule = true;
                current_selector.push('@');
            }

            // Opening brace
            '{' => {
                depth += 1;
                if depth == 1 {
                    if in_at_rule {
                        // At-rules: emit as-is
                        output.push_str(&current_selector);
                        output.push('{');
                    } else {
                        // Regular rule: scope each selector
                        let scoped = scope_selector_list(current_selector.trim(), &attr);
                        output.push_str(&scoped);
                        output.push(' ');
                        output.push('{');
                    }
                    current_selector.clear();
                } else {
                    output.push('{');
                }
            }

            // Closing brace
            '}' => {
                if depth > 0 {
                    depth -= 1;
                }
                output.push('}');
                if depth == 0 {
                    in_at_rule = false;
                    output.push('\n');
                }
            }

            // Everything else
            _ => {
                if depth == 0 {
                    current_selector.push(ch);
                } else {
                    output.push(ch);
                }
            }
        }
    }

    // Clean up extra blank lines
    output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Prefix every selector in a comma-separated selector list with `attr`.
///
/// `.a, .b` → `[data-domius="x"] .a, [data-domius="x"] .b`
fn scope_selector_list(selectors: &str, attr: &str) -> String {
    selectors
        .split(',')
        .map(|sel| {
            let sel = sel.trim();
            if sel.is_empty() {
                String::new()
            } else {
                format!("{} {}", attr, sel)
            }
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Hash generation ---

    #[test]
    fn test_hash_is_8_hex_chars() {
        let h = generate_scope_hash("src/Button.rs", ".btn { color: red; }");
        assert_eq!(h.len(), 8);
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_same_inputs_produce_same_hash() {
        let h1 = generate_scope_hash("path/comp.rs", "content");
        let h2 = generate_scope_hash("path/comp.rs", "content");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_different_paths_produce_different_hash() {
        let h1 = generate_scope_hash("path/A.rs", "content");
        let h2 = generate_scope_hash("path/B.rs", "content");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_different_content_produces_different_hash() {
        let h1 = generate_scope_hash("path/comp.rs", ".btn { color: red }");
        let h2 = generate_scope_hash("path/comp.rs", ".btn { color: blue }");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_empty_inputs_hash() {
        let h = generate_scope_hash("", "");
        assert_eq!(h.len(), 8);
    }

    // --- Selector scoping ---

    #[test]
    fn test_scope_single_rule() {
        let css = ".btn { color: red; }";
        let scoped = scope_css(css, "abc1");
        assert!(scoped.contains("[data-domius=\"abc1\"] .btn"));
        assert!(scoped.contains("color: red;"));
    }

    #[test]
    fn test_scope_multiple_rules() {
        let css = ".btn { color: red; } .icon { width: 16px; }";
        let scoped = scope_css(css, "abc1");
        assert!(scoped.contains("[data-domius=\"abc1\"] .btn"));
        assert!(scoped.contains("[data-domius=\"abc1\"] .icon"));
    }

    #[test]
    fn test_scope_comma_separated_selectors() {
        let css = ".btn, .icon { display: flex; }";
        let scoped = scope_css(css, "x1y2");
        assert!(scoped.contains("[data-domius=\"x1y2\"] .btn"));
        assert!(scoped.contains("[data-domius=\"x1y2\"] .icon"));
    }

    #[test]
    fn test_scope_element_selectors() {
        let css = "h1 { font-size: 2rem; } p { margin: 0; }";
        let scoped = scope_css(css, "1a2b");
        assert!(scoped.contains("[data-domius=\"1a2b\"] h1"));
        assert!(scoped.contains("[data-domius=\"1a2b\"] p"));
    }

    #[test]
    fn test_scope_class_and_element_combined() {
        let css = ".container h2 { color: blue; }";
        let scoped = scope_css(css, "ff00");
        assert!(scoped.contains("[data-domius=\"ff00\"] .container h2"));
    }

    #[test]
    fn test_scope_preserves_property_values() {
        let css = ".card { background: #fff; border: 1px solid rgba(0,0,0,.1); }";
        let scoped = scope_css(css, "test");
        assert!(scoped.contains("background: #fff"));
        assert!(scoped.contains("border: 1px solid rgba(0,0,0,.1)"));
    }

    #[test]
    fn test_at_media_passed_through() {
        let css = "@media (max-width: 768px) { .btn { display: none; } }";
        let scoped = scope_css(css, "abc1");
        assert!(scoped.contains("@media"));
        assert!(scoped.contains("max-width: 768px"));
    }

    #[test]
    fn test_scope_attr_format() {
        assert_eq!(scope_attr("a1b2"), "[data-domius=\"a1b2\"]");
    }

    #[test]
    fn test_scope_empty_css() {
        let scoped = scope_css("", "abc1");
        assert!(scoped.is_empty());
    }

    #[test]
    fn test_scope_comment_stripped() {
        let css = "/* header */ .title { font-size: 1rem; }";
        let scoped = scope_css(css, "00ff");
        assert!(scoped.contains("[data-domius=\"00ff\"] .title"));
        assert!(!scoped.contains("/* header */"));
    }

    #[test]
    fn test_scope_real_component_styles() {
        let css = r#"
            .wrapper { display: flex; flex-direction: column; gap: 8px; }
            .header { font-weight: bold; color: #333; }
            .footer, .caption { font-size: 0.8rem; color: #999; }
        "#;
        let hash = generate_scope_hash("src/Card.rs", css);
        let scoped = scope_css(css, &hash);
        let attr = format!("[data-domius=\"{}\"]", hash);
        assert!(scoped.contains(&format!("{} .wrapper", attr)));
        assert!(scoped.contains(&format!("{} .header", attr)));
        assert!(scoped.contains(&format!("{} .footer", attr)));
        assert!(scoped.contains(&format!("{} .caption", attr)));
    }
}
