use std;
use std::hash::{Hash, Hasher};

#[derive(Eq,PartialEq,Copy,Clone,Debug)]
pub struct ID{
    pub slot_index:usize,
}

impl ID{
    pub fn new(slot_index:usize) -> Self {
        ID{
            slot_index:slot_index
        }
    }

    pub fn zeroed() -> Self {
        ID{
            slot_index:<usize>::max_value()
        }
    }
}

impl std::fmt::Display for ID{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"(slot index:{})",self.slot_index)
    }
}

impl Hash for ID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.slot_index.hash(state);
    }
}
