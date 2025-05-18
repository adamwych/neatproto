use crate::CompileError;
use neatproto_analysis::analyze_block;
use neatproto_codegen::{CodeGenOptions, generate_code};
use neatproto_parser::{SourceFile, parse_block};

pub struct CompilationUnit<'a> {
    root_source_file: &'a SourceFile,
}

impl<'a> CompilationUnit<'a> {
    pub fn new(root_source_file: &'a SourceFile) -> Self {
        Self { root_source_file }
    }

    pub fn compile(self, codegen_opts: &CodeGenOptions) -> Result<String, CompileError> {
        match parse_block(&mut self.root_source_file.tokens()) {
            Ok(mut root_block) => {
                analyze_block(&mut root_block);
                Ok(generate_code(codegen_opts, &root_block))
            }
            Err(error) => Err(CompileError::Parse(error)),
        }
    }
}
