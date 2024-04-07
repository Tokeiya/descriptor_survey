use super::descriptor::Descriptor;

pub struct CommonDescriptor<const N:usize,S, D> {
	source: S,
	description: [D; N],
}

impl<const N:usize,S, D: ToString> CommonDescriptor<N,S, D> {
	pub fn new(source: S, description: [D; N]) -> Self {
		CommonDescriptor {
			source,
			description,
		}
	}
	
	pub fn source(&self) -> &S {
		&self.source
	}
	
	pub fn description(&self) -> &[D; N] {
		&self.description
	}
}

impl<const N:usize,S,D:ToString> Descriptor<N> for CommonDescriptor<N,S,D> {
	type Source = S;
	type Description = D;
	
	fn source(&self) -> &Self::Source {
		&self.source
	}
	
	fn description(&self) -> &[Self::Description; N] {
		&self.description
	}
}