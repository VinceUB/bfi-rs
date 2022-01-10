use crate::{
    interpreter::Interpreter,
    machine::{Machine, SimpleMachine}
};
use std::{
    fs::File,
    io::{Stdin, Stdout, Seek, BufReader, SeekFrom, Read, Write, ErrorKind},
    num::Wrapping
};

pub struct SimpleFileInterpreter{
    code: BufReader<File>,
    output: Stdout,
    input: Stdin,
    loop_list: Vec<u64>,
    machine: SimpleMachine
}

impl Interpreter for SimpleFileInterpreter {
    fn interpret(&mut self){
        let mut codebuf: [u8;1] = [0];
        loop {
            let _strpos = self.code.stream_position().expect("No");
            match self.code.read_exact(&mut codebuf){
                Ok(_) => (),
                Err(e) => {
                    match e.kind() {
                        ErrorKind::UnexpectedEof => break,
                        _ => panic!("Failed to read code:\n{}", e)
                    }
                }
            };

            match codebuf[0]{
                b'+' => self.machine.inc(),
                b'-' => self.machine.dec(),
                b'<' => self.machine.lft(),
                b'>' => self.machine.rgt(),
                b'.' => {
                    self.output.write_all(&[self.machine.get().0])
                    .expect("Failed to write to output");
                    self.output.flush().expect("Failed to flush output");
                },
                b',' => {
                    let mut readbuf: [u8;1] = [0];
                    self.input.read_exact(&mut readbuf).expect("Could not read from input");
                    self.machine.set(Wrapping(readbuf[0]));
                }
                b'[' => self.start_loop(),
                b']' => self.end_loop(),
                _ => ()
            };
        }
    }
}

impl SimpleFileInterpreter{
    pub fn new(code: File, input: Stdin, output: Stdout) -> Self{
        Self{
            code: BufReader::new(code),
            output: output,
            input: input,
            loop_list: Vec::new(),
            machine: SimpleMachine::new()
        }
    }

	fn start_loop(&mut self){
		if self.machine.get() == Wrapping(0){
			let mut loop_delta = 0;

			loop {
				let buf: &mut [u8; 1] = &mut [0];

				match self.code.read_exact(buf) {
					Ok(_) => (),
					Err(e) => {
						match e.kind() {
							ErrorKind::UnexpectedEof => panic!("Reached end of code while searching for ]"),
							_ => panic!("Failed to scan for ]:\n{}", e)
						}
					}
				}

				let c = buf[0];

				if c == b'[' {
					loop_delta += 1;
				} 
				else if c == b']' && loop_delta >0 {
					loop_delta -= 1;
				}
				else if c == b']' && loop_delta == 0 {
					break;
				}
			}
		} else {
			let pos = self.code.stream_position()
			.expect("Failed to find code position");
			self.loop_list.push(pos);
		}
	}
	
	fn end_loop(&mut self){
        if self.machine.get()!=Wrapping(0) {
            self.code.seek(SeekFrom::Start(
                self.loop_list.pop().expect("Failed to find last [") - 1
            )).expect("Failed to go to [");
        } else {
            self.loop_list.pop().expect("Failed to find last [");
        }
	}
}