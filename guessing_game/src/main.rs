use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main(){
 
  
    println!("Guess a number between 1 and 100!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        
    
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Enter number between 1-100");
            println!("You guessed: {guess}" );

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("Please enter a valid number");
                continue
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!(" The number is too big"),
            Ordering::Less => println!("The number is too small"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }



}