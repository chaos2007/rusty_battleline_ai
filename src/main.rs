use std::io;


fn main() {
    loop {
        
        let mut message = String::new();
        io::stdin().read_line(&mut message)
            .expect("failed to read line");

        //player <north/south> name (player <north/south> <name>)
        //go play-card (play 1 red,3)
        println!("{}", message);

    }
}

