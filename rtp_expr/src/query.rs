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

impl Query {

	pub fn into_keyword(&self) -> &str {
		match self {
			Self::Starts(_) => "starts",
			Self::Ends(_) => "ends",
			Self::Contains(_) => "contains",
			Self::Equals(_) => "equals",
			Self::Length(_) => "length",
			Self::Numeric => "numeric",
			Self::Alpha => "alpha",
			Self::Alphanumeric => "alphanumeric",
			Self::Special => "special"
		}
	}
}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
