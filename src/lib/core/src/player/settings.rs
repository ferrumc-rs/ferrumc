use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

impl Default for ChatMode {
    fn default() -> Self {
        Self::Enabled
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MainHand {
    Left,
    Right,
}

impl Default for MainHand {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParticleStatus {
    All,
    Decreased,
    Minimal,
}

impl Default for ParticleStatus {
    fn default() -> Self {
        Self::All
    }
}

impl Display for ChatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatMode::Enabled => write!(f, "Enabled"),
            ChatMode::CommandsOnly => write!(f, "CommandsOnly"),
            ChatMode::Hidden => write!(f, "Hidden"),
        }
    }
}

impl Display for ParticleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParticleStatus::All => write!(f, "All"),
            ParticleStatus::Decreased => write!(f, "Decreased"),
            ParticleStatus::Minimal => write!(f, "Minimal"),
        }
    }
}

impl Display for MainHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainHand::Left => write!(f, "Left"),
            MainHand::Right => write!(f, "Right"),
        }
    }
}
