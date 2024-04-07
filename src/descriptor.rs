use std::marker::PhantomData;

pub trait  Descriptor<S,D:ToString>{
	fn source(&self)->&S;
	fn description(&self)->&[D;2];
}

pub struct CommonDescriptor<S, D> {
	source: S,
	description: [D; 2],
}

impl<S, D: ToString> CommonDescriptor<S, D> {
	pub fn new(source: S, description: [D; 2]) -> Self {
		CommonDescriptor {
			source,
			description,
		}
	}

	pub fn source(&self) -> &S {
		&self.source
	}

	pub fn description(&self) -> &[D; 2] {
		&self.description
	}
}

impl<S,D:ToString> Descriptor<S,D> for CommonDescriptor<S,D> {
	fn source(&self) -> &S {
		&self.source
	}
	
	fn description(&self) -> &[D; 2] {
		&self.description
	}
}