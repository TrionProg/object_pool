
#[macro_export]
macro_rules! impl_slot {
    ($t:path) => {
        impl Slot for $t {
            fn set_id(&mut self,id:ID) {
                self.id=id;
            }

            fn get_id(&self) -> ID {
                self.id
            }
        }
    };
}
