use criterion::{Criterion, black_box, criterion_group, criterion_main};
use neatproto_codegen::CodeGenOptions;
use neatproto_compiler::{CompilationUnit, SourceFile};

fn compile_file(source_file: &SourceFile) {
    CompilationUnit::new(source_file)
        .compile(&CodeGenOptions::default())
        .expect("failed to compile");
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut str = String::with_capacity(1024);
    for i in 0..1 {
        str.push_str(format!("struct Struct{} {{\n", i).as_str());
        for j in 0..1 {
            str.push_str(format!("    field{}: float;\n", j).as_str());
        }
        str.push_str("}\n\n");
    }

    let source_file = SourceFile::new_from_source("local".to_string(), str.to_string());

    c.bench_function("random structs", |b| {
        b.iter(|| compile_file(black_box(&source_file)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
