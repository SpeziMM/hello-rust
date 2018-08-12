use std::io;

struct Rechnen {
	x:f64,
	y:f64,
	c:f64,
	
	
}

impl Rechnen {
	
	fn Multi(&self)-> f64{
	self.x *self.y - self.c
}	
	fn adi(&self)-> f64{
	self.x + self.y + self.c
}
	fn sub(&self)-> f64{
	self.x - self.y	- self.c
}
	fn div(&self)-> f64{
	self.x / self.y	-self.c
}	
	
	}
	


 
	


fn main() {

	
	
    println!("Bitte gib deine erst zahl an.");
    let mut guess1 = String::new();
    io::stdin().read_line(&mut guess1)
        .expect("Failed to read line");
		let m = "2";
	let guess_int = guess1.as_str().parse::<i32>().unwrap();
	println!("{}",guess_int+1);
		//
		//
    println!("Bitte gib deine zweite zahl an.");
    let mut guess2 = String::new();
    io::stdin().read_line(&mut guess2)
        .expect("Failed to read line");
		//
		//
	    println!("Bitte gib deine dritte zahl an.");
	    let mut guess3 = String::new();
	    io::stdin().read_line(&mut guess3)
	        .expect("Failed to read line");
		
		}
	
