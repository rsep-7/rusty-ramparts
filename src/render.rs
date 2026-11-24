//! Rendering and visual representation of the game.
//!
//! Draws the current game state using Macroquad without
//! modifying the underlying game logic or state.

use crate::game::GameState;
use macroquad::prelude::*;

/// Draws the current game state.
pub fn draw(state: GameState) {
    match state {
        GameState::MainMenu => {
            clear_background(DARKPURPLE);
            draw_text("MAIN MENU", 20.0, 40.0, 40.0, WHITE);
            draw_text("[Esc] close window", 20.0, 60.0, 20.0, WHITE);
            draw_text("[1] start game", 20.0, 80.0, 20.0, WHITE);
        }

        GameState::Playing => {
            clear_background(RED);
            draw_text("PLAYING", 20.0, 40.0, 40.0, WHITE);
            draw_text("[Esc] pause game", 20.0, 60.0, 20.0, WHITE);
            draw_text("[2] enter game over state", 20.0, 80.0, 20.0, WHITE);
        }

        GameState::Paused => {
            clear_background(BLUE);
            draw_text("PAUSED", 20.0, 40.0, 40.0, WHITE);
            draw_text("[Esc] exit to main menu", 20.0, 60.0, 20.0, WHITE);
            draw_text("[1] resume game", 20.0, 80.0, 20.0, WHITE);
        }

        GameState::GameOver => {
            clear_background(GREEN);
            draw_text("GAME OVER", 20.0, 40.0, 40.0, WHITE);
            draw_text("[Esc] exit to main menu", 20.0, 60.0, 20.0, WHITE);
        }
    }
    draw_text(format!("FPS: {}", get_fps()), 1200.0, 20.0, 20.0, WHITE);
}
