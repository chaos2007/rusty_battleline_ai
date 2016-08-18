extern crate rusty_battleline_interface as rbi;
use std::io;
use std::io::prelude::*;
use std::fs::File;

struct Ai {
}


impl rbi::game_state::AiInterface for Ai {
    fn update_game_state(&self, state: &rbi::game_state::GameState) -> String {
        let mut my_cards = state.player_hand.clone();
        let mut claims = state.claim_status.clone();
        let mut my_flags = state.player_side.clone();
        let mut colors = state.colors.clone();
        my_cards.sort_by_key(|k| k.number);
        let functions: Vec<fn(&Vec<rbi::message_parsing::Card>,
                              &Vec<rbi::message_parsing::Card>,
                              &Vec<rbi::message_parsing::Color>)
                              -> (bool, rbi::message_parsing::Card)> =
            vec![check_for_wedge, check_for_phalanx, check_for_battalion, check_for_skirmish];
        for function in functions {
            for (x, claimed) in claims.iter().enumerate() {
                if my_flags[x].len() <= 3 {
                    match *claimed {
                        rbi::message_parsing::ClaimStatus::Unclaimed => {
                            match function(&my_cards, &my_flags[x], &state.colors_vec) {
                                (true, card) => {
                                    let color_string = state.string_from_color(card.color);
                                    return format!("play {} {},{}",
                                                   x + 1,
                                                   color_string,
                                                   card.number);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
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
    //  let mut f = File::create("debug.txt").unwrap();
    //    f.write_all(b"======Start Log=======\n");
    //   f.sync_all();
    loop {
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("failed to read line");
        handler.run_one_round(&ai, message);
        // f.write_fmt(format_args!("PlayerHand: {:?}\nPlayerFlags: {:?}\nopponentFlags: {:?}\n",
        // handler.state.player_hand,
        // handler.state.player_side,
        // handler.state.opponent_side));
        // f.write_all(b"=====================================================================\n");
        // f.flush();
    }
}

fn check_for_phalanx(hand: &Vec<rbi::message_parsing::Card>,
                     flag_cards: &Vec<rbi::message_parsing::Card>,
                     colors_vec: &Vec<rbi::message_parsing::Color>)
                     -> (bool, rbi::message_parsing::Card) {
    let mut card: Option<rbi::message_parsing::Card> = None;
    for card_num in 1..10 {
        let mut num = 0;
        let mut tempCard: Option<rbi::message_parsing::Card> = None;
        for x in hand {
            match x {
                &rbi::message_parsing::Card { color, number } if card_num == number => {
                    num += 1;
                    tempCard = match tempCard {
                        Some(card) => Some(card),
                        _ => Some(x.clone()),
                    }
                }
                _ => {}
            }
        }
        let mut phalanx_flag_nums = 0;
        for x in flag_cards {
            match x {
                &rbi::message_parsing::Card { color, number } if card_num == number => {
                    phalanx_flag_nums += 1;
                }
                _ => {}
            }
        }
        num = if flag_cards.len() != phalanx_flag_nums {
            0
        } else {
            phalanx_flag_nums + num
        };
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


fn check_for_battalion(hand: &Vec<rbi::message_parsing::Card>,
                       flag_cards: &Vec<rbi::message_parsing::Card>,
                       colors_vec: &Vec<rbi::message_parsing::Color>)
                       -> (bool, rbi::message_parsing::Card) {
    let mut card: Option<rbi::message_parsing::Card> = None;
    for color_string in colors_vec {
        let mut num = 0;
        let mut tempCard: Option<rbi::message_parsing::Card> = None;
        for x in hand {
            match x {
                &rbi::message_parsing::Card { color, number } if *color_string == color => {
                    num += 1;
                    tempCard = match tempCard {
                        Some(rbi::message_parsing::Card { number: numbera, .. }) if number >
                                                                                    numbera => {
                            Some(rbi::message_parsing::Card {
                                color: color,
                                number: number,
                            })
                        }
                        Some(card) => Some(card),
                        _ => Some(x.clone()),
                    }
                }
                _ => {}
            }
        }
        let mut phalanx_flag_nums = 0;
        for x in flag_cards {
            match x {
                &rbi::message_parsing::Card { ref color, .. } if color_string == color => {
                    phalanx_flag_nums += 1;
                }
                _ => {}
            }
        }
        num = if flag_cards.len() != phalanx_flag_nums {
            0
        } else {
            phalanx_flag_nums + num
        };
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

fn check_for_wedge(hand: &Vec<rbi::message_parsing::Card>,
                   flag_cards: &Vec<rbi::message_parsing::Card>,
                   colors_vec: &Vec<rbi::message_parsing::Color>)
                   -> (bool, rbi::message_parsing::Card) {
    let mut card: Option<rbi::message_parsing::Card> = None;
    let mut a = hand.clone();
    let mut b = flag_cards.clone();
    a.append(&mut b);
    for card_hand in &a {
        let mut num = 1;
        let mut tempCard: Option<rbi::message_parsing::Card> = None;
        let lower_num = card_hand.number - 1;
        let higher_num = card_hand.number + 1;
        let color_card = card_hand.color;
        for another_card_hand in hand {
            if card_hand == another_card_hand {
                continue;
            }
            match another_card_hand {
                &rbi::message_parsing::Card { color, number } if (lower_num == number ||
                                                                  higher_num == number) &&
                                                                 color == color_card => {
                    num += 1;
                    tempCard = match tempCard {
                        Some(card) => Some(card),
                        _ => Some(another_card_hand.clone()),
                    }
                }
                _ => {}
            }
        }
        let mut wedge_flag_nums = 0;
        for another_card_hand in flag_cards {
            if card_hand == another_card_hand {
                continue;
            }
            match another_card_hand {
                &rbi::message_parsing::Card { color, number } if (lower_num == number ||
                                                                  higher_num == number) &&
                                                                 color == color_card => {
                    wedge_flag_nums += 1;
                }
                _ => {}
            }
        }
        num = if flag_cards.len() != wedge_flag_nums {
            0
        } else {
            wedge_flag_nums + num
        };
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

fn check_for_skirmish(hand: &Vec<rbi::message_parsing::Card>,
                      flag_cards: &Vec<rbi::message_parsing::Card>,
                      colors_vec: &Vec<rbi::message_parsing::Color>)
                      -> (bool, rbi::message_parsing::Card) {
    let mut card: Option<rbi::message_parsing::Card> = None;
    let mut a = hand.clone();
    let mut b = flag_cards.clone();
    a.append(&mut b);
    for card_hand in &a {
        let mut num = 1;
        let mut tempCard: Option<rbi::message_parsing::Card> = None;
        let lower_num = card_hand.number - 1;
        let higher_num = card_hand.number + 1;
        for another_card_hand in hand {
            if card_hand == another_card_hand {
                continue;
            }
            match another_card_hand {
                &rbi::message_parsing::Card { number, .. } if (lower_num == number ||
                                                               higher_num == number) => {
                    num += 1;
                    tempCard = match tempCard {
                        Some(card) => Some(card),
                        _ => Some(another_card_hand.clone()),
                    }
                }
                _ => {}
            }
        }
        let mut wedge_flag_nums = 0;
        for another_card_hand in flag_cards {
            if card_hand == another_card_hand {
                continue;
            }
            match another_card_hand {
                &rbi::message_parsing::Card { number, .. } if (lower_num == number ||
                                                               higher_num == number) => {
                    wedge_flag_nums += 1;
                }
                _ => {}
            }
        }
        num = if flag_cards.len() != wedge_flag_nums {
            0
        } else {
            wedge_flag_nums + num
        };
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
                            color: mp::Color::Color2,
                            number: 8,
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
    fn verify_battalion_3() {
        let hand = vec![mp::Card {
                            color: mp::Color::Color1,
                            number: 4,
                        },
                        mp::Card {
                            color: mp::Color::Color2,
                            number: 5,
                        },
                        mp::Card {
                            color: mp::Color::Color3,
                            number: 1,
                        },
                        mp::Card {
                            color: mp::Color::Color4,
                            number: 10,
                        },
                        mp::Card {
                            color: mp::Color::Color1,
                            number: 2,
                        },
                        mp::Card {
                            color: mp::Color::Color5,
                            number: 7,
                        },
                        mp::Card {
                            color: mp::Color::Color3,
                            number: 7,
                        }];
        let flag = vec![mp::Card {
                            color: mp::Color::Color3,
                            number: 9,
                        }];
        let (x, y) = super::check_for_battalion(&hand, &flag, &get_colors());
        assert!(x);
        assert_eq!(mp::Card {
                       color: mp::Color::Color3,
                       number: 7,
                   },
                   y);
        let flag = vec![mp::Card {
                            color: mp::Color::Color2,
                            number: 10,
                        }];
        let (x, y) = super::check_for_battalion(&hand, &flag, &get_colors());
        assert!(!x);
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

    #[test]
    fn verify_phalanx_cant_place_on_already_placed_flag() {
        let hand = vec![mp::Card {
                            color: mp::Color::Color1,
                            number: 7,
                        },
                        mp::Card {
                            color: mp::Color::Color2,
                            number: 7,
                        },
                        mp::Card {
                            color: mp::Color::Color3,
                            number: 7,
                        }];
        let flag = vec![mp::Card {
                            color: mp::Color::Color1,
                            number: 3,
                        }];
        let (x, _) = super::check_for_phalanx(&hand, &flag, &get_colors());
        assert!(!x);
    }

    #[test]
    fn verify_wedge() {
        let hand = vec![mp::Card {
                            color: mp::Color::Color1,
                            number: 5,
                        },
                        mp::Card {
                            color: mp::Color::Color1,
                            number: 6,
                        },
                        mp::Card {
                            color: mp::Color::Color2,
                            number: 1,
                        }];
        let flag = vec![mp::Card {
                            color: mp::Color::Color1,
                            number: 4,
                        }];
        let (x, _) = super::check_for_wedge(&hand, &flag, &get_colors());
        assert!(x);
    }

    #[test]
    fn verify_skirmish() {
        let hand = vec![mp::Card {
                            color: mp::Color::Color1,
                            number: 5,
                        },
                        mp::Card {
                            color: mp::Color::Color2,
                            number: 6,
                        },
                        mp::Card {
                            color: mp::Color::Color2,
                            number: 1,
                        }];
        let flag = vec![mp::Card {
                            color: mp::Color::Color3,
                            number: 4,
                        }];
        let (x, _) = super::check_for_skirmish(&hand, &flag, &get_colors());
        assert!(x);
    }
}
