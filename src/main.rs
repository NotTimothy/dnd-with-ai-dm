// main.rs

mod game;
mod server;
mod ai;

fn main() {
    // Initialize the game engine
    let mut game_engine = game::engine::GameEngine::new();

    // Initialize the UI
    game::ui::run_ui();

    // Start the server
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        server::handler::run_server().await;
    });

    // Run the game loop
    game_engine.run();
}

