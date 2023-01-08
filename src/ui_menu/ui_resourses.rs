use bevy::prelude::*;

#[derive(Resource)]
pub struct FontHolder {
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct MenuTitles {
    pub title: String,
    pub button_new_game: String,
    pub button_continue_game: String,
    pub button_exit_game: String,
}

impl MenuTitles {
    pub fn new() -> MenuTitles {
        MenuTitles {
            title: "Pixel Tanks".to_string(),
            button_new_game: "New Game".to_string(),
            button_continue_game: "Continue".to_string(),
            button_exit_game: "Exit".to_string(),
        }
    }
}
