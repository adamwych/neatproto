pub mod csharp;
pub mod rust;
mod writer;

pub use crate::csharp::CSharpCodeGenOptions;
use crate::csharp::generate_csharp;
pub use crate::rust::RustCodeGenOptions;
use crate::rust::generate_rust;
pub use convert_case::{Case, Casing};
use neatproto_ast::Block;

#[derive(Debug, Default, Copy, Clone)]
pub enum NameCase {
    #[default]
    Unchanged,
    Other(Case<'static>),
}

pub trait NameCasing<T: AsRef<str>> {
    fn to_name_case(&self, case: NameCase) -> String;
}

impl<T: AsRef<str> + ToString> NameCasing<T> for T {
    fn to_name_case(&self, case: NameCase) -> String {
        match case {
            NameCase::Unchanged => self.to_string(),
            NameCase::Other(case) => self.to_case(case),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, clap::ValueEnum)]
pub enum TargetLanguage {
    #[default]
    Rust,
    Csharp,
}

#[derive(Debug, Default)]
pub struct CodeGenOptions {
    pub target_language: TargetLanguage,
    pub field_name_case: NameCase,
    pub type_name_case: NameCase,
    pub enum_item_name_case: NameCase,

    pub rust: RustCodeGenOptions,
    pub csharp: CSharpCodeGenOptions,
}

pub fn generate_code(opts: &CodeGenOptions, root_block: &Block) -> String {
    match opts.target_language {
        TargetLanguage::Rust => generate_rust(opts, root_block),
        TargetLanguage::Csharp => generate_csharp(opts, root_block),
    }
}
