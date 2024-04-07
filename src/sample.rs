#[derive(Debug)]
pub struct Point{
	x:i32,y:i32
}

impl Point{
	pub fn new(x:i32,y:i32)->Point{
		Point{x,y}
	}
	pub fn x(&self)->i32{
		self.x
	}
	pub fn y(&self)->i32{
		self.y
	}
	
	pub fn assert_x(&self,x:i32){
		assert_eq!(self.x,x);
	}
	
	pub fn assert_y(&self,y:i32){
		assert_eq!(self.y,y);
	}
	
}

#[derive(Debug)]
pub struct Integer(i32);

impl Integer{
	pub fn value(&self)->i32{
		self.0
	}
	
	pub fn assert_value(&self,value:i32){
		assert_eq!(self.0,value);
	}
}

impl From<i32> for Integer{
	fn from(value:i32)->Self{
		Integer(value)
	}
}

#[derive(Debug)]
pub struct Envelope<T>(T);


impl<T> Envelope<T> {
	pub fn item(&self)->&T{
		&self.0
	}
}

impl<T> From<T> for Envelope<T>{
	fn from(item:T)->Self{
		Envelope(item)
	}
}