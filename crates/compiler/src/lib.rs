pub mod error;
pub mod unit;

use neatproto_codegen::CodeGenOptions;
pub use neatproto_parser::SourceFile;
use std::path::PathBuf;
pub use unit::*;
pub use error::*;

pub fn compile_nproto_to_string(
    input_file_path: PathBuf,
    code_gen_opts: &CodeGenOptions,
) -> Result<String, CompileError> {
    let source_file =
        SourceFile::new_from_path(input_file_path).expect("failed to read input file");
    let unit = CompilationUnit::new(&source_file);
    unit.compile(code_gen_opts)
}

pub fn compile_nproto_to_file(
    input_file_path: PathBuf,
    output_file_path: PathBuf,
    code_gen_opts: &CodeGenOptions,
) -> Result<(), CompileError> {
    let result = compile_nproto_to_string(input_file_path, code_gen_opts)?;

    let output_file_path_buf = output_file_path.to_path_buf();
    let output_file_parent_dir = output_file_path_buf
        .parent()
        .expect("output file's parent directory is invalid");
    std::fs::create_dir_all(output_file_parent_dir)
        .expect("failed to create output file parent directory");
    std::fs::write(output_file_path_buf, result)
        .expect("failed to write generated code to output file");

    Ok(())
}
