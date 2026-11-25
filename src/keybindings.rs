//! Keyboard input handling and action mapping.
//!
//! Maps physical keybindings to semantic actions and provides
//! action candidates without evaluating their validity
//! in the current game state.

use crate::game::Action;
use macroquad::prelude::*;

/// Returns the key bound to the given action.
///
/// If you change a key here, update the hints in `draw`.
pub fn action_to_key(action: Action) -> KeyCode {
    match action {
        Action::ExitToMenu => KeyCode::Escape,
        Action::GameOver => KeyCode::Key2,
        Action::Pause => KeyCode::Escape,
        Action::QuitGame => KeyCode::Escape,
        Action::Resume => KeyCode::Key1,
        Action::StartGame => KeyCode::Key1,
    }
}

/// Returns true if the action's key was pressed this frame.
pub fn is_action_pressed(action: Action) -> bool {
    is_key_pressed(action_to_key(action))
}

/// Returns an iterator over triggered action candidates
pub fn process_input() -> impl Iterator<Item = Action> {
    [
        Action::QuitGame,
        Action::StartGame,
        Action::Pause,
        Action::Resume,
        Action::GameOver,
        Action::ExitToMenu,
    ]
    .into_iter()
    .filter(|action| is_action_pressed(*action))
}
