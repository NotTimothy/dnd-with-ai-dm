// game/ui.rs
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};
use super::engine::GameEngine;

pub fn run_ui() {
    let main_window = WindowDesc::new(|| ui_builder())
        .title(LocalizedString::new("AI Dungeon Master"));
    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Failed to launch UI");
}

fn ui_builder() -> impl Widget<()> {
    // Build the UI using Druid widgets
    Flex::column()
        .with_child(Label::new("Welcome to AI Dungeon Master"))
        .with_child(Button::new("Start Game"))

        // .on_click(|_ctx, _data, _env| {
        //     let mut game_engine = GameEngine::new();
        //     game_engine.run();
        // }
}