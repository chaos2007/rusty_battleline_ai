extern crate rusty_battleline_interface as rbi;
use std::io;


fn main() {
    loop {
        
        let mut message = String::new();
        io::stdin().read_line(&mut message)
            .expect("failed to read line");

        let x = rbi::parse_message(message);
        match x {
            rbi::Message::PlayerDirection{ direction: rbi::Direction::North } => {
                println!("player north rusty_battleline_bot");
            },
            rbi::Message::PlayerDirection{ direction: rbi::Direction::South } => {
                println!("player south rusty_battleline_bot");
            },
            rbi::Message::PlayCard => {
                println!("play 1 red,1");
            },
            _ => {
            }
        }
    }
}

