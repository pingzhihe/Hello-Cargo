
use std::io; //prelude
use rand::Rng; // trait
use std::cmp::Ordering; // enum

fn main() {
    println!("Guessing Game!");

    println!("Guess a number between 1 and 100");
    let secret_number = rand::thread_rng().gen_range(1..101);         
    //  println!("The secret number is: {}", secret_number);


    loop {
        let mut guess = String::new();  
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        // shadowing
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), //arm
            Ordering::Greater => println!("Too big!"), 
            Ordering::Equal => {println!("You win!"); break;},
        }
        
    }


}
