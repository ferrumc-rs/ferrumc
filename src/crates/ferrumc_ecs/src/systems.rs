use crate::components::{Position, Velocity};
use crate::world::World;

// pub fn movement_system(world: &mut World) {
//     let (positions, velocities) = world.get_component_storages::<Position, Velocity>();

//     let (positions, velocities) = match (positions, velocities) {
//         (Some(p), Some(v)) => (p, v),
//         _ => {
//             // If the component storage is missing, return early
//             println!("Missing component storage");
//             return;
//         }
//     };

//     for (pos, vel) in positions.data.iter_mut().zip(velocities.data.iter()) {
//         let (Some(position), Some(velocity)) = (pos, vel) else {
//             continue;
//         };

//         position.add_velocity(velocity);
//         println!("New position: {:?}", position);
//     }
// }
