
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
