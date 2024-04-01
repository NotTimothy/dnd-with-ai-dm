// game/engine.rs
use bevy::prelude::*;

pub struct GameEngine;

impl GameEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) {
        // Initialize and run the game using Bevy
        App::new()
            .add_plugins(DefaultPlugins)
            // Add game-specific systems and resources
            .run();
    }
}