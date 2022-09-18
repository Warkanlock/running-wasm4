use crate::{
    ecs::player::MainPlayer,
    wasm4::{self},
};

pub struct MainGame {
    player: MainPlayer,
}

impl MainGame {
    pub fn new_game() -> Self {
        Self {
            player: MainPlayer::new("txxnano"),
        }
    }

    pub fn update(&mut self) {
        // take user input
        self.input();
        // update positions
        self.player.update();
        // draw
        self.player.draw();
    }

    pub fn input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };

        if gamepad & wasm4::BUTTON_LEFT != 0 {
            self.player.move_to("x", -self.player.speed);
        }
        if gamepad & wasm4::BUTTON_RIGHT != 0 {
            self.player.move_to("x", self.player.speed);
        }
        if gamepad & wasm4::BUTTON_UP != 0 {
            self.player.move_to("y", -self.player.speed);
        }
        if gamepad & wasm4::BUTTON_DOWN != 0 {
            self.player.move_to("y", self.player.speed);
        }
    }
}
