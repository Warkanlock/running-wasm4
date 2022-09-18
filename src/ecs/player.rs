use crate::wasm4;

const PLAYER_WIDTH: u32 = 12;
const PLAYER_HEIGHT: u32 = 12;
const PLAYER_SPEED: i32 = 2;

pub struct MainPlayer {
    pub name: &'static str,
    pub health: i32,
    pub shield: f32,
    pub x: i32,
    pub y: i32,
    pub speed: i32,
}

impl MainPlayer {
    pub fn new(name: &'static str) -> Self {
        Self {
            x: wasm4::SCREEN_SIZE as i32 / 2,
            y: wasm4::SCREEN_SIZE as i32 / 2,
            health: 100,
            shield: 100.0,
            name,
            speed: PLAYER_SPEED,
        }
    }

    pub fn draw(&self) {
        wasm4::rect(self.x, self.y, PLAYER_WIDTH, PLAYER_HEIGHT);
    }

    pub fn update(&mut self) {
        // do things here
    }

    fn check_edge(&self, number: i32) -> i32 {
        if number < 0 {
            return (wasm4::SCREEN_SIZE + PLAYER_WIDTH) as i32;
        }
        return 0 - PLAYER_WIDTH as i32;
    }

    pub fn move_x(&mut self, new_pos: i32) {
        if self.x >= 0 - PLAYER_WIDTH as i32 && self.x <= (wasm4::SCREEN_SIZE + PLAYER_WIDTH) as i32
        {
            self.x += new_pos;
        } else {
            self.x = self.check_edge(new_pos);
        }
    }

    pub fn move_y(&mut self, new_pos: i32) {
        if self.y >= 0 - PLAYER_WIDTH as i32 && self.y <= (wasm4::SCREEN_SIZE + PLAYER_WIDTH) as i32
        {
            self.y += new_pos;
        } else {
            self.y = self.check_edge(new_pos);
        }
    }
}
