pub trait Machine{
    type Cell;
    fn inc(&mut self);                  //Increment at data pointer (+)
    fn dec(&mut self);                  //Decretment at data pointer (-)
    fn get(&self) -> Self::Cell;        //Returns byte at data pointer
    fn set(&mut self, val: Self::Cell); //Sets byte at data pointer
    fn lft(&mut self);                  //Move pointer left (<)
    fn rgt(&mut self);                  //Move pointer right (>)
}