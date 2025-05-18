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

#[derive(Default)]
pub struct CodeGenOptions {
    pub field_name_case: NameCase,
    pub type_name_case: NameCase,
    pub enum_item_name_case: NameCase,
    pub rust: RustCodeGenOptions,
}

#[derive(Default)]
pub struct RustCodeGenOptions {
    pub with_debug: bool,
    pub with_serde: bool,
    pub serde_struct_field_name_case: NameCase,
    pub serde_enum_repr: Option<String>,
}
