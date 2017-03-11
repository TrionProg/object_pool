extern crate object_pool;
pub use object_pool::growable::{ID,Pool};

struct Slot{
    id:ID,
    value:usize,
}

impl object_pool::indexed_growable::Slot for Slot{
    fn set_id(&mut self, id:ID) {
        self.id=id;
    }

    fn get_id(&self) -> ID {
        self.id
    }
}

impl Slot{
    fn new(value:usize) -> Slot{
        Slot{
            id:ID::zeroed(),
            value:value,
        }
    }
}

#[test]
fn create_remove_1chunk(){
    let mut pool:Pool<Slot,Slot>=Pool::new();

    for i in 0..50{
        pool.insert(Slot::new(i));
    }

    for i in 0..50{
        assert_eq!( pool.get(ID::new(i, i+1)).unwrap().value, i );
        assert_eq!( pool.get(ID::new(i, i+1)).unwrap().id.slot_index, i );
        assert_eq!( pool.get(ID::new(i, i+1)).unwrap().id.unique_id, i+1 );
    }

    pool.remove(ID::new(20, 21));
    pool.remove(ID::new(21, 22));
    pool.remove(ID::new(30, 31));
    pool.remove(ID::new(8, 9));

    pool.insert(Slot::new(8));
    assert_eq!( pool.get(ID::new(8, 51)).unwrap().value, 8 );
    assert_eq!( pool.get(ID::new(8, 51)).unwrap().id, ID::new(8, 51) );

    pool.insert(Slot::new(20));
    assert_eq!( pool.get(ID::new(20, 52)).unwrap().value, 20 );
    assert_eq!( pool.get(ID::new(20, 52)).unwrap().id, ID::new(20, 52) );

    pool.insert(Slot::new(21));
    assert_eq!( pool.get(ID::new(21, 53)).unwrap().value, 21 );
    assert_eq!( pool.get(ID::new(21, 53)).unwrap().id, ID::new(21, 53) );

    assert_eq!( pool.remove(ID::new(20,52)), true);

    assert_eq!( pool.len(), 48 );
}

#[test]
fn create_remove_3chunk(){
    let mut pool:Pool<Slot,Slot>=Pool::new();

    for i in 0..150{
        pool.insert(Slot::new(i));
    }

    for i in 0..150{
        assert_eq!( pool.get(ID::new(i, i+1)).unwrap().value, i );
        assert_eq!( pool.get(ID::new(i, i+1)).unwrap().id.slot_index, i );
        assert_eq!( pool.get(ID::new(i, i+1)).unwrap().id.unique_id, i+1 );
    }

    pool.remove(ID::new(140, 141));
    pool.remove(ID::new(81, 82));
    pool.remove(ID::new(20, 21));

    pool.insert(Slot::new(20));
    assert_eq!( pool.get(ID::new(20, 151)).unwrap().value, 20 );
    assert_eq!( pool.get(ID::new(20, 151)).unwrap().id, ID::new(20, 151) );

    pool.insert(Slot::new(81));
    assert_eq!( pool.get(ID::new(81, 152)).unwrap().value, 81 );
    assert_eq!( pool.get(ID::new(81, 152)).unwrap().id, ID::new(81, 152) );

    assert_eq!( pool.len(), 149 );
}

#[test]
fn remove_4chunk(){
    let mut pool:Pool<Slot,Slot>=Pool::new();

    for i in 0..200{
        pool.insert(Slot::new(i));
    }

    for i in 0..200{
        assert_eq!( pool.get(ID::new(i, i+1)).unwrap().value, i );
        assert_eq!( pool.get(ID::new(i, i+1)).unwrap().id.slot_index, i );
        assert_eq!( pool.get(ID::new(i, i+1)).unwrap().id.unique_id, i+1 );
    }

    for i in 50..64*3 {
        pool.remove(ID::new(i, i+1));
    }

    assert_eq!( pool.chunks_count(),4 );

    pool.insert(Slot::new(50));
    assert_eq!( pool.get(ID::new(50, 201)).unwrap().value, 50 );
    assert_eq!( pool.get(ID::new(50, 201)).unwrap().id, ID::new(50, 201) );

    for i in 64*3..200 {
        pool.remove(ID::new(i, i+1));
    }

    assert_eq!( pool.chunks_count(),1 );

    pool.insert(Slot::new(51));

    assert_eq!( pool.len(), 52 );
}

#[test]
fn remove_all_4chunk(){
    let mut pool:Pool<Slot,Slot>=Pool::new();

    for i in 0..200{
        pool.insert(Slot::new(i));
    }

    for i in 72..64*3 {
        pool.remove(ID::new(i, i+1));
    }

    let mut slots_id=Vec::with_capacity(pool.len());

    for slot in pool.iter(){
        slots_id.push(slot.id);
    }

    println!("removing");

    for slot_id in slots_id.iter().rev() {
        println!("rem {}",slot_id.unique_id);
        pool.remove(*slot_id);
    }

    assert_eq!(pool.len(),0);
    assert_eq!(pool.chunks_count(),0);

    for i in 0..200{
        pool.insert(Slot::new(i));
    }

    assert_eq!(pool.len(),200);

    pool.clear();

    assert_eq!(pool.len(),0);
}
