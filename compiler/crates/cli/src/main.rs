use clap::{Args, Parser, Subcommand};
use neatproto_codegen::CodeGenOptions;
use neatproto_compiler::{CompileError, compile_nproto_to_file, compile_nproto_to_string};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Compile(CompileCommandArgs),
}

#[derive(Args, Debug)]
struct CompileCommandArgs {
    input: String,
    output: Option<String>,
}

fn compile(args: CompileCommandArgs) -> Result<(), CompileError> {
    let options = CodeGenOptions::default();
    match args.output {
        Some(output) => compile_nproto_to_file(args.input.into(), output.into(), &options),
        None => {
            compile_nproto_to_string(args.input.into(), &options).map(|code| println!("{}", code))
        }
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Compile(args) => {
            if let Err(err) = compile(args) {
                println!("{}", err);
            }
        }
    }
}
