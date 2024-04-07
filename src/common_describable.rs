use std::marker::PhantomData;
use crate::common_descriptor::CommonDescriptor;
use crate::describable::Describable;

pub struct CommonDescribable<const N:usize,T,S,D,F>(F,[String;N],PhantomData<(T,S,D)>);

impl<const N:usize,T,S,D:ToString,F:Fn(T)-> CommonDescriptor<N,S,D>> CommonDescribable<N,T,S,D,F> {
	pub fn new(process:F,header:[String;N])->Self{
		CommonDescribable(process,header, PhantomData)
	}
}

impl<const N:usize,T, S, C: ToString, F: Fn(T) -> CommonDescriptor<N,S, C>> Describable<N> for CommonDescribable<N,T,S,C,F> {
	type Item = T;
	type Descriptor = CommonDescriptor<N,S,C>;
	
	fn header(&self) -> &[String; N] {
		&self.1
	}
	
	fn describe(&self, item: T) -> CommonDescriptor<N,S, C> {
		self.0(item)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::sample::*;
	
	#[test]
	fn reference_test() {
		let integer = Integer::from(42);
		
		let fixture: CommonDescribable<2,&Integer, Envelope<&Integer>, String, _>
		  = CommonDescribable::new(|x| CommonDescriptor::new(Envelope::from(x), ["a".to_string(), "b".to_string()]),["X".to_string(),"Y".to_string()]);
		
		let d = fixture.describe(&integer);
		
		let i = d.source();
		assert_eq!(i.item().value(), 42)
	}
	
	#[test]
	fn value_test() {
		let point = Point::new(42, 20);
		
		let fixture: CommonDescribable<2,&Point, (i32, i32), String, _>
		  = CommonDescribable::new(|x: &Point| { CommonDescriptor::new((x.x(), x.y()), [x.x().to_string(), x.y().to_string()]) },["X".to_string(),"Y".to_string()]);
		
		let a = fixture.describe(&point);
		
		assert_eq!(a.source(), &(42, 20))
	}
}
