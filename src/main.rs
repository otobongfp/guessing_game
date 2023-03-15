use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("________A GUESSING GAME_________");
    println!("Guess a number between 1 and 10:");

    let _secret_number = rand::thread_rng().gen_range(1..=10);


    loop{

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Did you pass in a number?");

        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num, 
            Err(_) => continue,
        };

        match guess.cmp(&_secret_number){
            Ordering::Greater => println!("{guess} is bigger than the secret number, Try Again 😥"),
            Ordering::Less => println!("{guess} is smaller than the secret number, Try Again 😥"),
            Ordering::Equal => {
                println!("Correct, You Got It!!! 😀");
                break;
            },
        }
    }
}