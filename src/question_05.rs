use std::io;

pub fn run() {
	println!("Please enter number between 1-10");

	let mut user_input = String::new();
	io::stdin().read_line(&mut user_input).expect("Failed to read number");
	println!("\n");

	get_line(&mut user_input);
	
}

fn get_line(num: &mut String) {
	let number = num.trim().parse().unwrap();
	let mut index = 1;

	while index <= number {
		if index == 1 {
			println!("*");
		} 
		else if index == 2 {
			println!("**");
		}
		else if index == 3 {
			println!("***");
		}
		else if index == 4 {
			println!("****");
		}
		else if index == 5 {
			println!("*****");
		}
		else if index == 6 {
			println!("******");
		}
		else if index == 7 {
			println!("*******");
		}
		else if index == 8 {
			println!("********");
		}
		else if index == 9 {
			println!("*********");
		}
		else if index == 10 {
			println!("**********");
		}
		
		index += 1;
	}
}