use crate::{
    interpreter::Interpreter,
    machine::Machine
};
use std::{
    fs::File,
    io::{Stdin, Stdout, Seek, BufReader, SeekFrom, Read, Write, ErrorKind},
    num::Wrapping
};

pub struct Simple{
    mem: Vec<<Self as Machine>::Cell>,
    ptr: usize,
    code: BufReader<File>,
    output: Stdout,
    input: Stdin,
    loop_list: Vec<u64>
}

impl Machine for Simple{
    type Cell = Wrapping<u8>;

    fn inc(&mut self){
        self.mem[self.ptr]+=Wrapping(1);
    }
    fn dec(&mut self){
        self.mem[self.ptr]-=Wrapping(1);
    }
    fn get(&self) -> Self::Cell{
        return self.mem[self.ptr];
    }
    fn set(&mut self, val: Self::Cell){
        self.mem[self.ptr] = val;
    }
    fn lft(&mut self){
        self.ptr = match self.ptr.checked_sub(1){
            Some(p) => p,
            None => {
                panic!("Trying to move pointer below zero (too far left) at position {}", match self.code.stream_position(){
                    Ok(i) => i.to_string(),
                    Err(_) => "?".to_string()
                });
            }
        };
    }
    fn rgt(&mut self){
        self.ptr += 1;
        if self.ptr >= self.mem.len() {
            self.mem.push(Wrapping(0));
        }
    }
}

impl Interpreter for Simple {
    type CodeStream = File;
    type OutputStream = Stdout;
    type InputStream = Stdin;

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
                b'+' => self.inc(),
                b'-' => self.dec(),
                b'<' => self.lft(),
                b'>' => self.rgt(),
                b'.' => {
                    self.output.write_all(&[self.get().0])
                    .expect("Failed to write to output");
                    self.output.flush().expect("Failed to flush output");
                },
                b',' => {
                    let mut readbuf: [u8;1] = [0];
                    self.input.read_exact(&mut readbuf).expect("Could not read from input");
                    self.set(Wrapping(readbuf[0]));
                }
                b'[' => self.start_loop(),
                b']' => self.end_loop(),
                _ => ()
            };
        }
    }
}

impl Simple{
    pub fn new(code: File, input: Stdin, output: Stdout) -> Self{
        Self{
            code: BufReader::new(code),
            output: output,
            input: input,
            loop_list: Vec::new(),
            mem: vec![Wrapping(0u8)],
            ptr: 0
        }
    }

	fn start_loop(&mut self){
		if self.get() == Wrapping(0){
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
		self.code.seek(SeekFrom::Start(
			self.loop_list.pop().expect("Failed to find last [") - 1
		)).expect("Failed to go to [");
	}
}