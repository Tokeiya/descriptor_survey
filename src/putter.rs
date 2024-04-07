use super::sample::*;
use std::marker::PhantomData;

pub struct Putter<T,U,F>(F,PhantomData<(T,U)>);


impl<T,U,F:Fn(T)->Envelope<U>> Putter<T,U,F>{
	pub fn new(process:F)->Self{
		Putter(process,PhantomData)
	}
	
	pub fn put_in(&self,item:T)->Envelope<U>{
		self.0(item)
	}
}

#[cfg(test)]
mod tests{
	use std::ops::Deref;
	use super::*;
	
	#[test]
	fn ref_test(){
	    let a=Integer::from(200);
		
		let putter=Putter::<&Integer,&Integer,_>::new(|x|Envelope::from(x));
		let b=putter.put_in(&a);
		
		
		let a=b.item().deref().value();
		assert_eq!(a,200);
	}
	
}