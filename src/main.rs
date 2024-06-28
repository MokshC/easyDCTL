use std::io;	// bring in input/output from std library
use rand::Rng;	// brings in rng from rand lib
use std::cmp::Ordering;	//brings in <, >, = comparisons

fn main() {		// create main function

	// requesting input from user using print lines
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100); // gets rand num range 1-100    

	loop {	// keeps looping until there is an error
	
	    println!("Please input your guess.");    
    	let mut guess = String::new();	// creates mutable variable "guess" bound to "new()" which is an empty string

		io::stdin()	// using the io imported earlier, we can read a line from user
		    .read_line(&mut guess)	// this appends user input as mutable, the "&" shows this var was already created above and doesn't need to double up memory
		    .expect("Failed to read line");	// handling potential error
		// NOTE: the above could be written as one line, but it is ugly io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {	// converts guess from string to number, if possible
            Ok(num) => num,
            Err(_) => continue,							// otherwise loop restarts
        };

		println!("You guessed: {guess}");	// {} are used to represent variables
		println!("The secret number is: {secret_number}");
		
		// match outputs    
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
