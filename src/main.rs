mod keybindings;
mod state;

use keybindings::{Action, is_action_pressed};
use macroquad::prelude::*;
use state::GameState;

/// Duration of one game logic tick.
const TICK_DURATION: f32 = 1.0 / 60.0;
/// Upper limit for the time added each frame. Prevents spiral of death after frame spikes.
const MAX_FRAME_TIME: f32 = 0.2;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Rusty Ramparts"),
        window_width: 1280,
        window_height: 720,
        high_dpi: false,
        fullscreen: false,
        window_resizable: false,
        icon: None,
        ..Conf::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::MainMenu;
    let mut accumulator: f32 = 0.0;

    // Handle input, run game logic in fixed ticks and draw.
    // Logic runs at a fixed tick rate, independent of the display refresh rate.
    loop {
        // 1. Input
        match game_state {
            GameState::MainMenu => {
                if is_action_pressed(Action::QuitGame) {
                    break;
                }
                if is_action_pressed(Action::StartGame) {
                    game_state = GameState::Playing;
                }
            }

            GameState::Playing => {
                if is_action_pressed(Action::Pause) {
                    game_state = GameState::Paused;
                }
                if is_action_pressed(Action::GameOver) {
                    game_state = GameState::GameOver;
                }
            }

            GameState::Paused => {
                if is_action_pressed(Action::Resume) {
                    game_state = GameState::Playing;
                }
                if is_action_pressed(Action::ExitToMenu) {
                    game_state = GameState::MainMenu;
                }
            }

            GameState::GameOver => {
                if is_action_pressed(Action::ExitToMenu) {
                    game_state = GameState::MainMenu;
                }
            }
        }

        // 2. Update
        accumulator += get_frame_time().min(MAX_FRAME_TIME);
        while accumulator >= TICK_DURATION {
            update(game_state);
            accumulator -= TICK_DURATION;
        }

        // 3. Draw
        draw(game_state);

        next_frame().await
    }
}

/// Advances the game logic by one tick.
fn update(_state: GameState) {}

/// Draws the current game state.
fn draw(state: GameState) {
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
