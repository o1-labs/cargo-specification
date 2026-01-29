pub mod docusaurus;
pub mod mdbook;

use crate::build::MarkdownFlavor;

/// A trait for transforming markdown content based on the target flavor
pub trait MarkdownTransformer {
    fn transform(&self, content: &str) -> String;
}

/// Returns the appropriate transformer for the given markdown flavor
pub fn get_transformer(flavor: MarkdownFlavor) -> Box<dyn MarkdownTransformer> {
    match flavor {
        MarkdownFlavor::Mdbook => Box::new(mdbook::MdbookTransformer),
        MarkdownFlavor::Docusaurus => Box::new(docusaurus::DocusaurusTransformer),
    }
}
