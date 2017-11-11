use std;
use std::hash::{Hash, Hasher};

#[derive(Eq,PartialEq,Copy,Clone,Debug)]
pub struct ID{
    pub slot_index:usize,
    pub unique_id:usize,
}

impl ID{
    pub fn new(slot_index:usize, unique_id:usize) -> Self {
        ID{
            slot_index:slot_index,
            unique_id:unique_id,
        }
    }

    pub fn zeroed() -> Self {
        ID{
            slot_index:0,
            unique_id:0,
        }
    }
}

impl std::fmt::Display for ID{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"(slot index:{}, unique id:{})",self.slot_index,self.unique_id)
    }
}

impl Hash for ID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.slot_index.hash(state);
        self.unique_id.hash(state);
    }
}
