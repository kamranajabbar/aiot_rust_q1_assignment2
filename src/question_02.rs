// Make a function that takes s variable as parameter/argument but doesn’t take ownership of s variable and concatenates “ZINDABAD” in variable s to makes it “PAKISTAN ZINDABAD”. Print variable s after change.

pub fn run() {
	let mut s = String::from("PAKISTAN");

	add_string(&mut s);

	println!("{}", s);
}

fn add_string(my_str: &mut String) {
	my_str.push_str(" ZINDABAD");
}
