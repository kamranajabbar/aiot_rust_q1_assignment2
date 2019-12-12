pub fn run() {
	let tri1 = Triangle { length1: 100, length2: 100, length3: 100 };
	let tri2 = Triangle { length1: 25, length2: 80, length3: 60 };
	
	println!("Sum of width and height for given triangle : {}", tri1.sum());
	println!("Triangle side with the greatest value : {}", tri2.largest_size(&tri2));
}

impl Triangle {
    fn sum(&self) -> u32 { 
        self.length1 + self.length2 + self.length3
    }
}

impl Triangle {
    fn largest_size(&self, tri: &Triangle) -> u32 { 
        tri.length2
    }
}

#[derive(Debug)]
struct Triangle {
	length1: u32,
	length2: u32,
	length3: u32,
}
