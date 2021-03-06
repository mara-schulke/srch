#[derive(Clone, Debug, PartialEq)]
pub enum Query {
	Starts(String),
	Ends(String),
	Contains(String),
	Equals(String),
	Length(u64),
	Numeric,
	Alpha,
	Alphanumeric,
	Special
}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
