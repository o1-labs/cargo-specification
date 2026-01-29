use regex::Regex;

use super::MarkdownTransformer;

/// Docusaurus transformer - transforms admonitions, math, TOC markers, and
/// math underscores
pub struct DocusaurusTransformer;

impl MarkdownTransformer for DocusaurusTransformer {
    fn transform(&self, content: &str) -> String {
        let content = transform_admonitions(content);
        let content = transform_math_align(&content);
        let content = split_math_delimiters(&content);
        let content = remove_toc_markers(&content);
        transform_math_underscores(&content)
    }
}

/// Transform mdBook admonitions to Docusaurus format
/// ```` ```admonish warning ```` -> `:::warning`
/// ```` ```admonish ```` -> `:::note` (default type)
/// Closing ``` -> `:::`
fn transform_admonitions(content: &str) -> String {
    // Match ```admonish with optional type (defaults to "note")
    let re = Regex::new(r"^```admonish(?:\s+(\w+))?(?:\s.*)?$").unwrap();
    let mut result = String::new();
    let mut in_admonition = false;
    let mut admonition_indent = String::new();

    for line in content.lines() {
        let trimmed = line.trim_start();
        let current_indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();

        if let Some(caps) = re.captures(trimmed) {
            let admonition_type = caps.get(1).map_or("note", |m| m.as_str());
            in_admonition = true;
            admonition_indent = current_indent.clone();
            result.push_str(&format!("{}:::{}\n", current_indent, admonition_type));
        } else if in_admonition && trimmed == "```" && current_indent == admonition_indent {
            in_admonition = false;
            result.push_str(&format!("{}:::\n", current_indent));
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    // Remove trailing newline if original didn't have one
    if !content.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }

    result
}

/// Transform `\begin{align}` to `\begin{aligned}` and `\end{align}` to
/// `\end{aligned}` for KaTeX compatibility in Docusaurus
fn transform_math_align(content: &str) -> String {
    content
        .replace(r"\begin{align}", r"\begin{aligned}")
        .replace(r"\end{align}", r"\end{aligned}")
}

/// Split math block delimiters onto separate lines for remark-math compatibility
/// `$$\begin{aligned}` -> `$$\n\begin{aligned}`
/// `\end{aligned}$$` -> `\end{aligned}\n$$`
fn split_math_delimiters(content: &str) -> String {
    // Split opening: $$\begin{...} -> $$\n\begin{...}
    // Note: $$ in replacement string means literal $ in regex
    let re_open = Regex::new(r"\$\$\\begin\{").unwrap();
    let content = re_open.replace_all(content, "$$$$\n\\begin{").to_string();

    // Split closing: \end{...}$$ -> \end{...}\n$$
    let re_close = Regex::new(r"\\end\{([^}]+)\}\$\$").unwrap();
    re_close
        .replace_all(&content, "\\end{$1}\n$$$$")
        .to_string()
}

/// Remove `<!-- toc -->` markers used by mdBook for table of contents
fn remove_toc_markers(content: &str) -> String {
    // Remove the toc marker line (with optional leading spaces, not newlines)
    let re = Regex::new(r"(?m)^[ \t]*<!--[ \t]*toc[ \t]*-->[ \t]*\n?").unwrap();
    re.replace_all(content, "").to_string()
}

/// Transform escaped underscores in math blocks from `\_` to `_`
/// This handles both inline math `$...$` and display math `$$...$$`
fn transform_math_underscores(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut in_inline_math = false;
    let mut in_display_math = false;

    while let Some(c) = chars.next() {
        if c == '$' {
            if chars.peek() == Some(&'$') {
                // Display math delimiter
                chars.next();
                result.push_str("$$");
                in_display_math = !in_display_math;
            } else {
                // Inline math delimiter
                result.push('$');
                in_inline_math = !in_inline_math;
            }
        } else if (in_inline_math || in_display_math) && c == '\\' && chars.peek() == Some(&'_') {
            // Convert \_ to _ within math blocks
            chars.next();
            result.push('_');
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_admonitions_warning() {
        let input = "```admonish warning\nThis is a warning\n```";
        let expected = ":::warning\nThis is a warning\n:::";
        assert_eq!(transform_admonitions(input), expected);
    }

    #[test]
    fn test_transform_admonitions_info() {
        let input = "```admonish info\nSome information\n```";
        let expected = ":::info\nSome information\n:::";
        assert_eq!(transform_admonitions(input), expected);
    }

    #[test]
    fn test_transform_admonitions_plain_defaults_to_note() {
        let input = "```admonish\nThis is a plain admonition\n```";
        let expected = ":::note\nThis is a plain admonition\n:::";
        assert_eq!(transform_admonitions(input), expected);
    }

    #[test]
    fn test_transform_admonitions_with_title() {
        // mdBook-admonish supports titles like: ```admonish warning "Custom Title"
        let input = "```admonish warning \"Custom Title\"\nWarning with title\n```";
        let expected = ":::warning\nWarning with title\n:::";
        assert_eq!(transform_admonitions(input), expected);
    }

    #[test]
    fn test_transform_admonitions_all_types() {
        // Test all mdBook-admonish types
        let types = [
            "note",
            "abstract",
            "summary",
            "tldr",
            "info",
            "todo",
            "tip",
            "hint",
            "important",
            "success",
            "check",
            "done",
            "question",
            "help",
            "faq",
            "warning",
            "caution",
            "attention",
            "failure",
            "fail",
            "missing",
            "danger",
            "error",
            "bug",
            "example",
            "quote",
            "cite",
        ];

        for admonish_type in types {
            let input = format!("```admonish {}\nContent\n```", admonish_type);
            let expected = format!(":::{}\nContent\n:::", admonish_type);
            assert_eq!(
                transform_admonitions(&input),
                expected,
                "Failed for type: {}",
                admonish_type
            );
        }
    }

    #[test]
    fn test_transform_admonitions_with_surrounding_content() {
        let input = "Before\n\n```admonish warning\nWarning text\n```\n\nAfter";
        let expected = "Before\n\n:::warning\nWarning text\n:::\n\nAfter";
        assert_eq!(transform_admonitions(input), expected);
    }

    #[test]
    fn test_transform_admonitions_preserves_non_admonition_code_blocks() {
        let input = "```rust\nfn main() {}\n```";
        assert_eq!(transform_admonitions(input), input);
    }

    #[test]
    fn test_transform_math_align() {
        let input = r"$$
\begin{align}
a &= b \\
c &= d
\end{align}
$$";
        let expected = r"$$
\begin{aligned}
a &= b \\
c &= d
\end{aligned}
$$";
        assert_eq!(transform_math_align(input), expected);
    }

    #[test]
    fn test_split_math_delimiters_inline_block() {
        // This is the problematic pattern from kimchi
        let input = r"$$\begin{aligned}
    & z(x) \cdot zkpm(x)
\end{aligned}$$";
        let expected = r"$$
\begin{aligned}
    & z(x) \cdot zkpm(x)
\end{aligned}
$$";
        assert_eq!(split_math_delimiters(input), expected);
    }

    #[test]
    fn test_split_math_delimiters_already_split() {
        // Should not affect already properly formatted blocks
        let input = r"$$
\begin{aligned}
a &= b
\end{aligned}
$$";
        assert_eq!(split_math_delimiters(input), input);
    }

    #[test]
    fn test_split_math_delimiters_multiple_blocks() {
        let input = r"First block:
$$\begin{aligned}
a &= b
\end{aligned}$$

Second block:
$$\begin{aligned}
c &= d
\end{aligned}$$";
        let expected = r"First block:
$$
\begin{aligned}
a &= b
\end{aligned}
$$

Second block:
$$
\begin{aligned}
c &= d
\end{aligned}
$$";
        assert_eq!(split_math_delimiters(input), expected);
    }

    #[test]
    fn test_remove_toc_markers() {
        let input = "# Title\n\n<!-- toc -->\n\n## Section 1";
        let expected = "# Title\n\n\n## Section 1";
        assert_eq!(remove_toc_markers(input), expected);
    }

    #[test]
    fn test_remove_toc_markers_with_spaces() {
        let input = "# Title\n\n  <!--  toc  -->\n\n## Section 1";
        let expected = "# Title\n\n\n## Section 1";
        assert_eq!(remove_toc_markers(input), expected);
    }

    #[test]
    fn test_transform_math_underscores_inline() {
        let input = r"The variable $x\_1 + x\_2$ is important";
        let expected = r"The variable $x_1 + x_2$ is important";
        assert_eq!(transform_math_underscores(input), expected);
    }

    #[test]
    fn test_transform_math_underscores_display() {
        let input = r"$$
a\_1 + a\_2 = b
$$";
        let expected = r"$$
a_1 + a_2 = b
$$";
        assert_eq!(transform_math_underscores(input), expected);
    }

    #[test]
    fn test_transform_math_underscores_preserves_outside_math() {
        let input = r"Use snake\_case for variables. In math: $x\_1$";
        let expected = r"Use snake\_case for variables. In math: $x_1$";
        assert_eq!(transform_math_underscores(input), expected);
    }

    #[test]
    fn test_full_transform() {
        let transformer = DocusaurusTransformer;
        let input = r#"# Title

<!-- toc -->

```admonish warning
Be careful!
```

Inline math: $x\_1 + x\_2$

$$
\begin{align}
a &= b
\end{align}
$$
"#;
        let expected = r#"# Title


:::warning
Be careful!
:::

Inline math: $x_1 + x_2$

$$
\begin{aligned}
a &= b
\end{aligned}
$$
"#;
        assert_eq!(transformer.transform(input), expected);
    }
}
