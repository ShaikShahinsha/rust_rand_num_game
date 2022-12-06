
use std::io;
extern crate rand;

//importing external crate

use rand::random;

fn get_guess() -> u8{

    loop{
        println!("entering into loop and access input");

        let mut guess = String::new();

        io::stdin().read_line(& mut guess).expect("could not read from stdin");

        match guess.trim().parse::<u8>(){
            Ok(v) => return v,
            Err(e) => println!("couldn't understandinput {}",e)
        }
        
    }

}

fn handle_guess(guess:u8, correct:u8) -> bool{

    if guess < correct {
        println!("too low");
        false
    }else if guess > correct{
        println!("too high");
        false
    }else{
        println!("you go the match...");
        true
    }
}
fn main() {
    println!("Welcome to no guessing game");
    let correct:u8 = random();

    println!("correct values is {}",correct);

    loop{

        let guess = get_guess();
        if handle_guess(guess, correct){
            break;
        }
    }

}
