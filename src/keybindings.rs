use macroquad::prelude::*;

/// Actions that can be triggered by player input.
#[derive(Debug)]
pub enum Action {
    ExitToMenu,
    GameOver,
    Pause,
    QuitGame,
    Resume,
    StartGame,
}

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
