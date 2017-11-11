use super::ID;

pub trait Slot{
    fn set_id(&mut self,id:ID);
    fn get_id(&self) -> ID;
}
