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
	use super::Query;

	mod starts {
		use super::*;
		use pretty_assertions::assert_eq;

		#[test]
		fn starts_correctly() {
			assert_eq!(
				Query::Starts("foo".to_string()).exec("foobar".to_string()),
				true
			);
		}

		#[test]
		fn starts_correctly_but_with_space() {
			assert_eq!(
				Query::Starts("foo".to_string()).exec(" foobar".to_string()),
				false
			);
		}

		#[test]
		fn starts_incorrect() {
			assert_eq!(
				Query::Starts("foo".to_string()).exec("barfoo".to_string()),
				false
			);
		}
	}

	mod ends {
		use super::*;
		use pretty_assertions::assert_eq;

		#[test]
		fn correctly() {
			assert_eq!(
				Query::Ends("baz".to_string()).exec("foobaz".to_string()),
				true
			);
		}

		#[test]
		fn correctly_but_with_space() {
			assert_eq!(
				Query::Ends("baz".to_string()).exec("baz ".to_string()),
				false
			);
		}

		#[test]
		fn incorrect() {
			assert_eq!(
				Query::Ends("baz".to_string()).exec("bazfoo".to_string()),
				false
			);
		}
	}

	mod contains {
		use super::*;
		use pretty_assertions::assert_eq;

		#[test]
		fn at_start() {
			assert_eq!(
				Query::Contains("baz".to_string()).exec("bazfoo".to_string()),
				true
			);
		}

		#[test]
		fn at_start_with_space() {
			assert_eq!(
				Query::Contains("baz".to_string()).exec(" bazfoo".to_string()),
				true
			);
		}

		#[test]
		fn at_start_with_one_char_infront() {
			assert_eq!(
				Query::Contains("baz".to_string()).exec("Xbazfoo".to_string()),
				true
			);
		}

		#[test]
		fn somewhere_in_string() {
			assert_eq!(
				Query::Contains("baz".to_string()).exec("ewfnorbaz2dewf1!".to_string()),
				true
			);
		}

		#[test]
		fn at_end() {
			assert_eq!(
				Query::Contains("baz".to_string()).exec("foobaz".to_string()),
				true
			);
		}

		#[test]
		fn at_end_with_space() {
			assert_eq!(
				Query::Contains("baz".to_string()).exec("bazfoo ".to_string()),
				true
			);
		}

		#[test]
		fn at_end_with_one_char_behind() {
			assert_eq!(
				Query::Contains("baz".to_string()).exec("foobazX".to_string()),
				true
			);
		}

		fn does_not_contain() {
			assert_eq!(
				Query::Contains("baz".to_string()).exec("foobar".to_string()),
				false
			);
		}
	}
}
