/*

My first rust project. Its the guessing game from the rust docs/book. This time, I'm rewriting it from heart.

*/

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello there! Welcome to quiktea's Guessing Game! Please remember to only type numbers!");
    let secret_num = rand::thread_rng().gen_range(1, 101); //generating a "secret" random number
    loop {
        println!("Please guess a number!"); //just printing out "Please guess a number"
        let mut guess = String::new(); //creating a new mutuable variable called guess. This will be our variable that holds our user's guess.
        io::stdin()
            .read_line(&mut guess) // as references are immutable by default, we must pass it in as a "mutuable referencce".
            .expect("Failed to read input");
        
        let guess: u32 = match guess.trim().parse() { //trying to "convert" our guess variable value to a u32 number. 
            Ok(num) => num, //if the convert works, we take the result, store it in a variable called "num" and just spit that variable value out
            Err(_) => { // if the convert fails, we run a function
                println!("Invalid Number!"); //we print out that its an invalid number
                continue; //we continue in the loop
            }
        };

        match guess.cmp(&secret_num) { //matching it to the secret_num (comparing it with the cmp lib)
            Ordering::Less => println!("Too Small!"), // if its smaller than the secret_num reference, we print out "Too Small!"
            Ordering::Greater => println!("Too Large!"), //if its larger than the secret_num reference, we print out "Too Large!"
            Ordering::Equal => { //if its equal to the secret_num reference, we run a function
                println!("Amazing Job! You guessed the right number!"); //here we print out that its the right answer
                break; //we break the loop here to stop it from running
            }
        }

        
    }
}