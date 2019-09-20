#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate zeus_compiler;
use zeus_compiler::init::read_args;
use zeus_compiler::program::{Program, Version};
use zeus_compiler::source_file::SourceFile;

fn main() {
    pretty_env_logger::init();
    info!("Zeus Entertainment System compiler starting up...");
    let args = read_args();

    let source_file = args.value_of("source").unwrap_or("source.zeus");
    info!("Source file: {}", source_file);

    let target = args.value_of("target").unwrap_or("program.zeus");
    info!("Target file: {}", target);

    let mut source = SourceFile::new();
    source.load(source_file.to_string()).unwrap();
    source.tokenize().unwrap();
    let compiled = source.compile().unwrap();

    let version = Version::new(0, 1, 0);
    let mut program = Program::new(version);
    program.bytes = compiled;
    program.to_file(target.to_string()).expect(
        "Could not save the
        compiled program",
    );
}
