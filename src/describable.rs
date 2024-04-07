use std::marker::PhantomData;
use super::descriptor::{CommonDescriptor,Descriptor};


pub trait Describable<T,S,C:ToString,D:Descriptor<S,C>>{
	fn describe(&self,item:T)->D;
}

pub struct CommonDescribable<T,S,D,F>(F,PhantomData<(T,S,D)>);

impl<T,S,D:ToString,F:Fn(T)-> CommonDescriptor<S,D>> CommonDescribable<T,S,D,F> {
	pub fn new(process:F)->Self{
		CommonDescribable(process, PhantomData)
	}
}

impl<T, S, C: ToString, F: Fn(T) -> CommonDescriptor<S, C>> Describable<T, S, C, CommonDescriptor<S, C>> for CommonDescribable<T,S,C,F> {
	fn describe(&self, item: T) -> CommonDescriptor<S, C> {
		self.0(item)
	}
}

#[cfg(test)]
mod test{
	use super::*;
	use crate::sample::*;
	
	#[test]
	fn reference_test(){
	    let integer=Integer::from(42);
		
		let fixture: CommonDescribable<&Integer,Envelope<&Integer>,String,_> =
		  CommonDescribable::new(|x| CommonDescriptor::new(Envelope::from(x), ["a".to_string(),"b".to_string()]));
		let d=fixture.describe(&integer);
		
		let i=d.source();
		assert_eq!(i.item().value(),42)
	}
	
	#[test]
	fn value_test(){
		let point=Point::new(42,20);
		
		
		let fixture:CommonDescribable<&Point,(i32,i32),String,_> = CommonDescribable::new(|x:&Point|{CommonDescriptor::new((x.x(),x.y()),[x.x().to_string(),x.y().to_string()])});
		
		let a=fixture.describe(&point);
		
		assert_eq!(a.source(),&(42,20))
	
	}
	
	
}