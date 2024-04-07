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