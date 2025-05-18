use crate::writer::IndentedWriter;
use crate::{CodeGenOptions, NameCase, NameCasing};
use convert_case::Case;
use neatproto_ast::*;

#[derive(Debug, Default)]
pub struct RustCodeGenOptions {
    pub with_debug: bool,
    pub with_serde: bool,
    pub serde_struct_field_name_case: NameCase,
    pub serde_enum_repr: Option<String>,
}

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
        writer.write_indented_line("{");
        writer.push_indent();
    }

    for node in &block.nodes {
        match node {
            BlockNode::Block(block) => write_block(opts, writer, block, true),
            BlockNode::Structure(structure) => write_structure(opts, writer, structure),
            BlockNode::Alias(alias) => write_alias(opts, writer, alias),
            BlockNode::Enum(e) => write_enum(opts, writer, e),
        }
    }

    if with_brackets {
        writer.pop_indent();
        writer.write_indented_line("}");
    }
}

fn write_structure(opts: &CodeGenOptions, writer: &mut IndentedWriter, structure: &Structure) {
    write_structure_attributes(opts, writer, structure);

    writer.write_indented(format!(
        "pub struct {}",
        structure.name.to_name_case(opts.type_name_case)
    ));

    write_structure_body(opts, writer, structure, true);
}

fn write_structure_attributes(
    opts: &CodeGenOptions,
    writer: &mut IndentedWriter,
    _structure: &Structure,
) {
    if opts.rust.with_debug {
        writer.write_indented_line("#[derive(Debug)]");
    }
    if opts.rust.with_serde {
        writer.write_indented_line("#[derive(Serialize, Deserialize)]");
        write_serde_rename_all_attr(opts, writer);
    }
}

fn write_structure_body(
    opts: &CodeGenOptions,
    writer: &mut IndentedWriter,
    structure: &Structure,
    with_access_modifiers: bool,
) {
    writer.write_line(" {");
    writer.push_indent();

    for field in &structure.fields {
        writer.write_indent();

        if with_access_modifiers {
            writer.write("pub ");
        }

        writer.write_string_line(format!(
            "{}: {},",
            field.name.to_name_case(opts.field_name_case),
            get_full_type_name(opts, &field.type_name)
        ));
    }

    writer.pop_indent();
    writer.write_indented("}");
}

fn write_alias(opts: &CodeGenOptions, writer: &mut IndentedWriter, alias: &Alias) {
    writer.write_indented_line(format!(
        "pub type {} = {};",
        alias.alias_name.to_name_case(opts.type_name_case),
        translate_type_name(opts, &alias.aliased_type_name)
    ));
}

fn write_enum(opts: &CodeGenOptions, writer: &mut IndentedWriter, e: &Enum) {
    let is_tagged_union = e.items.iter().any(|item| item.structure.is_some());
    if is_tagged_union {
        write_tagged_union(opts, writer, e);
        return;
    }

    writer.write_indented_line("#[derive(Clone, Copy, PartialEq, Eq)]");

    if opts.rust.with_debug {
        writer.write_indented_line("#[derive(Debug)]");
    }

    if opts.rust.with_serde {
        if let Some(repr) = &opts.rust.serde_enum_repr {
            writer.write_indented_line("#[derive(Serialize_repr, Deserialize_repr)]");
            writer.write_indented_line(format!("#[repr({})]", repr));
        } else {
            writer.write_indented_line("#[derive(Serialize, Deserialize)]");
        }

        write_serde_rename_all_attr(opts, writer);
    }

    writer.write_indented_line(format!(
        "pub enum {} {{",
        e.name.to_name_case(opts.type_name_case)
    ));
    writer.push_indent();

    for item in &e.items {
        writer.write_indented(item.name.to_name_case(opts.enum_item_name_case));

        if let Some(value_token) = &item.value_token {
            writer.write(format!(" = {}", value_token.token.value()).as_str());
        }

        writer.write_line(",");
    }

    writer.pop_indent();
    writer.write_indented_line("}");
}

fn write_tagged_union(opts: &CodeGenOptions, writer: &mut IndentedWriter, e: &Enum) {
    if opts.rust.with_debug {
        writer.write_indented_line("#[derive(Debug)]");
    }

    if opts.rust.with_serde {
        writer.write_indented_line("#[derive(Serialize, Deserialize)]");
        writer.write_indented_line("#[serde(tag = \"kind\", content = \"value\")]");
        write_serde_rename_all_attr(opts, writer);
    }

    writer.write_indented_line(format!(
        "pub enum {} {{",
        e.name.to_name_case(opts.type_name_case)
    ));
    writer.push_indent();

    for item in &e.items {
        if opts.rust.with_serde {
            write_serde_rename_all_attr(opts, writer);
        }

        writer.write_indented(item.name.to_name_case(opts.enum_item_name_case));

        if let Some(structure) = &item.structure {
            write_structure_body(opts, writer, structure, false);
        }

        writer.write_line(",");
    }

    writer.pop_indent();
    writer.write_indented_line("}");
}

fn write_serde_rename_all_attr(opts: &CodeGenOptions, writer: &mut IndentedWriter) {
    if let NameCase::Other(case) = opts.rust.serde_struct_field_name_case {
        writer.write_indented_line(format!(
            "#[serde(rename_all = \"{}\")]",
            map_case_to_serde(&case).expect("invalid `serde_struct_field_name_case`")
        ));
    }
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

fn get_full_type_name(opts: &CodeGenOptions, type_name: &TypeName) -> String {
    let name = translate_type_name(opts, &type_name.token.value());
    if type_name.is_array {
        if let Some(size) = &type_name.array_size {
            return format!("[{}; {}]", name, size.value());
        }
        return format!("Vec<{}>", name);
    }
    name
}

fn translate_type_name(opts: &CodeGenOptions, type_name: &String) -> String {
    BuiltinTypeName::parse(type_name)
        .map(|t| translate_builtin_type_name(t).to_string())
        .unwrap_or_else(|| type_name.to_name_case(opts.type_name_case))
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
