#[macro_export]
macro_rules! component_id {
    ($type: ty) => {
        std::any::TypeId::of::<$type>()
    };
}


/*#[macro_export]
macro_rules! register_components {
    ($($component:ty),* $(,)?) => {
        pub struct TypedComponentStorage {
            storage: ComponentStorage
        }

        impl TypedComponentStorage {
            pub fn new() -> Self {
                TypedComponentStorage {
                    storage: ComponentStorage::new()
                }
            }


            pub fn get<T>(&self, entity: Entity) -> Option<&T>
            where
                T: Component
            {
                self.storage.get::<T>(entity)
            }
        }
    };
}*/