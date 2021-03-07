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
		fn correct() {
			assert_eq!(
				Query::Starts("foo".to_string()).exec("foobar".to_string()),
				true
			);
		}

		#[test]
		fn correct_but_with_space() {
			assert_eq!(
				Query::Starts("foo".to_string()).exec(" foobar".to_string()),
				false
			);
		}

		#[test]
		fn incorrect() {
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
		fn correct() {
			assert_eq!(
				Query::Ends("baz".to_string()).exec("foobaz".to_string()),
				true
			);
		}

		#[test]
		fn correct_but_with_space() {
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

	mod equals {
		use super::*;
		use pretty_assertions::assert_eq;

		#[test]
		fn correct() {
			assert_eq!(
				Query::Equals("foo".to_string()).exec("foo".to_string()),
				true
			);
		}

		#[test]
		fn correct_but_with_space() {
			assert_eq!(
				Query::Equals("foo".to_string()).exec(" foo".to_string()),
				false
			);
		}

		#[test]
		fn close_to_correct() {
			assert_eq!(
				Query::Equals("foo".to_string()).exec("fooo".to_string()),
				false
			);
		}

		#[test]
		fn incorrect() {
			assert_eq!(
				Query::Equals("foo".to_string()).exec("bar".to_string()),
				false
			);
		}
	}

	mod length {
		use super::*;
		use pretty_assertions::assert_eq;

		#[test]
		fn correct() {
			assert_eq!(
				Query::Length(3).exec("foo".to_string()),
				true
			);
		}

		#[test]
		fn one_char_to_short() {
			assert_eq!(
				Query::Length(3).exec("fo".to_string()),
				false
			);
		}

		#[test]
		fn one_char_to_long() {
			assert_eq!(
				Query::Length(3).exec("fooo".to_string()),
				false
			);
		}

		#[test]
		fn completly_wrong_length() {
			assert_eq!(
				Query::Length(3).exec("foobarbaz".to_string()),
				false
			);
		}
	}

	mod numeric {
		use super::*;
		use pretty_assertions::assert_eq;

		#[test]
		fn only_digits() {
			assert_eq!(
				Query::Numeric.exec("123456789".to_string()),
				true
			);
		}

		#[test]
		fn digits_and_spaces() {
			assert_eq!(
				Query::Numeric.exec("123 213124 2".to_string()),
				false
			);
		}

		#[test]
		fn digits_and_alpha() {
			assert_eq!(
				Query::Numeric.exec("123e".to_string()),
				false
			);
		}

		#[test]
		fn digits_and_punctuation() {
			assert_eq!(
				Query::Numeric.exec("123.2".to_string()),
				false
			);
		}

		#[test]
		fn empty() {
			assert_eq!(
				Query::Numeric.exec("".to_string()),
				true
			);
		}
	}

	mod alpha {
		use super::*;
		use pretty_assertions::assert_eq;

		#[test]
		fn only_alpha() {
			assert_eq!(
				Query::Alpha.exec("abc".to_string()),
				true
			);
		}

		#[test]
		fn alpha_and_spaces() {
			assert_eq!(
				Query::Alpha.exec("abc def ghij k".to_string()),
				false
			);
		}

		#[test]
		fn alpha_and_digits() {
			assert_eq!(
				Query::Alpha.exec("ABC1".to_string()),
				false
			);
		}

		#[test]
		fn alpha_and_punctuation() {
			assert_eq!(
				Query::Alpha.exec("abc.com".to_string()),
				false
			);
		}

		#[test]
		fn empty() {
			assert_eq!(
				Query::Alpha.exec("".to_string()),
				true
			);
		}
	}

}
