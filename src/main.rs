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
        for (x, claimed) in claims.iter().enumerate().rev() {
            if my_flags[x].len() <= 3 {
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

fn check_for_battalion(hand: Vec<rbi::message_parsing::Card>, 
                       flag_cards: Vec<rbi::message_parsing::Card>,
                       colors: Vec<String>) -> (bool, rbi::message_parsing::Card) {
    let mut card:Option<rbi::message_parsing::Card> = None;
    let hand = hand.clone();
    let flag_cards = flag_cards.clone();
    let colors = colors.clone();
    for color in &colors {
        let mut num = 0;
        let mut tempCard:Option<rbi::message_parsing::Card> = None;
        for x in &hand {
            match x {
                &rbi::message_parsing::Card{color:colora,number:numbera}=> {
                    num += 1;
                    match &tempCard {
                        &Some(rbi::message_parsing::Card{color:colorb, number:numberb}) => {
                            tempCard = Some(rbi::message_parsing::Card {color:colorb, number:numberb}) 
                        },
                        _ => {
                        }
                    }
                }
            }
        }
        for x in flag_cards.clone(){
            match x {
                rbi::message_parsing::Card{color:color,..} => {
                    num += 1;
                }
            }
        }
        if num >= 3 && tempCard != None {
            card = tempCard;
        }
    }
    match card {
        Some(ref x) => return (true,x.clone()),
        _ => return (false, rbi::message_parsing::Card{color:String::from("a"), number:1}),
    }
}

#[cfg(test)]
mod test_game_state {
    use super::*;
    use rbi::game_state::AiInterface;
    use rbi::message_parsing;
    use rbi::message_parsing as mp;
    extern crate rusty_battleline_interface as rbi;

    fn get_colors() -> Vec<String>{
        vec![String::from("a"), String::from("b"), String::from("c"),
                                   String::from("d"), String::from("e"), String::from("f")]
    }

    #[test]
    fn verify_battalion_1() {
        let hand = vec![mp::Card{color:String::from("a"), number:7},
                        mp::Card{color:String::from("a"), number:1}];
        let flag = vec![mp::Card{color:String::from("a"), number:6}];
        let (x, y) = super::check_for_battalion(hand, flag, get_colors());
        assert!(x);
        assert_eq!(mp::Card{color:String::from("a"), number:7}, y);
    }
    
    #[test]
    fn verify_battalion_2() {
        let hand = vec![mp::Card{color:String::from("a"), number:7},
                        mp::Card{color:String::from("b"), number:1}];
        let flag = vec![mp::Card{color:String::from("a"), number:6}];
        let (x, _) = super::check_for_battalion(hand, flag, get_colors());
        assert!(!x);
    }

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
        state.state.player_hand = vec![mp::Card{color:String::from("a"), number:7},
                                       mp::Card{color:String::from("c"), number:3}];
        let response = ai.update_game_state(&(state.state));
        assert_eq!("play 9 a,7", response);
        state.state.player_hand = vec![mp::Card{color:String::from("c"), number:3}];
        let response = ai.update_game_state(&(state.state));
        assert_eq!("play 9 c,3", response);
    }
}
