use std::marker::PhantomData;

pub struct Descriptor<'a, S, D> {
	source: S,
	description: [D; 2],
	_phantom: PhantomData<&'a ()>,
}

impl<'a, S, D: ToString> Descriptor<'a, S, D> {
	pub fn new(source: S, description: [D; 2]) -> Self {
		Descriptor {
			source,
			description,
			_phantom: PhantomData,
		}
	}
	
	pub fn source(&self) -> &S {
		&self.source
	}
	
	pub fn description(&self) -> &[D; 2] {
		&self.description
	}
}