use std;
use super::{SlotID};

pub struct Pool<T:Sized>{
    slots:Vec<Option<T>>,
    capacity:usize,
    free:usize,
}

impl<T:Sized> Pool<T>{
    pub fn with_capacity(capacity:SlotID) -> Self{
        let mut slots=Vec::with_capacity(capacity as usize);

        for _ in 0..capacity {
            slots.push(None);
        }

        Pool{
            slots:slots,
            capacity:capacity as usize,
            free:0,
        }
    }

    pub fn insert(&mut self, t:T) -> Option<(SlotID,&mut T)> {
        if self.free==self.capacity {
            return None;
        }

        let slot_id=self.free;
        self.slots[slot_id]=Some(t);

        self.free+=1;
        while self.free<self.capacity && self.slots[self.free].is_some() {
            self.free+=1;
        }

        match self.slots[slot_id] {
            Some( ref mut slot ) => Some( (slot_id as SlotID, slot) ),
            None => None,
        }
    }

    pub fn alloc(&mut self) -> Option<(SlotID,&mut T)> {
        if self.free==self.capacity {
            return None;
        }

        let slot_id=self.free;
        self.slots[slot_id]=Some(unsafe{std::mem::uninitialized()});

        self.free+=1;
        while self.free<self.capacity && self.slots[self.free].is_some() {
            self.free+=1;
        }

        match self.slots[slot_id] {
            Some( ref mut slot ) => Some( (slot_id as SlotID, slot) ),
            None => None,
        }
    }

    pub fn erase(&mut self, slot_id:SlotID) {
        self.slots[slot_id as usize]=None;

        if (slot_id as usize) < self.free {
            self.free=slot_id as usize;
        }
    }

    pub fn get(&self, slot_id:SlotID) -> Option<&T> {
        if (slot_id as usize) >= self.capacity {
            return None;
        }

        match self.slots[slot_id as usize] {
            Some( ref slot ) => Some( slot ),
            None => None,
        }
    }

    pub fn get_mut(&mut self, slot_id:SlotID) -> Option<&mut T> {
        if (slot_id as usize) >= self.capacity {
            return None;
        }

        match self.slots[slot_id as usize] {
            Some( ref mut slot ) => Some( slot ),
            None => None,
        }
    }
}
