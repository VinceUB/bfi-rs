use super::Machine;
use std::num::Wrapping;

pub struct SimpleMachine{
    mem: Vec<<Self as Machine>::Cell>,
    ptr: usize
}

impl Machine for SimpleMachine{
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
        self.ptr = self.ptr.checked_sub(1).expect("Trying to move pointer below zero");
    }
    fn rgt(&mut self){
        self.ptr += 1;
        if self.ptr >= self.mem.len() {
            self.mem.push(Wrapping(0));
        }
    }
}

impl Default for SimpleMachine{
    fn default() -> Self {
        return Self{
            mem: vec![Wrapping(0)],
            ptr: 0
        }
    }
}

impl SimpleMachine{
    pub fn new() -> Self {
        Self::default()
    }
}