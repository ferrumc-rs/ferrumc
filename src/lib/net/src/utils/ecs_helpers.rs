use ferrumc_ecs::components::storage::{Component, ComponentRef, ComponentRefMut};
use ferrumc_ecs::ECSResult;
use ferrumc_core::state::GlobalState;

pub trait EntityExt {
    fn get<T: Component>(&self, state: GlobalState) -> ECSResult<ComponentRef<T>>;
    fn get_mut<T: Component>(&self, state: GlobalState) -> ECSResult<ComponentRefMut<T>>;
}

impl EntityExt for usize {
    fn get<T: Component>(&self, state: GlobalState) -> ECSResult<ComponentRef<T>> {
        state.universe.get::<T>(*self)
    }

    fn get_mut<T: Component>(&self, state: GlobalState) -> ECSResult<ComponentRefMut<T>> {
        state.universe.get_mut::<T>(*self)
    }
}