extern crate rusty_battleline_interface as rbi;
use std::io;

struct Ai {
}

impl rbi::game_state::AiInterface for Ai {
    fn update_game_state(&self, state: &rbi::game_state::GameState) -> String {
        let mut my_cards = state.player_hand.clone();
        let mut claims = state.claim_status.clone();
        let mut my_flags = state.player_side.clone();
        //claims.reverse();
        my_flags.reverse();
        my_cards.sort_by_key(|k| k.number);
        for (x, claimed) in claims.iter().enumerate() {
            if (my_flags[x].len() <= 3) {
                match *claimed {
                    rbi::message_parsing::ClaimStatus::Unclaimed => {
                        match my_cards.last() {
                            Some(card) => {
                                return format!("play {} {},{}", x+1, card.color, card.number);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        return String::from("play 1 red,1");
    }

    fn get_bot_name(&self) -> String {
        return String::from("rusty_battleline_bot_wip");
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


#[cfg(test)]
mod test_game_state {
    use super::*;
    use rbi::game_state::AiInterface;
    use rbi::message_parsing;
    use rbi::message_parsing as mp;
    extern crate rusty_battleline_interface as rbi;

    #[test]
    fn playHighestOnLast() {
        let ai = super::Ai{};
        let mut state:rbi::game_state::GameHandler = Default::default();
        state.run_one_round(&ai, String::from("colors a b c d e f"));
        state.run_one_round(&ai, String::from("player north name"));
        state.state.player_hand = vec![mp::Card{color:String::from("a"), number:7},
                                       mp::Card{color:String::from("b"), number:10},
                                       mp::Card{color:String::from("c"), number:3}];
        let response = ai.update_game_state(&(state.state));
        assert_eq!("play 9 b,10", response);
    }
}
