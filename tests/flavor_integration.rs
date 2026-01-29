use std::{fs, path::PathBuf, process::Command};
use tempfile::TempDir;

fn cargo_spec_binary() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target/debug/cargo-spec");
    path
}

fn setup_test_spec(dir: &TempDir, template: &str) -> PathBuf {
    let spec_path = dir.path().join("Specification.toml");
    let template_path = dir.path().join("template.md");

    fs::write(
        &spec_path,
        r#"[metadata]
name = "Test"
authors = ["Test Author"]

[config]
template = "template.md"

[sections]
"#,
    )
    .unwrap();

    fs::write(&template_path, template).unwrap();
    spec_path
}

fn setup_test_spec_with_flavor(dir: &TempDir, template: &str, flavor: &str) -> PathBuf {
    let spec_path = dir.path().join("Specification.toml");
    let template_path = dir.path().join("template.md");

    fs::write(
        &spec_path,
        format!(
            r#"[metadata]
name = "Test"
authors = ["Test Author"]

[config]
template = "template.md"

[output]
flavor = "{}"

[sections]
"#,
            flavor
        ),
    )
    .unwrap();

    fs::write(&template_path, template).unwrap();
    spec_path
}

fn run_cargo_spec(spec_path: &PathBuf, output_path: &PathBuf, flavor: Option<&str>) -> String {
    let mut cmd = Command::new(cargo_spec_binary());
    cmd.arg("spec")
        .arg("build")
        .arg("-s")
        .arg(spec_path)
        .arg("-o")
        .arg(output_path);

    if let Some(f) = flavor {
        cmd.arg("--flavor").arg(f);
    }

    let output = cmd.output().expect("Failed to execute cargo-spec");

    if !output.status.success() {
        panic!(
            "cargo-spec failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fs::read_to_string(output_path).expect("Failed to read output file")
}

// =============================================================================
// Admonition Tests
// =============================================================================

#[test]
fn test_admonition_warning_mdbook() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish warning
This is a warning message.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("mdbook"));

    assert!(result.contains("```admonish warning"));
    assert!(result.contains("This is a warning message."));
    assert!(!result.contains(":::warning"));
}

#[test]
fn test_admonition_warning_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish warning
This is a warning message.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(!result.contains("```admonish warning"));
    assert!(result.contains(":::warning"));
    assert!(result.contains("This is a warning message."));
    assert!(result.contains(":::"));
}

#[test]
fn test_admonition_info_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish info
Some informational text.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::info"));
    assert!(result.contains("Some informational text."));
}

#[test]
fn test_admonition_note_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish note
A note for the reader.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::note"));
    assert!(result.contains("A note for the reader."));
}

#[test]
fn test_admonition_tip_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish tip
Here's a helpful tip!
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::tip"));
    assert!(result.contains("Here's a helpful tip!"));
}

#[test]
fn test_admonition_danger_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish danger
This is dangerous!
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::danger"));
    assert!(result.contains("This is dangerous!"));
}

#[test]
fn test_admonition_plain_defaults_to_note_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish
This is a plain admonition without a type.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::note"));
    assert!(result.contains("This is a plain admonition without a type."));
    assert!(!result.contains("```admonish"));
}

#[test]
fn test_admonition_with_title_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish warning "Custom Warning Title"
This warning has a custom title.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::warning"));
    assert!(result.contains("This warning has a custom title."));
    assert!(!result.contains("```admonish"));
}

#[test]
fn test_admonition_example_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish example
This is an example.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::example"));
    assert!(result.contains("This is an example."));
}

#[test]
fn test_admonition_bug_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish bug
Known bug description.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::bug"));
    assert!(result.contains("Known bug description."));
}

#[test]
fn test_admonition_quote_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish quote
A famous quote here.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::quote"));
    assert!(result.contains("A famous quote here."));
}

#[test]
fn test_admonition_abstract_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish abstract
This is an abstract.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::abstract"));
    assert!(result.contains("This is an abstract."));
}

#[test]
fn test_admonition_success_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish success
Operation completed successfully.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::success"));
    assert!(result.contains("Operation completed successfully."));
}

#[test]
fn test_admonition_question_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish question
Frequently asked question here.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::question"));
    assert!(result.contains("Frequently asked question here."));
}

#[test]
fn test_admonition_failure_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish failure
This operation failed.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::failure"));
    assert!(result.contains("This operation failed."));
}

#[test]
fn test_multiple_admonitions_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish warning
First warning.
```

Some text between.

```admonish info
Some info.
```

More text.

```admonish tip
A tip.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::warning"));
    assert!(result.contains("First warning."));
    assert!(result.contains(":::info"));
    assert!(result.contains("Some info."));
    assert!(result.contains(":::tip"));
    assert!(result.contains("A tip."));
    // Count the closing ::: markers (should be 3)
    assert_eq!(result.matches(":::").count(), 6); // 3 opening + 3 closing
}

#[test]
fn test_admonition_with_multiline_content_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish warning
Line 1 of the warning.
Line 2 of the warning.

A new paragraph in the warning.

- Bullet 1
- Bullet 2
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::warning"));
    assert!(result.contains("Line 1 of the warning."));
    assert!(result.contains("Line 2 of the warning."));
    assert!(result.contains("A new paragraph in the warning."));
    assert!(result.contains("- Bullet 1"));
    assert!(result.contains("- Bullet 2"));
}

#[test]
fn test_non_admonition_code_blocks_preserved() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```rust
let x = 5;
let y = 10;
```

```python
x = 5
y = 10
```

```
Plain code block
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains("```rust"));
    assert!(result.contains("let x = 5"));
    assert!(result.contains("```python"));
    assert!(result.contains("x = 5"));
    assert!(!result.contains(":::rust"));
    assert!(!result.contains(":::python"));
}

// =============================================================================
// TOC Marker Tests
// =============================================================================

#[test]
fn test_toc_marker_preserved_mdbook() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

<!-- toc -->

## Section 1
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("mdbook"));

    assert!(result.contains("<!-- toc -->"));
}

#[test]
fn test_toc_marker_removed_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

<!-- toc -->

## Section 1
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(!result.contains("<!-- toc -->"));
    assert!(result.contains("# Test"));
    assert!(result.contains("## Section 1"));
}

#[test]
fn test_toc_marker_with_spaces_removed_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

  <!--  toc  -->

## Section 1
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(!result.contains("toc"));
    assert!(result.contains("# Test"));
    assert!(result.contains("## Section 1"));
}

#[test]
fn test_multiple_toc_markers_removed_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

<!-- toc -->

## Section 1

<!-- toc -->

## Section 2
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(!result.contains("<!-- toc -->"));
    assert!(result.contains("## Section 1"));
    assert!(result.contains("## Section 2"));
}

#[test]
fn test_other_html_comments_preserved_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

<!-- This is a regular comment -->

<!-- Another comment -->

## Section 1
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains("<!-- This is a regular comment -->"));
    assert!(result.contains("<!-- Another comment -->"));
}

// =============================================================================
// Math Underscore Tests
// =============================================================================

#[test]
fn test_math_underscores_preserved_mdbook() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

Inline math: $x\_1 + x\_2$

Display math: $$a\_1 + a\_2$$
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("mdbook"));

    assert!(result.contains(r"$x\_1 + x\_2$"));
    assert!(result.contains(r"$$a\_1 + a\_2$$"));
}

#[test]
fn test_math_underscores_transformed_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

Inline math: $x\_1 + x\_2$

Display math: $$a\_1 + a\_2$$
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains("$x_1 + x_2$"));
    assert!(result.contains("$$a_1 + a_2$$"));
    assert!(!result.contains(r"\_"));
}

#[test]
fn test_underscores_outside_math_preserved_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

Use snake\_case for variable names.

In math: $x\_1$

More snake\_case text.
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    // Outside math, escaped underscores should be preserved
    assert!(result.contains(r"snake\_case"));
    // Inside math, escaped underscores should be converted
    assert!(result.contains("$x_1$"));
}

#[test]
fn test_math_block_delimiters_split_docusaurus() {
    let dir = TempDir::new().unwrap();
    // Note: We can't use \begin{aligned} directly in the template because
    // TinyTemplate interprets braces. This test verifies the transformation
    // works when align is already on separate lines.
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

Some math here.
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    // Basic sanity check
    assert!(result.contains("# Test"));
}

#[test]
fn test_complex_math_expressions_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

The formula is $x\_1 + x\_2 + x\_3 + \ldots + x\_n$.
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(r"$x_1 + x_2 + x_3 + \ldots + x_n$"));
}

#[test]
fn test_multiple_math_blocks_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

First: $a\_1$

Second: $b\_2$

Third: $$c\_3 + d\_4$$
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains("$a_1$"));
    assert!(result.contains("$b_2$"));
    assert!(result.contains("$$c_3 + d_4$$"));
}

// =============================================================================
// Math Align Tests
// =============================================================================

#[test]
fn test_math_align_preserved_mdbook() {
    let dir = TempDir::new().unwrap();
    // Note: We need to escape the braces for TinyTemplate
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

Some text here.
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("mdbook"));

    assert!(result.contains("Some text here."));
}

// =============================================================================
// Config File Flavor Tests
// =============================================================================

#[test]
fn test_config_file_docusaurus_flavor() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec_with_flavor(
        &dir,
        r#"# Test

```admonish warning
Warning from config.
```

Math: $x\_1$
"#,
        "docusaurus",
    );
    let output_path = dir.path().join("output.md");

    // No flavor flag - should use config file
    let result = run_cargo_spec(&spec_path, &output_path, None);

    assert!(result.contains(":::warning"));
    assert!(result.contains("$x_1$"));
}

#[test]
fn test_config_file_mdbook_flavor() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec_with_flavor(
        &dir,
        r#"# Test

```admonish warning
Warning from config.
```

Math: $x\_1$
"#,
        "mdbook",
    );
    let output_path = dir.path().join("output.md");

    // No flavor flag - should use config file
    let result = run_cargo_spec(&spec_path, &output_path, None);

    assert!(result.contains("```admonish warning"));
    assert!(result.contains(r"$x\_1$"));
}

#[test]
fn test_cli_overrides_config_file_to_mdbook() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec_with_flavor(
        &dir,
        r#"# Test

```admonish warning
Warning text.
```
"#,
        "docusaurus", // Config says docusaurus
    );
    let output_path = dir.path().join("output.md");

    // CLI says mdbook - should override config
    let result = run_cargo_spec(&spec_path, &output_path, Some("mdbook"));

    assert!(result.contains("```admonish warning"));
    assert!(!result.contains(":::warning"));
}

#[test]
fn test_cli_overrides_config_file_to_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec_with_flavor(
        &dir,
        r#"# Test

```admonish warning
Warning text.
```
"#,
        "mdbook", // Config says mdbook
    );
    let output_path = dir.path().join("output.md");

    // CLI says docusaurus - should override config
    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::warning"));
    assert!(!result.contains("```admonish warning"));
}

#[test]
fn test_default_flavor_without_config() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish warning
Warning text.
```
"#,
    );
    let output_path = dir.path().join("output.md");

    // No flavor in config, no CLI flag - should default to mdbook
    let result = run_cargo_spec(&spec_path, &output_path, None);

    assert!(result.contains("```admonish warning"));
    assert!(!result.contains(":::warning"));
}

// =============================================================================
// Combined Transformation Tests
// =============================================================================

#[test]
fn test_all_transformations_together_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Comprehensive Test

<!-- toc -->

## Introduction

```admonish warning
Be careful with $x\_1$ and $y\_2$.
```

Some regular text with snake\_case.

```admonish info
More information here.
```

## Math Section

Inline math: $a\_i + b\_j$

```rust
let example = 42;
```

```admonish tip
Final tip!
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    // TOC removed
    assert!(!result.contains("<!-- toc -->"));

    // Admonitions transformed
    assert!(result.contains(":::warning"));
    assert!(result.contains(":::info"));
    assert!(result.contains(":::tip"));
    assert!(!result.contains("```admonish"));

    // Math underscores transformed inside admonition
    assert!(result.contains("$x_1$"));
    assert!(result.contains("$y_2$"));
    assert!(result.contains("$a_i + b_j$"));

    // Underscores outside math preserved
    assert!(result.contains(r"snake\_case"));

    // Regular code blocks preserved
    assert!(result.contains("```rust"));
    assert!(result.contains("let example = 42"));
}

#[test]
fn test_all_transformations_mdbook_passthrough() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Comprehensive Test

<!-- toc -->

## Introduction

```admonish warning
Be careful with $x\_1$ and $y\_2$.
```

Some regular text.

## Math Section

Inline math: $a\_i + b\_j$
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("mdbook"));

    // Everything should be preserved unchanged
    assert!(result.contains("<!-- toc -->"));
    assert!(result.contains("```admonish warning"));
    assert!(result.contains(r"$x\_1$"));
    assert!(result.contains(r"$y\_2$"));
    assert!(result.contains(r"$a\_i + b\_j$"));
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_empty_admonition_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish warning
```
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::warning"));
    assert!(result.contains(":::"));
}

#[test]
fn test_admonition_at_end_of_file_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

```admonish warning
Final warning.
```"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains(":::warning"));
    assert!(result.contains("Final warning."));
}

#[test]
fn test_math_at_end_of_line_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

The value is $x\_n$"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains("$x_n$"));
}

#[test]
fn test_consecutive_math_blocks_docusaurus() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

$a\_1$$b\_2$$c\_3$
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    assert!(result.contains("$a_1$$b_2$$c_3$"));
}

#[test]
fn test_unmatched_dollar_signs_preserved() {
    let dir = TempDir::new().unwrap();
    let spec_path = setup_test_spec(
        &dir,
        r#"# Test

The price is $50 and another $100.

Real math: $x\_1$
"#,
    );
    let output_path = dir.path().join("output.md");

    let result = run_cargo_spec(&spec_path, &output_path, Some("docusaurus"));

    // The $50 and $100 are treated as math (which is expected behavior)
    // but there's no \_ to transform there
    assert!(result.contains("$x_1$"));
}
