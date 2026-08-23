//! URL pattern matching and client-side routing for Domus.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Pattern segment types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Segment {
    /// Literal path segment, e.g. `user` in `/user/:id`.
    Exact(String),
    /// Named parameter, e.g. `:id`.
    Param(String),
    /// Catch-all wildcard `*`.
    Wildcard,
}

// ---------------------------------------------------------------------------
// Route pattern
// ---------------------------------------------------------------------------

/// A compiled URL route pattern supporting `/`, `/path`, `/:param`, and `*`.
#[derive(Debug, Clone)]
pub struct RoutePattern {
    raw: String,
    segments: Vec<Segment>,
}

impl RoutePattern {
    /// Compile a pattern string into a `RoutePattern`.
    pub fn new(pattern: &str) -> Self {
        let segments = if pattern == "*" {
            vec![Segment::Wildcard]
        } else {
            pattern
                .split('/')
                .filter(|s| !s.is_empty())
                .map(|seg| {
                    if seg.starts_with(':') {
                        Segment::Param(seg[1..].to_string())
                    } else {
                        Segment::Exact(seg.to_string())
                    }
                })
                .collect()
        };

        Self {
            raw: pattern.to_string(),
            segments,
        }
    }

    /// Try to match `path` against this pattern.
    /// Returns `Some(params)` on match, where `params` holds named captures.
    pub fn matches(&self, path: &str) -> Option<HashMap<String, String>> {
        // Wildcard matches everything
        if self.segments == [Segment::Wildcard] {
            return Some(HashMap::new());
        }

        let path_segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if path_segs.len() != self.segments.len() {
            // Special case: root "/" → both are empty
            if self.segments.is_empty() && path_segs.is_empty() {
                return Some(HashMap::new());
            }
            return None;
        }

        let mut params = HashMap::new();

        for (pattern_seg, path_seg) in self.segments.iter().zip(path_segs.iter()) {
            match pattern_seg {
                Segment::Exact(s) => {
                    if s != path_seg {
                        return None;
                    }
                }
                Segment::Param(name) => {
                    params.insert(name.clone(), path_seg.to_string());
                }
                Segment::Wildcard => {
                    // Wildcard handled above; shouldn't appear in segment list
                    return Some(params);
                }
            }
        }

        Some(params)
    }

    /// The original pattern string.
    pub fn pattern(&self) -> &str {
        &self.raw
    }
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

/// Registered route entry.
struct RouteEntry<H> {
    pattern: RoutePattern,
    handler: H,
}

/// Client-side router.
///
/// Route matching is pure Rust and can be used in tests without WASM.
/// DOM navigation methods are gated behind `#[cfg(target_arch = "wasm32")]`.
pub struct Router<H> {
    routes: Vec<RouteEntry<H>>,
}

impl<H> Router<H> {
    /// Create an empty router.
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Register a route pattern with its handler.
    pub fn register(&mut self, pattern: &str, handler: H) {
        self.routes.push(RouteEntry {
            pattern: RoutePattern::new(pattern),
            handler,
        });
    }

    /// Find the first matching route for `path`.
    /// Returns a reference to the handler and the extracted params.
    pub fn match_route(&self, path: &str) -> Option<(&H, HashMap<String, String>)> {
        for entry in &self.routes {
            if let Some(params) = entry.pattern.matches(path) {
                return Some((&entry.handler, params));
            }
        }
        None
    }
}

impl<H> Default for Router<H> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- RoutePattern::matches ---

    #[test]
    fn test_root_matches_root() {
        let p = RoutePattern::new("/");
        assert!(p.matches("/").is_some());
    }

    #[test]
    fn test_root_does_not_match_path() {
        let p = RoutePattern::new("/");
        assert!(p.matches("/user").is_none());
    }

    #[test]
    fn test_exact_match() {
        let p = RoutePattern::new("/about");
        assert!(p.matches("/about").is_some());
        assert!(p.matches("/contact").is_none());
    }

    #[test]
    fn test_exact_multi_segment() {
        let p = RoutePattern::new("/blog/post");
        assert!(p.matches("/blog/post").is_some());
        assert!(p.matches("/blog").is_none());
    }

    #[test]
    fn test_param_extraction() {
        let p = RoutePattern::new("/user/:id");
        let result = p.matches("/user/42").unwrap();
        assert_eq!(result.get("id").unwrap(), "42");
    }

    #[test]
    fn test_multiple_params() {
        let p = RoutePattern::new("/org/:org/repo/:repo");
        let result = p.matches("/org/acme/repo/domus").unwrap();
        assert_eq!(result.get("org").unwrap(), "acme");
        assert_eq!(result.get("repo").unwrap(), "domus");
    }

    #[test]
    fn test_param_does_not_match_empty() {
        let p = RoutePattern::new("/user/:id");
        assert!(p.matches("/user").is_none());
    }

    #[test]
    fn test_wildcard_matches_any_path() {
        let p = RoutePattern::new("*");
        assert!(p.matches("/anything").is_some());
        assert!(p.matches("/deep/path/here").is_some());
        assert!(p.matches("/").is_some());
    }

    #[test]
    fn test_wildcard_returns_empty_params() {
        let p = RoutePattern::new("*");
        let params = p.matches("/any").unwrap();
        assert!(params.is_empty());
    }

    // --- Router ---

    #[test]
    fn test_router_matches_registered_route() {
        let mut router: Router<&str> = Router::new();
        router.register("/", "home");
        router.register("/user/:id", "user");

        assert!(router.match_route("/").is_some());
        assert!(router.match_route("/user/42").is_some());
    }

    #[test]
    fn test_router_returns_handler() {
        let mut router: Router<&str> = Router::new();
        router.register("/about", "about_handler");

        let (handler, _) = router.match_route("/about").unwrap();
        assert_eq!(*handler, "about_handler");
    }

    #[test]
    fn test_router_returns_params() {
        let mut router: Router<&str> = Router::new();
        router.register("/user/:id", "user_handler");

        let (_, params) = router.match_route("/user/99").unwrap();
        assert_eq!(params.get("id").unwrap(), "99");
    }

    #[test]
    fn test_router_no_match_returns_none() {
        let mut router: Router<&str> = Router::new();
        router.register("/", "home");
        router.register("/about", "about");

        assert!(router.match_route("/notfound").is_none());
    }

    #[test]
    fn test_router_wildcard_as_fallback() {
        let mut router: Router<&str> = Router::new();
        router.register("/", "home");
        router.register("*", "not_found");

        let (handler, _) = router.match_route("/unknown-page").unwrap();
        assert_eq!(*handler, "not_found");
    }

    #[test]
    fn test_router_first_match_wins() {
        let mut router: Router<&str> = Router::new();
        router.register("/page", "first");
        router.register("/page", "second");

        let (handler, _) = router.match_route("/page").unwrap();
        assert_eq!(*handler, "first");
    }

    #[test]
    fn test_router_empty_returns_none() {
        let router: Router<&str> = Router::new();
        assert!(router.match_route("/").is_none());
    }
}
