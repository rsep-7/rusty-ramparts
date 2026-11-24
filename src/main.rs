//! # Rusty Ramparts
//!
//! A tower defense game written in Rust using Macroquad.

mod game;
mod keybindings;
mod render;

use keybindings::process_input;
use macroquad::prelude::*;

use crate::game::Game;
use crate::render::draw;

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
    let mut game = Game::new();
    let mut accumulator: f32 = 0.0;

    // Handle input, run game logic in fixed ticks and draw.
    // Logic runs at a fixed tick rate, independent of the display refresh rate.
    while game.is_running() {
        // 1. Input
        for action in process_input() {
            if game.handle_action(action) {
                break; // Stop after the first valid action
            }
        }

        if !game.is_running() {
            break;
        }

        // 2. Update
        accumulator += get_frame_time().min(MAX_FRAME_TIME);
        while accumulator >= TICK_DURATION {
            game.update(TICK_DURATION);
            accumulator -= TICK_DURATION;
        }

        // 3. Draw
        draw(game.state());

        next_frame().await
    }
}
