
use std::borrow::Borrow;
use std::marker::PhantomData;

use super::ID;
use super::Slot;
use super::chunk::Chunk;
//use super::Iter;

const SLOTS_COUNT:usize=64;

pub struct Pool<SC:From<S> + Borrow<S>,S:Slot>{
    chunks:Vec< Box<Chunk<SC,S>> >,
    free:usize,
    last:usize,
    unique_id:usize,
}

impl<SC:From<S> + Borrow<S>,S:Slot> Pool<SC,S> {
    pub fn new() -> Self {
        Pool{
            chunks:Vec::new(),
            free:0,
            last:0,
            unique_id:1,
        }
    }

    pub fn insert(&mut self,mut slot:S) -> &mut SC {
        if self.free==self.chunks.len() {
            self.chunks.push( Box::new(Chunk::new()) );
            self.last=self.free;
        }

        let insert_chunk_index=self.free;
        let insert_slot_index=self.chunks[self.free].get_free_slot_index();
        let id=ID::new(self.free*SLOTS_COUNT + insert_slot_index, self.unique_id);
        self.unique_id+=1; //TODO:select set of unique ids if owerflow and maybe ID<T,T> with T limit

        slot.set_id(id);
        self.chunks[insert_chunk_index].insert(slot);

        while self.free<self.chunks.len() && self.chunks[self.free].is_full() {
            self.free+=1;
        }

        self.chunks[insert_chunk_index].get_by_index_mut(insert_slot_index)
    }

    pub fn insert_limited(&mut self,mut slot:S) -> Option<&mut SC> {
        if self.free==self.chunks.len() {
            self.chunks.push( Box::new(Chunk::new()) );
            self.last=self.free;
        }

        let insert_chunk_index=self.free;
        let insert_slot_index=self.chunks[self.free].get_free_slot_index();
        let id=ID::new(self.free*SLOTS_COUNT + insert_slot_index, self.unique_id);
        self.unique_id+=1; //TODO:select set of unique ids if owerflow and maybe ID<T,T> with T limit

        slot.set_id(id);
        self.chunks[insert_chunk_index].insert(slot);

        while self.free<self.chunks.len() && self.chunks[self.free].is_full() {
            self.free+=1;
        }

        Some(self.chunks[insert_chunk_index].get_by_index_mut(insert_slot_index))
    }

    pub fn get<T>(&self, id:T) -> Option<&SC> where T:Into<ID> {
        let id:ID=id.into();
        let chunk_index=id.slot_index/SLOTS_COUNT;

        if chunk_index >= self.chunks.len() {
            return None;
        }

        self.chunks[chunk_index].get(id)
    }

    pub fn get_mut(&mut self, id:ID) -> Option<&mut SC> {
        let chunk_index=id.slot_index/SLOTS_COUNT;

        if chunk_index >= self.chunks.len() {
            return None;
        }

        self.chunks[chunk_index].get_mut(id)
    }

    pub fn remove(&mut self, id:ID) -> bool{
        let chunk_index=id.slot_index/SLOTS_COUNT;

        if chunk_index >= self.chunks.len() {
            return false;
        }

        let removed=self.chunks[chunk_index].remove(id);
        if removed {
            if chunk_index<self.free {
                self.free=chunk_index;
            }

            if self.chunks[chunk_index].is_empty() && chunk_index==self.last {
                let mut clear_all=false;
                loop {
                    if self.last>0 {
                        self.last-=1;
                    }else{
                        clear_all=true;
                        break;
                    }

                    if !self.chunks[self.last].is_empty() {
                        break;
                    }
                }

                if clear_all {
                    self.chunks.clear();
                }else{
                    if self.last*2<=self.chunks.len(){
                        self.chunks.truncate(self.last+1);
                    }
                }
            }
        }

        removed
    }

    pub fn iter(&self) -> Iter<SC,S> {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut<SC,S> {
        IterMut::new(self)
    }

    pub fn chunks_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn len(&self) -> usize{
        let mut len=0;

        for chunk in self.chunks.iter(){
            len+=chunk.len();
        }

        len
    }

    pub fn future_id(&self) -> ID {
        if self.free==self.chunks.len() {
            ID::new(self.free*SLOTS_COUNT, self.unique_id)
        }else{
            if self.chunks[self.free].is_full() {
                ID::new((self.free+1)*SLOTS_COUNT, self.unique_id)
            }else{
                let insert_slot_index=self.chunks[self.free].get_free_slot_index();
                ID::new(self.free*SLOTS_COUNT + insert_slot_index, self.unique_id)
            }
        }
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
        self.free=0;
        self.last=0;
    }
}

// Iter

pub struct Iter<'a,SC:From<S> + Borrow<S> + 'a,S:Slot + 'a>{
    pool:&'a Pool<SC,S>,
    chunk_index:usize,
    slot_index:usize,
}

impl<'a,SC:From<S> + Borrow<S> + 'a,S:Slot + 'a> Iter<'a,SC,S> {
    pub fn new(pool:&'a Pool<SC,S>) -> Self {
        Iter{
            pool:pool,
            chunk_index:0,
            slot_index:0,
        }
    }
}

impl<'a,SC:From<S> + Borrow<S> + 'a,S:Slot + 'a> Iterator for Iter<'a,SC,S>{
    type Item=&'a SC;

    fn next(&mut self) -> Option<Self::Item> {
        loop{
            if self.chunk_index >= self.pool.chunks.len() {
                return None;
            }

            loop {
                let slot=&self.pool.chunks[self.chunk_index].slots[self.slot_index];

                self.slot_index+=1;
                let end_reached=self.slot_index==SLOTS_COUNT;

                if end_reached {
                    self.slot_index=0;
                    self.chunk_index+=1;
                }

                match *slot{
                    Some( ref slot_container ) => return Some(slot_container),
                    None => {},
                }

                if end_reached {
                    break;
                }
            }
        }
    }
}

// IterMut

pub struct IterMut<'a,SC:From<S> + Borrow<S> + 'a,S:Slot + 'a>{
    pool:*mut Pool<SC,S>,
    chunk_index:usize,
    slot_index:usize,
    _phantom_data:PhantomData<&'a mut ()>,
}

impl<'a,SC:From<S> + Borrow<S> + 'a,S:Slot + 'a> IterMut<'a,SC,S> {
    pub fn new(pool:&mut Pool<SC,S>) -> Self {
        IterMut{
            pool:pool as *mut Pool<SC,S>,
            chunk_index:0,
            slot_index:0,
            _phantom_data:PhantomData,
        }
    }
}

impl<'a,SC:From<S> + Borrow<S> + 'a,S:Slot + 'a> Iterator for IterMut<'a,SC,S>{
    type Item=&'a mut SC;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe{
            loop{
                if self.chunk_index >= (*self.pool).chunks.len() {
                    return None;
                }

                loop {
                    let slot=&mut (*self.pool).chunks[self.chunk_index].slots[self.slot_index];

                    self.slot_index+=1;
                    let end_reached=self.slot_index==SLOTS_COUNT;

                    if end_reached {
                        self.slot_index=0;
                        self.chunk_index+=1;
                    }

                    match *slot{
                        Some( ref mut slot_container ) => return Some(slot_container),
                        None => {},
                    }

                    if end_reached {
                        break;
                    }
                }
            }
        }
    }
}
