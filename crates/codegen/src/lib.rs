pub mod rust;
mod writer;

pub use convert_case::{Case, Casing};
pub use rust::*;

#[derive(Default, Copy, Clone)]
pub enum NameCase {
    #[default]
    Unchanged,
    Other(Case<'static>),
}

impl NameCase {
    pub fn format(self, name: &String) -> String {
        match self {
            NameCase::Unchanged => name.clone(),
            NameCase::Other(case) => name.to_case(case),
        }
    }
}

#[derive(Default)]
pub struct CodeGenOptions {
    pub field_name_case: NameCase,
    pub type_name_case: NameCase,
    pub rust: RustCodeGenOptions,
}

#[derive(Default)]
pub struct RustCodeGenOptions {
    pub with_debug: bool,
    pub with_serde: bool,
    pub serde_field_name_case: NameCase,
}
