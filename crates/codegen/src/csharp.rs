use crate::writer::IndentedWriter;
use crate::{CodeGenOptions, NameCasing};
use convert_case::{Case, Casing};
use neatproto_ast::*;

#[derive(Debug)]
pub struct CSharpCodeGenOptions {
    pub namespace: String,
    pub with_json_convert_for_union_tags: bool,
}

impl Default for CSharpCodeGenOptions {
    fn default() -> Self {
        Self {
            namespace: "MyNamespace".into(),
            with_json_convert_for_union_tags: true,
        }
    }
}

pub fn generate_csharp(opts: &CodeGenOptions, root_block: &Block) -> String {
    let mut writer = IndentedWriter::default();

    writer.write_string_line("using NeatProto;");
    writer.next_line();
    writer.write_string_line(format!("namespace {};", opts.csharp.namespace));
    writer.next_line();

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
    writer.write_indented(format!(
        "public class {}",
        structure.name.to_name_case(opts.type_name_case)
    ));

    writer.next_line();
    writer.write_indented_line("{");
    writer.push_indent();

    write_structure_body(opts, writer, structure, true);

    writer.pop_indent();
    writer.write_indented("}");
    writer.next_line();
}

fn write_structure_body(
    opts: &CodeGenOptions,
    writer: &mut IndentedWriter,
    structure: &Structure,
    with_access_modifiers: bool,
) {
    for field in &structure.fields {
        writer.write_indented_line("[global::System.ComponentModel.DataAnnotations.Required]");
        writer.write_indent();
        if with_access_modifiers {
            writer.write("public required ");
        }

        let field_full_type_name = get_full_type_name(opts, &field.type_name);
        let field_name = field.name.to_name_case(opts.field_name_case);

        writer.write_string_line(format!(
            "{field_full_type_name} {field_name} {{ get; set; }}"
        ));
    }
}

fn write_alias(opts: &CodeGenOptions, writer: &mut IndentedWriter, alias: &Alias) {
    let alias_name = alias.alias_name.to_name_case(opts.type_name_case);
    let aliased_type_name = translate_type_name(opts, &alias.aliased_type_name);

    writer.write_indented_line(format!(
        "[global::System.Text.Json.Serialization.JsonConverter(typeof({}JsonConverter))]",
        alias_name
    ));
    writer.write_line(
        format!(
            r#"
public record struct {alias_name}({aliased_type_name} Value)
{{
    public override string ToString() => Value.ToString();

    public static implicit operator {aliased_type_name}({alias_name} alias) => alias.Value;
    public static implicit operator {alias_name}({aliased_type_name} alias) => new(alias);
}}
    "#
        )
        .trim(),
    );

    writer.write_indented(
        format!(
            r#"
public class {alias_name}JsonConverter : global::System.Text.Json.Serialization.JsonConverter<{alias_name}>
{{
    public override void Write(
        global::System.Text.Json.Utf8JsonWriter writer,
        {alias_name} aliasType,
        global::System.Text.Json.JsonSerializerOptions options)
    {{
        writer.WriteValue(aliasType.Value);
    }}

    public override {alias_name} Read(
        ref global::System.Text.Json.Utf8JsonReader reader,
        global::System.Type typeToConvert,
        global::System.Text.Json.JsonSerializerOptions options)
    {{
        reader.Get(out {aliased_type_name} value);
        return value;
    }}
}}"#) .trim()
    );

    writer.next_line();
}

fn write_enum(opts: &CodeGenOptions, writer: &mut IndentedWriter, e: &Enum) {
    let is_tagged_union = e.items.iter().any(|item| item.structure.is_some());
    if is_tagged_union {
        write_tagged_union(opts, writer, e);
        return;
    }

    writer.write_indented_line(format!(
        "public enum {} {{",
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
    let enum_class_name = e.name.to_name_case(opts.type_name_case);
    let discriminator_class_name = format!("{}Type", enum_class_name);

    // --
    // Generate a separate class for each item, but put all of them inside an abstract class,
    // which basically acts as a namespace.
    writer.write_indented_line(format!("public abstract class {}", enum_class_name));
    writer.write_indented_line("{");
    writer.push_indent();

    for item in &e.items {
        writer.write_indented(format!(
            "public class {} : {}, global::NeatProto.ITaggedUnionCase<{}>",
            item.name.to_name_case(opts.enum_item_name_case),
            enum_class_name,
            discriminator_class_name
        ));

        writer.next_line();
        writer.write_indented_line("{");
        writer.push_indent();

        writer.write_indented_line(format!(
            "public static {} Kind => {}.{};",
            discriminator_class_name,
            discriminator_class_name,
            item.name.to_case(Case::Camel)
        ));

        if let Some(structure) = &item.structure {
            write_structure_body(opts, writer, structure, true);
        }

        writer.pop_indent();
        writer.write_indented_line("}");
    }

    writer.next_line();
    writer.pop_indent();
    writer.write_indented_line("}");

    // --
    // Generate class representing the discriminator.
    if opts.csharp.with_json_convert_for_union_tags {
        writer.write_indented_line(format!(
            "[global::System.Text.Json.Serialization.JsonConverter(typeof({}JsonConverter))]",
            discriminator_class_name
        ));
    }

    writer.write_indented_line(format!(
        "public readonly record struct {}(string Value)",
        discriminator_class_name
    ));
    writer.write_indented_line("{");
    writer.push_indent();

    for item in &e.items {
        writer.write_indented_line(format!(
            "public static readonly {} {} = new(\"{}\");",
            discriminator_class_name,
            item.name.to_case(Case::Camel),
            item.name.to_case(Case::Camel),
        ));
    }

    writer.pop_indent();
    writer.write_indented_line("}");

    // --
    // Generate JSON converter for the discriminator struct.
    if opts.csharp.with_json_convert_for_union_tags {
        writer.write_indented(
            format!(
                r#"
public class {discriminator_class_name}JsonConverter : global::System.Text.Json.Serialization.JsonConverter<{discriminator_class_name}>
{{
    public override void Write(
        global::System.Text.Json.Utf8JsonWriter writer,
        {discriminator_class_name} messageType,
        global::System.Text.Json.JsonSerializerOptions options)
    {{
        writer.WriteStringValue(messageType.Value);
    }}

    public override {discriminator_class_name} Read(
        ref global::System.Text.Json.Utf8JsonReader reader,
        global::System.Type typeToConvert,
        global::System.Text.Json.JsonSerializerOptions options)
    {{
        if (reader.TokenType != global::System.Text.Json.JsonTokenType.String)
            throw new global::System.Text.Json.JsonException($"Expected string token, but got {{reader.TokenType}}");

        var value = reader.GetString();
        return value switch
        {{
    "#)
            .trim(),
        );

        writer.next_line();
        writer.push_indent();
        writer.push_indent();
        writer.push_indent();

        for item in &e.items {
            writer.write_indented_line(format!(
                "nameof({}.{}) => {}.{},",
                discriminator_class_name,
                item.name.to_case(Case::Camel),
                discriminator_class_name,
                item.name.to_case(Case::Camel),
            ));
        }

        writer.pop_indent();
        writer.pop_indent();
        writer.pop_indent();

        writer.write_indented(
            r#"
            _ => throw new global::System.Text.Json.JsonException($"Unknown type: {value}")
        };
    }"#,
        );

        writer.next_line();
        writer.pop_indent();
        writer.write_indented_line("}");
    }
}

fn get_full_type_name(opts: &CodeGenOptions, type_name: &TypeName) -> String {
    let name = translate_type_name(opts, &type_name.token.value());
    if type_name.is_array {
        if let Some(_) = &type_name.array_size {
            return format!("{}[]", name);
        }
        return format!("global::System.Collections.Generic.List<{}>", name);
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
        BuiltinTypeName::Uint8 => "byte",
        BuiltinTypeName::Uint16 => "ushort",
        BuiltinTypeName::Uint32 => "uint",
        BuiltinTypeName::Uint64 => "ulong",
        BuiltinTypeName::Uint128 => "global::System.UInt128",
        BuiltinTypeName::Int8 => "sbyte",
        BuiltinTypeName::Int16 => "short",
        BuiltinTypeName::Int32 => "int",
        BuiltinTypeName::Int64 => "long",
        BuiltinTypeName::Int128 => "global::System.Int128",
        BuiltinTypeName::Float32 => "float",
        BuiltinTypeName::Float64 => "double",
        BuiltinTypeName::Bool => "bool",
        BuiltinTypeName::String => "string",
    }
}
