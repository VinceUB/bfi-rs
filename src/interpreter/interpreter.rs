use std::io::Read;
use std::io::Write;
use std::io::Seek;

pub trait Interpreter{
    type CodeStream: Seek+Read;
    type InputStream: Read;
    type OutputStream: Write;

    fn interpret(&mut self) -> ();
}