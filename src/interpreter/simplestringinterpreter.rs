use super::Interpreter;
use super::super::machine::{Machine, SimpleMachine};
use std::io::{Stdin, Stdout, Write, Read};
use std::num::Wrapping;

pub struct SimpleStringInterpreter{
    code: String,
    machine: SimpleMachine,
    input: Stdin,
    output: Stdout,
    loop_list: Vec<usize>
}

impl Interpreter for SimpleStringInterpreter {
    fn interpret(&mut self) {
        let mut char_iter = StringIterator::from_string(&self.code);
        loop{
            let asdf = char_iter.next();
            match asdf{
                None => break,
                Some(b'.') => {
                    self.output.write_all(&[self.machine.get().0]).expect("Failed to write to stdout");
                    self.output.flush().expect("Couldn't flush");
                },
                Some(b',') => {
                    let buf: &mut [u8; 1] = &mut [0]; 
                    self.input.read_exact(buf).expect("Failed to read from stdin");
                    self.machine.set(Wrapping(buf[0]));
                },
                Some(b'[') => {
                    if self.machine.get() == Wrapping(0){
                        let mut loop_delta = 0;
                        loop{
                            let c = char_iter.next().expect("Reached end while looking for ]");

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
                        self.loop_list.push(char_iter.index());
                    }
                },
                Some (b']') => {
                    /*char_iter = char_iter.rev();
                    for _ in 0..i - (self.loop_list.pop().expect("Couldn't get last [")-1) {
                        char_iter.next_back();
                    }*/
                    //char_iter.nth(self.loop_list.pop().expect("Couldn't get last [")-1);

                    char_iter.goto(self.loop_list.pop().expect("Couldn't get last [")-1);
                },
                Some (b'+') => self.machine.inc(),
                Some (b'-') => self.machine.dec(),
                Some (b'>') => self.machine.rgt(),
                Some (b'<') => self.machine.lft(),
                Some (_) => ()
            }
        }
    }
}

impl SimpleStringInterpreter {
    pub fn new(code: String, input: Stdin, output: Stdout) -> Self{
        Self {
            code: code,
            machine: SimpleMachine::new(),
            input: input,
            output: output,
            loop_list: vec![]
        }
    }
}

struct StringIterator {
    i: usize,
    s: Vec<u8>
}

impl Iterator for StringIterator{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item>{
        if self.i>=self.s.len() {
            return None;
        }
        self.i+=1;
        return Some(self.s[self.i]);
    }
}

impl StringIterator{
    fn goto(&mut self, i: usize) {
        if self.i>=self.s.len() {
            panic!("tred");
        }
        self.i = i;
    }

    fn index(&self) -> usize{
        self.i
    }

    fn from_string(string: &String) -> Self{
        Self {
            i: 0,
            s: string.clone().into_bytes()
        }
    } 
}