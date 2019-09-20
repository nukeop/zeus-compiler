extern crate zeus_compiler;
use zeus_compiler::program::{Program, Version};
use zeus_compiler::source_file::SourceFile;

fn main() {
    let source_path = "examples/intro.source";
    let target_path = "intro.compiled.zeus";
    let mut source = SourceFile::new();
    source.load(source_path.to_string()).unwrap();
    source.tokenize().unwrap();
    let compiled = source.compile().unwrap();

    let version = Version::new(0, 1, 0);
    let mut program = Program::new(version);
    program.bytes = compiled;
    program.to_file(target_path.to_string()).expect(
        "Could not save the
        compiled program",
    );
}
