use std::io;

pub fn run() {
	println!("Please input string");

	let mut user_string = String::new();
	io::stdin().read_line(&mut user_string).expect("Failed to read line");

	let get_str_leng = get_len(&mut user_string);
	println!("User input string lenght is : {}", get_str_leng);
}

fn get_len(s: &mut String) -> usize {
	let string_trimed = s.trim();
	string_trimed.len()
}