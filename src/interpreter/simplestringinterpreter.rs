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
        let mut i = 0;
        loop{
            match self.code.chars().nth(i){
                None => break,
                Some('.') => {
                    self.output.write_all(&[self.machine.get().0]).expect("Failed to write to stdout");
                    self.output.flush().expect("Couldn't flush");
                },
                Some(',') => {
                    let buf: &mut [u8; 1] = &mut [0]; 
                    self.input.read_exact(buf).expect("Failed to read from stdin");
                    self.machine.set(Wrapping(buf[0]));
                },
                Some('[') => {
                    if self.machine.get() == Wrapping(0){
                        let mut loop_delta = 0;
                        loop{
                            i += 1;
                            let c = self.code.chars().nth(i).expect("Reached end while looking for ]");

                            if c == '[' {
                                loop_delta += 1;
                            } 
                            else if c == ']' && loop_delta >0 {
                                loop_delta -= 1;
                            }
                            else if c == ']' && loop_delta == 0 {
                                break;
                            }
                        }
                    } else {
                        self.loop_list.push(i);
                    }
                },
                Some (']') => {
                    i = self.loop_list.pop().expect("Couldn't get last [")-1;
                },
                Some ('+') => self.machine.inc(),
                Some ('-') => self.machine.dec(),
                Some ('>') => self.machine.rgt(),
                Some ('<') => self.machine.lft(),
                Some (_) => ()
            }
            i+=1;
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