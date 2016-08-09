extern crate rusty_battleline_interface as rbi;
use std::io;

struct Ai {
}

impl rbi::game_state::AiInterface for Ai {
    fn update_game_state(&self, state: &rbi::game_state::GameState) -> String {
        return String::from("play 1 red,1");
    }
    
    fn get_bot_name(&self) -> String {
        return String::from("rusty_battleline_bot");
    }
}

fn main() {
    let handler:rbi::game_state::GameHandler = Default::default();
    let ai = Ai{}; 
    loop {
        handler.run_one_round(&ai);
    }
}

