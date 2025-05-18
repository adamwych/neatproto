use crate::LocalizedToken;

#[derive(Debug)]
pub struct TypeName {
    pub token: LocalizedToken,
    pub is_array: bool,
    pub array_size: Option<LocalizedToken>,
    pub resolved: Option<String>,
}

#[derive(Debug)]
pub enum BuiltinTypeName {
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Float32,
    Float64,
    Bool,
    String,
}

impl BuiltinTypeName {
    pub fn parse(string: &str) -> Option<Self> {
        match string {
            "uint8" => Some(BuiltinTypeName::Uint8),
            "uint16" => Some(BuiltinTypeName::Uint16),
            "uint32" => Some(BuiltinTypeName::Uint32),
            "uint64" => Some(BuiltinTypeName::Uint64),
            "uint128" => Some(BuiltinTypeName::Uint128),
            "int8" => Some(BuiltinTypeName::Int8),
            "int16" => Some(BuiltinTypeName::Int16),
            "int32" => Some(BuiltinTypeName::Int32),
            "int64" => Some(BuiltinTypeName::Int64),
            "int128" => Some(BuiltinTypeName::Int128),
            "float" => Some(BuiltinTypeName::Float32),
            "float64" => Some(BuiltinTypeName::Float64),
            "bool" => Some(BuiltinTypeName::Bool),
            "string" => Some(BuiltinTypeName::String),
            _ => None,
        }
    }
}
