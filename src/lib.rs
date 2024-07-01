use std::io;	// bring in input/output from std library

// Easier way to hold color info
pub struct ColorProfile {
	colorspace: String,
	gamma: String,
}

pub fn get_input() -> ColorProfile{

	// prompt for user
	println!("
		[0] ACES AP0
		[1] ARRI Wide Gamut 3
		[2] RED Wide Gamut RGB
		[3] Sony S-Gamut3.Cine

		What is your input color space?
	");
	
	let mut input = String::new();	// creates mutable variable empty string for input color space
	
	// loop incase there is an error
	let ics = loop {
		io::stdin()	// trying to read user input
			.read_line(&mut input)
			.expect("Failed to read input color space");

		let ics: u8 = match input.trim().parse() {	// converts input from string to number, if possible
			Ok(num) => num,
			Err(_) => {
						println!("Please input a number");	// otherwise loop restarts
						continue
			},							
		};
		
		break ics;
	}
	
	let new = ics + 3;
	println!("{}", new);
	
	/*
	
	// creates recommended gamma
	let mut rec = String::new();	// creates mutable variable empty string for recommended gamma
	match ics {
		1 => rec = String::from("ARRI LogC3"),
		2 => rec = String::from("RED Log3G10"),
		3 => rec = String::from("Sony S-Log3"),
		other => rec = String::from("Linear"),
	};

	// prompt for user
	println!("
		[Enter] {rec}\n
		[0] Linear\n
		[1] ARRI LogC3\n
		[2] RED Log3G10\n
		[3] Sony S-Log3\n
		\n
		What is your input gamma?
	");
	
	let mut ig = String::new();	// creates mutable variable empty string for input gamma
	
	loop{
		io::stdin()	// trying to read user input
			.read_line(&mut ig)
			.expect("Failed to read input gamma");

		let ig: u8 = match ig.trim().parse() {	// converts input from string to number, if possible
			Ok(num) => num,
			Err(_) => {
						println!("You input {ig}, please input a number");	// otherwise loop restarts
						continue
			},							
		break;
		};
	}
	
	*/
	
	let result = ColorProfile {
		colorspace: String::from("test colorspace"),
		gamma: String::from("test gamma"),
	};
	
	result
}	
