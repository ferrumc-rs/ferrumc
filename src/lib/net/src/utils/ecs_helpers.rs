use ferrumc_ecs::components::storage::{Component, ComponentRef, ComponentRefMut};
use ferrumc_ecs::ECSResult;
use ferrumc_state::GlobalState;

#[allow(async_fn_in_trait)]
pub trait EntityExt {
    async fn get<T: Component>(&self, state: &GlobalState) -> ECSResult<ComponentRef<T>>;
    async fn get_mut<T: Component>(&self, state: &GlobalState) -> ECSResult<ComponentRefMut<T>>;
}

impl EntityExt for usize {
    async fn get<T: Component>(&self, state: &GlobalState) -> ECSResult<ComponentRef<T>> {
        state.universe.get::<T>(*self).await
    }

    async fn get_mut<T: Component>(&self, state: &GlobalState) -> ECSResult<ComponentRefMut<T>> {
        state.universe.get_mut::<T>(*self).await
    }
}
