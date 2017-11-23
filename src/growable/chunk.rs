use std;

use std::mem;
use std::ptr;
use std::borrow::Borrow;

use super::ID;

const SLOTS_COUNT:usize=64;

macro_rules! make_array {
    ($n:expr, $constructor:expr) => {{
        let mut items: [_; $n] = mem::uninitialized();
        for place in items.iter_mut() {
            ptr::write(place, $constructor);
        }
        items
    }}
}

pub struct Chunk<SC:From<S> + Borrow<S>,S>{
    pub slots:[Option<SC>;SLOTS_COUNT],
    free:usize,
    len:usize,
    _phantom_data:std::marker::PhantomData<S>,
}

impl<SC:From<S> + Borrow<S>,S> Chunk<SC,S> {
    pub fn new() -> Self {
        Chunk{
            slots:unsafe { make_array!(SLOTS_COUNT, None) },
            free:0,
            len:0,
            _phantom_data:std::marker::PhantomData,
        }
    }

    pub fn is_full(&self) -> bool {
        self.free==SLOTS_COUNT
    }

    pub fn is_empty(&self) -> bool {
        self.len==0
    }

    pub fn get_free_slot_index(&self) -> usize {
        self.free
    }

    pub fn insert(&mut self, slot:S) {
        self.slots[self.free]=Some(SC::from(slot));
        self.len+=1;

        self.free+=1;
        while self.free<SLOTS_COUNT && self.slots[self.free].is_some() {
            self.free+=1;
        }
    }

    pub fn get(&self, id:ID) -> Option<&SC> {
        let slot_index=id.slot_index%SLOTS_COUNT;

        match self.slots[slot_index] {
            Some( ref slot_container ) => {
                Some( slot_container )
            },
            None => None,
        }
    }

    pub fn get_mut(&mut self, id:ID) -> Option<&mut SC> {
        let slot_index=id.slot_index%SLOTS_COUNT;

        match self.slots[slot_index] {
            Some( ref mut slot_container ) => {
                Some( slot_container )
            },
            None => None,
        }
    }

    pub fn get_by_index_mut(&mut self, index:usize) -> &mut SC {
        match self.slots[index] {
            Some( ref mut sc ) => sc,
            None => unreachable!(),
        }
    }

    pub fn remove(&mut self, id:ID) -> bool {
        let slot_index=id.slot_index%SLOTS_COUNT;

        if self.slots[slot_index].is_none() {
            return false;
        }

        self.slots[slot_index]=None;
        self.len-=1;

        if slot_index<self.free {
            self.free=slot_index;
        }

        true
    }

    pub fn len(&self) -> usize{
        self.len
    }
}
