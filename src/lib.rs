pub mod interpreter{
    mod interpreter;
    pub use interpreter::Interpreter;
}
pub mod machine{
    mod machine;
    pub use machine::Machine;
    mod debuggable;
    pub use debuggable::Debuggable;
}

mod simple;
pub use simple::Simple;