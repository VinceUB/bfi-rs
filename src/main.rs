extern crate brainfuck as bf;

use bf::{
    interpreter::{Interpreter, SimpleStringInterpreter, SimpleFileInterpreter}
};
use std::fs::File;
use std::io;
use std::io::Read;
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
    
    let mut code_file = File::open(matches.value_of("code").unwrap()).expect("Could not open code file");

    let code: &mut String = &mut String::new();
    code_file.read_to_string(code).expect("Couldn't read code file");

    let mut bf: SimpleStringInterpreter = SimpleStringInterpreter::new(code.to_owned(), io::stdin(), io::stdout());

    //let mut bf: SimpleFileInterpreter = SimpleFileInterpreter::new(code_file, io::stdin(), io::stdout());

    bf.interpret();
}
