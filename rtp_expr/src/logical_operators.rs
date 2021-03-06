#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
	And,
	Or
}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
