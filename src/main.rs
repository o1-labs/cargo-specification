use cargo_spec::{
    build::{build, watch, MarkdownFlavor, OutputFormat},
    init::{init, new, DEFAULT_MANIFEST, DEFAULT_TEMPLATE},
    toml_parser,
};
use clap::{Args, Parser, Subcommand};
use miette::Result;
use std::path::PathBuf;

/// To make cargo subcommands work, you need to use `bin_name`,
/// as well as a subcommand.
#[derive(Debug, Parser)]
#[clap(bin_name = "cargo")]
#[clap(author, version, about)]
enum Cli {
    #[clap(subcommand)]
    Spec(Spec),
}

/// The struct that represents the default command of `cargo spec`
#[derive(Debug, Subcommand)]
enum Spec {
    /// Creates the necessary files to start a specification in a folder
    New { name: String },

    /// Creates the necessary files to start a specification in an existing directory
    Init {
        #[clap(value_name = "SPEC_DIR")]
        path: PathBuf,
    },

    /// Create the specification file at the given path.
    Build(Opt),

    /// Watches any listed files in the specification toml file and
    /// re-create the specification on any changes.
    Watch(Opt),
}

/// The different options that can be passed to this CLI
#[derive(Args, Debug)]
#[clap(author, version, about, bin_name = "cargo")]
struct Opt {
    /// The path to the specification toml file (defaults to Specification.toml).
    #[clap(short, long, value_name = "SPEC_PATH")]
    specification_path: Option<PathBuf>,

    /// The path to the specification file to write
    /// (defaults to specification.md or specification.html)
    #[clap(short, long, value_name = "OUTPUT_FILE")]
    output_file: Option<PathBuf>,

    /// The output format (defaults to markdown)
    #[clap(short = 'f', long, value_name = "OUTPUT_FORMAT")]
    #[clap(value_enum)]
    output_format: Option<OutputFormat>,

    /// The markdown flavor to use (mdbook or docusaurus)
    #[clap(short = 'F', long, value_name = "MARKDOWN_FLAVOR")]
    #[clap(value_enum)]
    flavor: Option<MarkdownFlavor>,
}

/// Resolve the markdown flavor from CLI, config file, or default.
/// Priority: CLI flag > config file > default (Mdbook)
fn resolve_flavor(
    cli_flavor: Option<MarkdownFlavor>,
    toml_spec: &std::path::Path,
) -> Result<MarkdownFlavor> {
    // CLI flag takes precedence
    if let Some(flavor) = cli_flavor {
        return Ok(flavor);
    }

    // Try to read from config file
    if toml_spec.exists() {
        let spec = toml_parser::parse_toml_spec(toml_spec)?;
        if let Some(output_config) = spec.output {
            if let Some(flavor_str) = output_config.flavor {
                return match flavor_str.to_lowercase().as_str() {
                    "mdbook" => Ok(MarkdownFlavor::Mdbook),
                    "docusaurus" => Ok(MarkdownFlavor::Docusaurus),
                    _ => Ok(MarkdownFlavor::default()),
                };
            }
        }
    }

    // Default
    Ok(MarkdownFlavor::default())
}

fn main() -> Result<()> {
    //~ 1. parse command-line arguments
    let Cli::Spec(args) = Cli::parse();

    //~ 2. depending on the mode:
    match args {
        Spec::New { name } => {
            new(name)?;

            println!("Created new specification as {DEFAULT_MANIFEST} and {DEFAULT_TEMPLATE}");
            println!("You can now run `cargo spec build` to create the specification file");
        }

        Spec::Init { path } => {
            init(None, path.clone())?;

            println!("Created new specification at {path}/{DEFAULT_MANIFEST} and {path}/{DEFAULT_TEMPLATE}", path = path.display());
            println!(
                "You can now run `cd {path} && cargo spec build` to create the specification file",
                path = path.display()
            );
        }

        //~   a. the `Build` mode builds the specification
        Spec::Build(Opt {
            specification_path,
            output_file,
            output_format,
            flavor,
        }) => {
            let toml_spec = specification_path.unwrap_or_else(|| PathBuf::from(DEFAULT_MANIFEST));
            let output_format = output_format.unwrap_or(OutputFormat::Markdown);
            let flavor = resolve_flavor(flavor, &toml_spec)?;

            let _ = build(toml_spec, output_file, output_format, flavor)?;
        }

        //~   b. the `Watch` mode builds the specification on every change
        Spec::Watch(Opt {
            specification_path,
            output_file,
            output_format,
            flavor,
        }) => {
            let toml_spec = specification_path.unwrap_or_else(|| PathBuf::from(DEFAULT_MANIFEST));
            let output_format = output_format.unwrap_or(OutputFormat::Markdown);
            let flavor = resolve_flavor(flavor, &toml_spec)?;

            watch(toml_spec, output_format, output_file, flavor);
        }
    };

    Ok(())
}
