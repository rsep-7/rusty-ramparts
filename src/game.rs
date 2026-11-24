//! Core game logic and application state management.
//!
//! Owns the game state, defines valid state transitions,
//! processes actions, and manages the game lifecycle.
//! Independent of input handling and rendering.

/// Owns the game state and manages its lifecycle
pub struct Game {
    state: GameState,
    running: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents the current state of the game
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

/// Actions that can be triggered by player input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    ExitToMenu,
    GameOver,
    Pause,
    QuitGame,
    Resume,
    StartGame,
}

/// Represents the effect of a valid action
enum ActionEffect {
    Transition(GameState),
    Quit,
}

impl Game {
    /// Creates a new game in the main menu
    pub fn new() -> Self {
        Game {
            state: GameState::MainMenu,
            running: true,
        }
    }

    /// Returns whether the application is still running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Returns the current game state
    pub fn state(&self) -> GameState {
        self.state
    }

    /// Resolves an action without modifying the game state
    fn resolve_action(&self, action: Action) -> Option<ActionEffect> {
        match (self.state, action) {
            (GameState::MainMenu, Action::StartGame) => {
                Some(ActionEffect::Transition(GameState::Playing))
            }
            (GameState::Playing, Action::Pause) => {
                Some(ActionEffect::Transition(GameState::Paused))
            }
            (GameState::Paused, Action::Resume) => {
                Some(ActionEffect::Transition(GameState::Playing))
            }
            (GameState::Paused, Action::ExitToMenu) | (GameState::GameOver, Action::ExitToMenu) => {
                Some(ActionEffect::Transition(GameState::MainMenu))
            }
            (GameState::Playing, Action::GameOver) => {
                Some(ActionEffect::Transition(GameState::GameOver))
            }
            (GameState::MainMenu, Action::QuitGame) => Some(ActionEffect::Quit),
            _ => None,
        }
    }

    /// Attempts to apply an action in the current game state.
    ///
    /// Returns `true` if the action was handled successfully,
    /// or `false` if it is not valid in the current state.
    pub fn handle_action(&mut self, action: Action) -> bool {
        match self.resolve_action(action) {
            Some(ActionEffect::Transition(state)) => {
                self.state = state;
                true
            }
            Some(ActionEffect::Quit) => {
                self.running = false;
                true
            }
            None => false,
        }
    }

    /// Advances the game logic by one tick.
    pub fn update(&mut self, _dt: f32) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_lifecycle() {
        let mut game = Game::new();

        assert_eq!(game.state(), GameState::MainMenu);
        assert!(game.is_running());

        assert!(game.handle_action(Action::StartGame));
        assert_eq!(game.state(), GameState::Playing);

        assert!(game.handle_action(Action::Pause));
        assert_eq!(game.state(), GameState::Paused);

        assert!(game.handle_action(Action::Resume));
        assert_eq!(game.state(), GameState::Playing);

        assert!(game.handle_action(Action::GameOver));
        assert_eq!(game.state(), GameState::GameOver);

        assert!(game.handle_action(Action::ExitToMenu));
        assert_eq!(game.state(), GameState::MainMenu);

        assert!(game.handle_action(Action::QuitGame));
        assert!(!game.is_running());
    }

    #[test]
    fn invalid_action_preserves_state() {
        let mut game = Game::new();

        assert!(!game.handle_action(Action::Pause));
        assert_eq!(game.state(), GameState::MainMenu);
        assert!(game.is_running());
    }
}
