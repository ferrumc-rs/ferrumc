use ferrumc_macros::{packet, NetEncode};

#[derive(NetEncode)]
#[packet(packet_id = "game_event", state = "play")]
pub struct GameEventPacket {
    pub event_id: u8,
    pub value: f32,
}

impl GameEventPacket {
    /// GameEvent packet ID to indicate no respawn block is available.  
    /// This displays this message to the user:
    /// > "You have no home bed or charged respawn anchor, or it was obstructed"
    pub const NO_RESPAWN_BLOCK_AVAILABLE: u8 = 0;

    /// GameEvent packet ID to begin raining.
    /// The rest of the rain generation is handled client-side.  
    /// To stop the rain, use [`END_RAINING`].
    pub const BEGIN_RAINING: u8 = 1;

    /// GameEvent packet ID to end raining.
    /// The rest of the rain generation is handled client-side.  
    /// To start the rain, use [`BEGIN_RAINING`].
    pub const END_RAINING: u8 = 2;

    /// GameEvent packet ID to change the gamemode of a player.  
    /// Value options:
    /// - 0: Survival
    /// - 1: Creative
    /// - 2: Adventure
    /// - 3: Spectator
    pub const CHANGE_GAME_MODE: u8 = 3;

    /// GameEvent packet ID to indicate that the player has won the game.  
    /// Value options:
    /// - 0: Respawn player without credits screen
    /// - 1: Show credits screen and then respawn player  
    ///
    /// A value of `1` should be sent if the player has NOT yet recieved the
    /// 'The end?' advancement (or any other check to see if they have killed
    /// the dragon).  
    /// > Raisers of this packet most likely will not be implemented for a long
    /// > time.
    pub const WIN_GAME: u8 = 4;

    /// GameEvent packet ID to trigger demo messages.
    /// *Copied from https://minecraft.wiki/w/Java_Edition_protocol/Packets*  
    /// Possible values:
    /// - 0: Show welcome to demo screen.
    /// - 101: Tell movement controls.
    /// - 102: Tell jump control.
    /// - 103: Tell inventory control.
    /// - 104: Tell that the demo is over and print a message about how to take a screenshot.
    pub const DEMO_EVENT: u8 = 5;

    /// GameEvent packet ID to indicate that an arrow has hit a player.
    pub const ARROW_HIT_PLAYER: u8 = 6;

    /// GameEvent packet ID to change the rain intensity level.
    /// The protocol wiki page states that this seems to change both the lighting
    /// and sky color.
    /// Value:
    /// - `f32` from `0-1` indicating the rain intensity.
    pub const RAIN_LEVEL_CHANGE: u8 = 7;

    /// GameEvent packet ID to change the thunder intensity level.
    /// The protocol wiki page states that this seems to change both the lighting
    /// and sky color.
    /// Value:
    /// - `f32` from `0-1` indicating the thunder intensity.
    pub const THUNDER_LEVEL_CHANGE: u8 = 8;

    /// GameEvent packet ID to play the pufferfish sting sound effect.
    pub const PLAY_PUFFERFISH_STING_SOUND: u8 = 9;

    /// GameEvent packet ID to play the elder guardian mob appearance sound effect.
    pub const PLAY_ELDER_GUARDIAN_MOB_APPEARANCE: u8 = 10;

    /// GameEvent packet ID to enable/disable the respawn screen when the player
    /// dies.
    /// This is used when changing the `doImmediateRespawn` gamerule.
    /// Possible values:
    /// - 0: Enable respawn screen.
    /// - 1: Disable respawn screen (immediate respawn).
    pub const DISABLE_RESPAWN_SCREEN: u8 = 11;

    /// GameEvent packet ID to indicate that the client should enable/disable
    /// limited crafting mode. If enabled (1), the recipe book will only show
    /// recipes that have already been crated by the player.
    /// Possible values:
    /// - 0: Disable limited crafting mode.
    /// - 1: Enable limited crafting mode.
    pub const LIMITED_CRAFTING: u8 = 12;

    /// GameEvent packet ID to indicate that the client should start waiting
    /// for level chunks to be sent from the server. This is used during the
    /// initial connection to the server, but also if the world has been
    /// reloaded (e.g. when changing dimensions) or possibly also when exploring
    /// the terrain.
    pub const START_WAITING_FOR_LEVEL_CHUNKS: u8 = 13;

    pub fn new(event_id: u8, value: f32) -> Self {
        Self { event_id, value }
    }

    pub fn start_waiting_for_level_chunks() -> Self {
        Self::new(13, 0f32)
    }
}
