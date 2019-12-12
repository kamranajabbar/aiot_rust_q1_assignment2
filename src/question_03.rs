use std::io;

pub fn run() {
	println!("Please input number 1"); 
	let mut num1 = String::new();
	io::stdin().read_line(&mut num1).expect("Failed to read line");

	println!("Please input number 2");
	let mut num2 = String::new();
	io::stdin().read_line(&mut num2).expect("Failed to read line"); 

	println!("Please input number 3");
	let mut num3 = String::new();
	io::stdin().read_line(&mut num3).expect("Failed to read line");
	
	let number1: u32 = num1.trim().parse().expect("Please type number!");
	let number2: u32 = num2.trim().parse().expect("Please type number!");
	let number3: u32 = num3.trim().parse().expect("Please type number!");

	let avg_value = (number1 + number2 + number3) / 3;

	println!("Average value of your input numbers is : {}", avg_value);
}
