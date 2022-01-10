use super::Machine;

pub trait Debuggable: Machine{
    type InstructionPointer;
    type DataPointer;
    fn instruction_pointer(&self) -> &Self::InstructionPointer;
    fn instruction_pointer_mut(&mut self) -> &mut Self::InstructionPointer;

    fn data_pointer(&self) -> &Self::DataPointer;
    fn data_pointer_mut(&mut self) -> &mut Self::DataPointer;

    fn data(&self) -> &Vec<Self::Cell>;
    fn data_mut(&self) -> &mut Vec<Self::Cell>;

    fn cell_at(&self, index: &Self::DataPointer) -> &Self::Cell;
    fn cell_at_mut(&mut self, index: &Self::DataPointer) -> &mut Self::Cell;
}