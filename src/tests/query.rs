use tokio::net::TcpListener;
use crate::create_state;
use crate::utils::components::player::Player;
use crate::utils::encoding::position::Position;

#[test]
async fn demonstrate_simple_query_usage() {
    // Create the game state, including setting up a TCP listener for the server.
    let state = create_state(TcpListener::bind("0.0.0.0:25565").await.unwrap()).await.unwrap();

    // Define a query to get all players and their positions from the ECS world.
    let mut query = state.world.query::<(&Player, &Position)>();

    // Loop through each player and their position, printing out the player's username and position.
    while let Some((_, (player, pos))) = query.next().await {
        println!("Player {} is at {:?}", player.get_username(), *pos);
    }
}