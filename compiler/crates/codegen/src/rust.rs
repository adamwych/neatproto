use crate::writer::IndentedWriter;
use crate::{CodeGenOptions, NameCase};
use convert_case::Case;
use neatproto_ast::{Alias, Block, BlockNode, BuiltinTypeName, Structure};

pub fn generate_rust(opts: &CodeGenOptions, root_block: &Block) -> String {
    let mut writer = IndentedWriter::default();
    write_block(opts, &mut writer, root_block, false);
    writer.to_string()
}

fn write_block(
    opts: &CodeGenOptions,
    writer: &mut IndentedWriter,
    block: &Block,
    with_brackets: bool,
) {
    if with_brackets {
        writer.write_line("{");
        writer.push_indent();
    }

    for node in &block.nodes {
        match node {
            BlockNode::Block(block) => write_block(opts, writer, block, true),
            BlockNode::Structure(structure) => write_structure(opts, writer, structure),
            BlockNode::Alias(alias) => write_alias(opts, writer, alias),
        }
    }

    if with_brackets {
        writer.pop_indent();
        writer.write_line("}");
    }
}

fn write_structure(opts: &CodeGenOptions, writer: &mut IndentedWriter, structure: &Structure) {
    if opts.rust.with_debug {
        writer.write_line("#[derive(Debug)]");
    }
    if opts.rust.with_serde {
        writer.write_line("#[derive(Serialize, Deserialize)]");
    }
    match opts.rust.serde_field_name_case {
        NameCase::Other(case) => {
            if let Some(serde_name) = map_case_to_serde(&case) {
                writer.write_line(format!("#[serde(rename_all = \"{}\")]", serde_name));
            }
        }
        _ => (),
    }

    writer.write_line(format!(
        "pub struct {} {{",
        opts.type_name_case.format(&structure.name)
    ));
    writer.push_indent();

    for field in &structure.fields {
        writer.write_line(format!(
            "pub {}: {},",
            opts.field_name_case.format(&field.name),
            translate_type_name(opts, &field.type_name.token.value())
        ));
    }

    writer.pop_indent();
    writer.write_line("}");
}

fn write_alias(opts: &CodeGenOptions, writer: &mut IndentedWriter, alias: &Alias) {
    writer.write_line(format!(
        "pub type {} = {};",
        opts.type_name_case.format(&alias.alias_name),
        translate_type_name(opts, &alias.aliased_type_name)
    ));
}

/// Maps `Case` enum to a value that `#[serde(rename_all = ???)]` supports.
fn map_case_to_serde(case: &Case) -> Option<&'static str> {
    match case {
        Case::Lower => Some("lowercase"),
        Case::Upper => Some("UPPERCASE"),
        Case::Pascal => Some("PascalCase"),
        Case::Camel => Some("camelCase"),
        Case::Snake => Some("snake_case"),
        Case::UpperSnake => Some("SCREAMING_SNAKE_CASE"),
        Case::Kebab => Some("kebab-case"),
        Case::UpperKebab => Some("SCREAMING-KEBAB-CASE"),
        _ => None,
    }
}

fn translate_type_name(opts: &CodeGenOptions, type_name: &String) -> String {
    BuiltinTypeName::parse(type_name)
        .map(|t| translate_builtin_type_name(t).to_string())
        .unwrap_or_else(|| opts.type_name_case.format(&type_name))
}

fn translate_builtin_type_name(type_name: BuiltinTypeName) -> &'static str {
    match type_name {
        BuiltinTypeName::Uint8 => "u8",
        BuiltinTypeName::Uint16 => "u16",
        BuiltinTypeName::Uint32 => "u32",
        BuiltinTypeName::Uint64 => "u64",
        BuiltinTypeName::Uint128 => "u128",
        BuiltinTypeName::Int8 => "i8",
        BuiltinTypeName::Int16 => "i16",
        BuiltinTypeName::Int32 => "i32",
        BuiltinTypeName::Int64 => "i64",
        BuiltinTypeName::Int128 => "i128",
        BuiltinTypeName::Float32 => "f32",
        BuiltinTypeName::Float64 => "f64",
        BuiltinTypeName::Bool => "bool",
        BuiltinTypeName::String => "String",
    }
}
