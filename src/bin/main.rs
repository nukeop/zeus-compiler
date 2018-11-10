#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate zeus_compiler;
use zeus_compiler::init::read_args;
use zeus_compiler::ops::Op;
use zeus_compiler::source_file::SourceFile;

fn main() {
    pretty_env_logger::init();
    info!("Zeus Entertainment System compiler starting up...");
    let args = read_args();

    let target = args.value_of("target").unwrap_or("program.z");
    info!("Target file: {}", target);

    let mut source = SourceFile::new();
    source.load(target.to_string()).unwrap();

    let lines = source.lines.unwrap();
    for mut line in lines {
        let compiled = line.to_compiled(); 
        println!("{:?}", compiled);
    }
}
