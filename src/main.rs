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

    loop {
	    let mut guess = String::new();

	           io::stdin().read_line(&mut guess)
	               .expect("Failed to read line");
		
       
           let guess: u32 = match guess.trim().parse() {
               Ok(num) => num,
               Err(_) => continue,
           };

           println!("You guessed: {}", guess+1);

                   break;
               }
		}
	
