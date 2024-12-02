mod util;
mod game;
mod textgame;
use util::{Random, WasmRandom};
use wasm_bindgen::prelude::*;
use textgame::textgame::TextGame;



#[wasm_bindgen]
extern "C" {
    fn getRandomNumber(max: u8) -> u8;
}

#[wasm_bindgen]
extern "C" {
    fn addLine(line: String);
}

fn external_write_fn(line : String) {
    addLine(line);
}

#[wasm_bindgen]
pub struct WasmGame {
    textgame:TextGame
}

#[wasm_bindgen]
impl WasmGame {
    pub fn new() -> Self {
        WasmGame {
            textgame : TextGame::new(external_write_fn)
        }
    }

    pub fn game_action(&mut self, action: String) {
        let mut wr = WasmRandom::new();
        wr.set_random_function(getRandomNumber);
        self.textgame.text_action(action, &wr);
    }

    pub fn next_instruction(&mut self) {
        self.textgame.next_instruction_text();
    }
    
    pub fn start_game(&mut self) {
        let mut wr = WasmRandom::new();
        wr.set_random_function(getRandomNumber);
        let _ = &self.textgame.start(&wr);
    }   
}


