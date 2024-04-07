pub trait  Descriptor<const N:usize>{
	type Source;
	type Description:ToString;
	
	fn source(&self)->&Self::Source;
	fn description(&self)->&[Self::Description;N];
}