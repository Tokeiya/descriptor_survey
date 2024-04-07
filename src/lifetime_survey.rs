use std::marker::PhantomData;

pub struct Envelope<'a,T>(
	T,
	PhantomData<&'a()>
);

impl<'a,T:'a> Envelope<'a,T> {
	pub fn new(item: T) -> Self {
		Envelope(item, PhantomData)
	}
}




#[cfg(test)]
mod test{
	use super::*;

	struct Integer(i32);

	#[test]
	fn ref_test(){
	}
}