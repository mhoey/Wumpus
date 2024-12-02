mod game;
mod textgame;
mod util;
use std::io::stdin;
use crate::textgame::textgame::TextGame;
use crate::util::OsRandom;
use crate::util::Random;

fn stdout_fn(text: String) {
    println!("{}",text);
}

fn main() {
    let os_random = OsRandom::new();
    let mut game = 
        TextGame::new(stdout_fn);
    game.start(&os_random);


    while !game.is_game_over() {
        let mut text_action = String::new();
        let _ = stdin().read_line(&mut text_action);
        game.text_action(text_action.to_owned(), &os_random);
    }
 
}