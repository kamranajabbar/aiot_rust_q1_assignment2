pub fn run() {
	let mut rect1 = Rectangle{
		width: 50,
		height: 100
	};

	rect1.width = 150;

	println!("Width for rect1 after changed is : {}", rect1.width);
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}
