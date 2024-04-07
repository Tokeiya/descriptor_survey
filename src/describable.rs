use std::marker::PhantomData;
use super::descriptor::Descriptor;
pub struct Describable<T, F> {
	func: F,
	_phantom: PhantomData<T>,
}

impl<T, S, D, F> Describable<T, F>
	where
	  F: for<'x> Fn(&'x T) -> Descriptor<'x, S, D>,
{
	pub fn new(func: F) -> Self {
		Describable {
			func,
			_phantom: PhantomData,
		}
	}

	pub fn describe<'a>(&self, item: &'a T) -> Descriptor<'a, S, D> {
		(self.func)(item)
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use crate::point::Point;
	fn create_sample()->Point{
		Point::new(42,64)
	}

	#[test]
	fn tuple_source(){
		let fixture=Describable::new(|x:&Point|{
			Descriptor::new(x.y(),[x.x().to_string(),x.y().to_string()])
		});

		let p=create_sample();
		let act=fixture.describe(&p);
		let s=act.source();
		assert_eq!(s,&64)
	}



	#[test]
	fn use_reference(){
		let fixture=Describable::new(|x:&Point|{
			Descriptor::new(x,[x.x().to_string(),x.y().to_string()])
		});
		
		
		let p=create_sample();
		let act=fixture.describe(&p);

	}
}