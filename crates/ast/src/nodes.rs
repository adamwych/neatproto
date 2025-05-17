use crate::TypeName;

#[derive(Debug)]
pub struct Block {
    pub nodes: Vec<BlockNode>,
}

#[derive(Debug)]
pub enum BlockNode {
    Block(Block),
    Structure(Structure),
    Alias(Alias),
    Enum(Enum),
}

#[derive(Debug)]
pub struct Structure {
    pub name: String,
    pub fields: Vec<StructureField>,
}

#[derive(Debug)]
pub struct StructureField {
    pub name: String,
    pub type_name: TypeName,
}

#[derive(Debug)]
pub struct Alias {
    pub alias_name: String,
    pub aliased_type_name: String,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub items: Vec<EnumItem>,
}

#[derive(Debug)]
pub struct EnumItem {
    pub name: String,
}
