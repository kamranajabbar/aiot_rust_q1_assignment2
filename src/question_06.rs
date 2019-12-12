pub fn run() {
	let student1 = Student {
		name: String::from("Kamran Jabbar"), 
		email: String::from("kamranajabbar@gmail.com"),
		phone_number: 02135011257,
		gender: String::from("Male"),
	};

	let student2 = Student {
		name: String::from("Muhammad Kashan"), 
		email: String::from("m.kashan@gmail.com"),
		phone_number: 02136033259,
		gender: String::from("Male"),
	};

	println!("Email address of student 1 : {}", student1.email);
	println!("All fields of student 2 : {:#?}", student2); 
}

#[derive(Debug)]
struct Student {
	name: String,
	email: String,
	phone_number: u32,
	gender: String,
}