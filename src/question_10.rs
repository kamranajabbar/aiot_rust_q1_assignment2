use std::io;

pub fn run() {
	println!("Please enter name"); 
	let mut name = String::new();
	io::stdin().read_line(&mut name).expect("Failed to read line");

	println!("Please enter age");
	let mut age = String::new();
	io::stdin().read_line(&mut age).expect("Failed to read line"); 

	println!("Please enter country");
	let mut country = String::new();
	io::stdin().read_line(&mut country).expect("Failed to read line");
	
	let person = Person {
		name: name,
		age: age,
		country: country,
	};

	let my_array = [person.name.trim(), person.age.trim(), person.country.trim()];

	println!("Print New Created Array : {:?}", my_array);
}

#[derive(Debug)]
struct Person {
	name: String,
	age: String,
	country: String,
}
