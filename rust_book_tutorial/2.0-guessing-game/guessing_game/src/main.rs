// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::io;
use std::cmp::Ordering;
use rand::Rng;

// fn main() {
//     println!("Guess the number!");
//     println!("Please input your guess.");
//     let apples = 5; // immutable
//     let mut bananas = 5; // mutable

//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//     println!("You guessed: {}", guess);
//     println!("You guessed: {guess}");
//     println!("You guessed: {guess}", guess = guess);
//     // why new line?
//     println!("You guessed: {guess}\n", guess = guess);

//     let x = 5;
//     let y = 10;
    
//     println!("x = {x} and y + 2 = {}", y + 2);    
//     println!("x = {x} and y + 2 = {}", y + 2);    
// }



/**
 * Guess the number!
The secret number is: 39
Please input your guess.
123
You guessed: 123
Too big!
damon@damonui-MacBookPro guessing_game % cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 13
Please input your guess.
cvb
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:42:43
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
 * 
 */

fn main2() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");
    let mut guess = String::new();
    // io::stdin()
        // .read_line(&mut guess);  // .expect가 없으면    = note: this `Result` may be an `Err` variant, which should be handled
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");  // 만약에 숫자가 아니면 에러가 난다.

    // println!("You guessed: {}", guess);

    // match guess.cmp(&secret_number) {
    //     Ordering::Less => println!("Too small!"),
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("You win!"),
    // }

    // loop {
        
    //     println!("Guess the number!");

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => println!("You win!"),
    //     }
    // }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }    
}
