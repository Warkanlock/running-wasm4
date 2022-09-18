// src/lib.rs
#[cfg(feature = "buddy-alloc")]
mod alloc;
mod ecs;
mod game;
mod wasm4;
use game::MainGame;
use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm4::PALETTE;

lazy_static! {
    static ref MAIN_GAME: Mutex<MainGame> = Mutex::new(MainGame::new_game());
}

#[no_mangle]
fn start() {
    unsafe { *PALETTE = [0xfbf7f3, 0xe5b083, 0x426e5d, 0x20283d] }
}

#[no_mangle]
fn update() {
    MAIN_GAME.lock().expect("game_state").update();
}
