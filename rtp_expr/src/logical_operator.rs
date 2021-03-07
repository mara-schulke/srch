#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogicalOperator {
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
