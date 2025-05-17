use neatproto_ast::{Alias, Block, BlockNode, Enum, Structure, TypeName};

pub fn analyze_block(block: &mut Block) {
    for node in &mut block.nodes {
        match node {
            BlockNode::Block(block) => analyze_block(block),
            BlockNode::Structure(structure) => visit_structure(structure),
            BlockNode::Alias(alias) => visit_alias(alias),
            BlockNode::Enum(e) => visit_enum(e),
        }
    }
}

fn visit_structure(structure: &mut Structure) {
    for field in &mut structure.fields {
        field.type_name.resolved = resolve_type_name(&field.type_name);
    }
}

fn visit_alias(_alias: &mut Alias) {}

fn visit_enum(_enum: &mut Enum) {}

fn resolve_type_name(_type_name: &TypeName) -> Option<String> {
    None
}
