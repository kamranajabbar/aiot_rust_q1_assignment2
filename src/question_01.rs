pub fn run() {
	let x = String::from("PIAIC IoT");
	let y = x;

	println!("{}", x);
}

// Value of "x" variable has been moved into "y" variable. So i would like to say that "x" was moved into "y". As we know that "Rust" invalidated first variable, therefore, we are facing error.