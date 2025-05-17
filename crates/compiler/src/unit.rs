use crate::CompileError;
use neatproto_analysis::analyze_block;
use neatproto_codegen::{CodeGenOptions, generate_rust};
use neatproto_parser::{Parser, SourceFile};

pub struct CompilationUnit<'a> {
    root_source_file: &'a SourceFile,
}

impl<'a> CompilationUnit<'a> {
    pub fn new(root_source_file: &'a SourceFile) -> Self {
        Self { root_source_file }
    }

    pub fn compile(self, codegen_opts: &CodeGenOptions) -> Result<String, CompileError> {
        let mut parser = Parser::new(self.root_source_file.tokens());
        match parser.parse() {
            Ok(mut root_block) => {
                analyze_block(&mut root_block);
                Ok(generate_rust(codegen_opts, &root_block))
            }
            Err(error) => Err(CompileError::Parse(error)),
        }
    }
}
