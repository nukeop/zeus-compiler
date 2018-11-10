use clap::{App,Arg, ArgMatches};


pub fn read_args<'a>() -> ArgMatches<'a> {
    App::new("zeus-compiler")
        .version("0.1.0")
        .about("Compiler for the Zeus entertainment system assembly language")
        .author("nukeop <nukeop@gumblert.tech>")
        .arg(Arg::with_name("source")
             .short("s")
             .long("source")
             .value_name("FILE")
             .help("Program to compile to binary form")
             .required(true))
        .arg(Arg::with_name("target")
             .short("t")
             .long("target")
             .value_name("FILE")
             .help("Where to save the compiled program")
             .required(true))
        .get_matches()
}
