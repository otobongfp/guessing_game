use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("A GUESSING GAME BUILT WITH RUST");

    println!("Pick a number between 1 and 10");


    //Random Number Generator
    let _secret_number :u32 = rand::thread_rng().gen_range(1..=10);


    loop {
        //Variable to collect the guess
        let mut guess = String::new();

        //read the input from the cli
        io::stdin().read_line(&mut guess).expect("something went wrong");

        //convert guess from string to Int
        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("{} is smaller than secret number 😥", guess),
            Ordering::Greater => println!("{} is bigger than the secret number 😥", guess),
            Ordering::Equal => {
                println!("Correct, you got it 😀");
                break;
            },
        }
    }

}