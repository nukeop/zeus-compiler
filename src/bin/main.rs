#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate zeus_compiler;
use zeus_compiler::init::read_args;
use zeus_compiler::ops::Op;
use zeus_compiler::program::Program;
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

    let mut program = Program::new();
    let lines = source.lines.unwrap();
    for mut line in lines {
        let compiled = line.to_compiled().unwrap(); 
        for byte in compiled {
            program.bytes.push(byte);
        }
    }

    program.to_file(target.to_string()).expect("Could not save the
compiled program");
}
