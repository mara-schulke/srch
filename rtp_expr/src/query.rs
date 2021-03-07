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

	pub fn exec(&self, tested_string: String) -> bool {
		match self {
			Self::Starts(arg) => tested_string.starts_with(arg),
			Self::Ends(arg) => tested_string.ends_with(arg),
			Self::Contains(arg) => tested_string.contains(arg),
			Self::Equals(arg) => &tested_string == arg,
			Self::Length(len) => tested_string.len() == *len as usize,
			Self::Numeric => tested_string.chars().all(|c| c.is_ascii_digit()),
			Self::Alpha => tested_string.chars().all(|c| c.is_ascii_alphabetic()),
			Self::Alphanumeric => tested_string.chars().all(|c| c.is_ascii_alphanumeric()),
			Self::Special => tested_string.chars().all(|c| c.is_ascii_punctuation())
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
