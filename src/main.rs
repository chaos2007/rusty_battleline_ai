extern crate rusty_battleline_interface as rbi;
use std::io;

struct Ai {
}

impl rbi::game_state::AiInterface for Ai {
    fn update_game_state(&self, state: &rbi::game_state::GameState) -> String {
        let mut my_cards = state.player_hand.clone();
        let mut claims = state.claim_status.clone();
        claims.reverse();
        my_cards.sort_by_key(|k| k.number);
        for (x, claimed) in claims.iter().enumerate() {
            match *claimed {
                rbi::message_parsing::ClaimStatus::Unclaimed => {
                    match my_cards.last() {
                        Some(card) => {
                            return format!("play {} {},{}", x, card.color, card.number);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        return String::from("play 1 red,1");
    }

    fn get_bot_name(&self) -> String {
        return String::from("rusty_battleline_bot");
    }
}

fn main() {
    let mut handler: rbi::game_state::GameHandler = Default::default();
    let ai = Ai {};
    loop {
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("failed to read line");
        handler.run_one_round(&ai, message);
    }
}
