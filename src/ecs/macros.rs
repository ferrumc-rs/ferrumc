#[macro_export]
macro_rules! component_id {
    ($type: ty) => {
        std::any::TypeId::of::<$type>()
    };
}

