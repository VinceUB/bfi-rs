extern crate brainfuck as bf;

use bf::{
    Simple,
    interpreter::Interpreter
};
use std::fs::File;
use std::io;
use clap::{App, Arg};


fn main() {
    let matches = App::new("Brainfuck interpreter")
        .author("Vince <vince@ultrabanana.net>")
        .about("Interprets brainfuck code")
        .arg(
            Arg::new("code")
            .short('f')
            .long("file")
            .takes_value(true)
            .help("The code to input"))
        .get_matches();
    
    let code_file = File::open(matches.value_of("code").unwrap()).expect("Could not open code file");

    let mut bf: Simple = Simple::new(code_file, io::stdin(), io::stdout());

    bf.interpret();
}
