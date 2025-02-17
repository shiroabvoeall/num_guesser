use std::io;
use std::io::Write;
use std::{thread, time};
use rand;

fn main() {
    let mut response_ans: String = String::new();
    let mut player_health: i8 = 0;
    
    print!("Welcome!\nWould you want to play? ( y / n ) : "); 
    text_flush(); text_read(&mut response_ans); 

    verify(&mut response_ans, &mut player_health);
}

fn the_game(response_ans: &mut String, player_health: &mut i8) {
    let number_to_guess = number_ans();
    let mut the_guess = String::new();
    *player_health += 6;

    loop {
        if *player_health < 1 {
            print!("\nThe number was {}, nice try though!\nWould you want to play again? ( y / n ) :", number_to_guess);
            text_flush(); 

            response_ans.clear();
            text_read(response_ans); 

            *player_health = 0;
            verify(response_ans, player_health);
            
            break;
        }
        *player_health -= 1;
        print!("\nGuesses left : {}\nInput your answer : ", player_health); text_flush(); text_read(&mut the_guess);
        let trimmed_ans = the_guess.trim();

        match trimmed_ans.parse::<i8>() {
            Ok(guess) => {
                if guess == number_to_guess {
                    print!("The number {} was indeed the right one!\n", guess);
                    break;
                } else {
                    print!("The number {} was incorrect!\n", guess); 
                }
            }
            Err(_) => println!("Was not able to parse!"),
        }

        the_guess.clear();
    }
    
}

fn verify(response_ans: &mut String, player_health: &mut i8) {
    let trimmed_resp = response_ans.trim();
    match response_check(trimmed_resp) {
        true => the_game(response_ans, player_health),
        false => {
            println!("See you around!");
            thread::sleep(time::Duration::new(1, 0));
        },
    }
}
fn number_ans() -> i8 {
    let number_generated = rand::random_range(1..=100); number_generated
}

fn text_flush() {
    io::stdout().flush().expect("Err");
}

fn text_read(response_ans: &mut String) {
    io::stdin().read_line(response_ans).expect("Err");
}

fn response_check(response_ans: &str) -> bool {
    match response_ans.to_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => {
            print!("Your response '{}' is invalid! ", response_ans.trim());
            return false;
        },
    }
} 