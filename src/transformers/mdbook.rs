use super::MarkdownTransformer;

/// mdBook transformer - passthrough that preserves content unchanged
pub struct MdbookTransformer;

impl MarkdownTransformer for MdbookTransformer {
    fn transform(&self, content: &str) -> String {
        content.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passthrough() {
        let transformer = MdbookTransformer;
        let input = r#"
```admonish warning
This is a warning
```

Math: $x\_1 + x\_2$

<!-- toc -->

$$
\begin{align}
a &= b \\
c &= d
\end{align}
$$
"#;
        assert_eq!(transformer.transform(input), input);
    }
}
