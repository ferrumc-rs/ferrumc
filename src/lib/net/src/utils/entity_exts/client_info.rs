use ferrumc_ecs::ECSResult;
use ferrumc_ecs::entities::Entity;
use ferrumc_state::GlobalState;
use crate::packets::incoming::client_information::ClientInformation;

pub trait ClientInfoEntityExt {
    fn get_render_distance(&self, state: &GlobalState) -> ECSResult<u8>;
}

impl ClientInfoEntityExt for Entity {
    fn get_render_distance(&self, state: &GlobalState) -> ECSResult<u8> {
        let info = state.universe.get::<ClientInformation>(*self)?;
        Ok(info.view_distance)
    }
}