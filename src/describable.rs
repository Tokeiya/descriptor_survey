use super::descriptor::Descriptor;

pub trait Describable<const N:usize>{
	type Item;
	type Descriptor:Descriptor<N>;
	
	fn header(&self)->&[String;N];
	fn describe(&self,item:Self::Item)->Self::Descriptor;
}