//! Utility for conditionally joining class names.
//!
//! Similar to the popular `classnames` npm package.

/// Join class names conditionally.
///
/// # Example
/// ```ignore
/// let classes = class_names(&[
///     Some("btn"),
///     Some(if disabled { "btn-disabled" } else { "" }),
///     Some(if primary { "btn-primary" } else { "" }),
/// ]);
/// // Result: "btn btn-disabled btn-primary"
/// ```
pub fn class_names(classes: &[Option<&str>]) -> String {
    classes
        .iter()
        .filter_map(|c| *c)
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Macro for more ergonomic class name construction.
///
/// # Example
/// ```ignore
/// let classes = cn!("btn", disabled.then_some("btn-disabled"), primary.then_some("btn-primary"));
/// ```
#[macro_export]
macro_rules! cn {
    ($($class:expr),* $(,)?) => {{
        let mut classes = Vec::new();
        $(
            if let Some(c) = $class {
                if !c.is_empty() {
                    classes.push(c);
                }
            }
        )*
        classes.join(" ")
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_names_basic() {
        let result = class_names(&[Some("btn"), Some("btn-primary")]);
        assert_eq!(result, "btn btn-primary");
    }

    #[test]
    fn test_class_names_filters_empty() {
        let result = class_names(&[Some("btn"), Some(""), None, Some("active")]);
        assert_eq!(result, "btn active");
    }

    #[test]
    fn test_class_names_conditional() {
        let disabled = true;
        let primary = false;
        let result = class_names(&[
            Some("btn"),
            Some(if disabled { "btn-disabled" } else { "" }),
            Some(if primary { "btn-primary" } else { "" }),
        ]);
        assert_eq!(result, "btn btn-disabled");
    }

    #[test]
    fn test_class_names_all_empty() {
        let result = class_names(&[Some(""), None, Some("")]);
        assert_eq!(result, "");
    }
}
