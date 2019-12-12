pub fn run() {
	let rect = Rectangle { width: 100, height: 100 };

	println!("Sum of width and height : {}", rect.sum() );
}

impl Rectangle { 
    fn sum(&self) -> u32 { 
        self.width + self.height 
    }
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}
