pub mod interpreter{
    mod interpreter;
    pub use interpreter::Interpreter;
    mod simplefileinterpreter;
    pub use simplefileinterpreter::SimpleFileInterpreter;
    mod simplestringinterpreter;
    pub use simplestringinterpreter::SimpleStringInterpreter;
}
pub mod machine{
    mod machine;
    pub use machine::Machine;
    mod debuggable;
    pub use debuggable::Debuggable;
    mod simplemachine;
    pub use simplemachine::SimpleMachine;
}