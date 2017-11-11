
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
    ($t:path,$field:ident,$id:ident) => {
        impl Slot for $t {
            fn set_id(&mut self,id:$id) {
                self.$field=id;
            }

            fn get_id(&self) -> $id {
                self.$field
            }
        }
    };
}
