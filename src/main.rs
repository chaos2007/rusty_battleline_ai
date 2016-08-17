extern crate rusty_battleline_interface as rbi;
use std::io;

struct Ai {
}


impl rbi::game_state::AiInterface for Ai {
    fn update_game_state(&self, state: &rbi::game_state::GameState) -> String {
        let mut my_cards = state.player_hand.clone();
        let mut claims = state.claim_status.clone();
        let mut my_flags = state.player_side.clone();
        let mut colors = state.colors.clone();
        // claims.reverse();
        my_flags.reverse();
        my_cards.sort_by_key(|k| k.number);
        for (x, claimed) in claims.iter().enumerate() {
            if my_flags[x].len() <= 3 {
                match *claimed {
                    rbi::message_parsing::ClaimStatus::Unclaimed => {
                        match check_for_battalion(&my_cards, &my_flags[x], &state.colors_vec) {
                            (true, card) => {
                                let color_string = state.string_from_color(card.color);
                                return format!("play {} {},{}", x + 1, color_string, card.number);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        for (x, claimed) in claims.iter().enumerate().rev() {
            if my_flags[x].len() <= 3 {
                match *claimed {
                    rbi::message_parsing::ClaimStatus::Unclaimed => {
                        match my_cards.last() {
                            Some(card) => {
                                let color_string = state.string_from_color(card.color);
                                return format!("play {} {},{}", x + 1, color_string, card.number);
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

fn check_for_battalion(hand: &Vec<rbi::message_parsing::Card>,
                       flag_cards: &Vec<rbi::message_parsing::Card>,
                       colors_vec: &Vec<rbi::message_parsing::Color>)
                       -> (bool, rbi::message_parsing::Card) {
    let mut card: Option<rbi::message_parsing::Card> = None;
    let hand = hand.clone();
    let flag_cards = flag_cards.clone();
    for color_string in colors_vec {
        let mut num = 0;
        let mut tempCard: Option<rbi::message_parsing::Card> = None;
        for x in &hand {
            match x {
                &rbi::message_parsing::Card { ref color, number } if color_string == color => {
                    num += 1;
                    tempCard = match tempCard {
                        Some(rbi::message_parsing::Card { color: ref colora, number: numbera }) => {
                            Some(rbi::message_parsing::Card {
                                color: (*colora).clone(),
                                number: numbera,
                            })
                        }
                        _ => Some(x.clone()),
                    }
                }
                _ => {}
            }
        }
        for x in flag_cards.clone() {
            match x {
                rbi::message_parsing::Card { ref color, .. } if color_string == color => {
                    num += 1;
                }
                _ => {}
            }
        }
        if num >= 3 && tempCard != None {
            card = tempCard;
        }
    }
    match card {
        Some(ref x) => return (true, x.clone()),
        _ => {
            return (false,
                    rbi::message_parsing::Card {
                color: rbi::message_parsing::Color::Color1,
                number: 1,
            })
        }
    }
}

#[cfg(test)]
mod test_game_state {
    use super::*;
    use rbi::game_state::AiInterface;
    use rbi::message_parsing;
    use rbi::message_parsing as mp;
    extern crate rusty_battleline_interface as rbi;

    fn get_colors() -> Vec<mp::Color> {
        vec![mp::Color::Color1,
             mp::Color::Color2,
             mp::Color::Color3,
             mp::Color::Color4,
             mp::Color::Color5,
             mp::Color::Color6]
    }

    #[test]
    fn verify_battalion_1() {
        let hand = vec![mp::Card {
                            color: mp::Color::Color1,
                            number: 7,
                        },
                        mp::Card {
                            color: mp::Color::Color1,
                            number: 1,
                        }];
        let flag = vec![mp::Card {
                            color: mp::Color::Color1,
                            number: 6,
                        }];
        let (x, y) = super::check_for_battalion(&hand, &flag, &get_colors());
        assert!(x);
        assert_eq!(mp::Card {
                       color: mp::Color::Color1,
                       number: 7,
                   },
                   y);
    }

    #[test]
    fn verify_battalion_2() {
        let hand = vec![mp::Card {
                            color: mp::Color::Color1,
                            number: 7,
                        },
                        mp::Card {
                            color: mp::Color::Color2,
                            number: 1,
                        }];
        let flag = vec![mp::Card {
                            color: mp::Color::Color1,
                            number: 6,
                        }];
        let (x, _) = super::check_for_battalion(&hand, &flag, &get_colors());
        assert!(!x);
    }

    #[test]
    fn playHighestOnLast() {
        let ai = super::Ai {};
        let mut state: rbi::game_state::GameHandler = Default::default();
        state.run_one_round(&ai, String::from("colors a b c d e f"));
        state.run_one_round(&ai, String::from("player north name"));
        state.state.player_hand = vec![mp::Card {
                                           color: mp::Color::Color1,
                                           number: 7,
                                       },
                                       mp::Card {
                                           color: mp::Color::Color2,
                                           number: 10,
                                       },
                                       mp::Card {
                                           color: mp::Color::Color3,
                                           number: 3,
                                       }];
        let response = ai.update_game_state(&(state.state));
        assert_eq!("play 9 b,10", response);
        state.state.player_hand = vec![mp::Card {
                                           color: mp::Color::Color1,
                                           number: 7,
                                       },
                                       mp::Card {
                                           color: mp::Color::Color3,
                                           number: 3,
                                       }];
        let response = ai.update_game_state(&(state.state));
        assert_eq!("play 9 a,7", response);
        state.state.player_hand = vec![mp::Card {
                                           color: mp::Color::Color3,
                                           number: 3,
                                       }];
        let response = ai.update_game_state(&(state.state));
        assert_eq!("play 9 c,3", response);
    }
}
