
use ::growable;

use std::sync::RwLock;
use std::sync::Arc;
use std::sync::{RwLockWriteGuard, RwLockReadGuard};

pub use growable::Slot;
pub use growable::ID;

pub struct Pool<T:Slot>{
    inner:RwLock< growable::Pool<Arc<T>,T> >,
}

impl<T:Slot> Pool<T> {
    pub fn new() -> Self {
        Pool{
            inner:RwLock::new(growable::Pool::new())
        }
    }

    pub fn insert(&self, slot:T) -> Arc<T> {
        let mut inner_guard=self.inner.write().unwrap();

        inner_guard.insert(slot).clone()
    }

    pub fn get(&self, id:ID) -> Option< Arc<T> > {
        let inner_guard=self.inner.read().unwrap();

        match inner_guard.get(id) {
            Some( slot_container ) => Some( slot_container.clone() ),
            None => None,
        }
    }

    pub fn remove(&self, id:ID) -> bool {
        let mut inner_guard=self.inner.write().unwrap();

        inner_guard.remove(id)
    }

    pub fn clear(&self) {
        self.inner.write().unwrap().clear();
    }

    pub fn read(&self) -> RwLockReadGuard< growable::pool::Pool<Arc<T>,T> >{
        self.inner.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard< growable::pool::Pool<Arc<T>,T> >{
        self.inner.write().unwrap()
    }
}
