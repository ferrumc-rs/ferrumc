use bevy_ecs::resource::Resource;

#[derive(Resource, Debug, Default)]
pub struct WorldTime {
    pub world_age: i64,
    pub time_of_day: i64,
    pub tick_counter: u64,
}
