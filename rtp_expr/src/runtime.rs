use crate::parser::AST;
use crate::logical_operator::LogicalOperator;


struct Runtime {
	ast: AST
}

impl Runtime {

	pub fn new(ast: AST) -> Self {
		Self {
			ast
		}
	}

	pub fn run(&self, input: String) -> bool {
		match &self.ast {
			AST::Query(query) => query.exec(input.clone()),
			AST::BinaryExpression { left, operator, right } => match operator {
				LogicalOperator::And => run((**left).clone(), input.clone()) && run((**right).clone(), input.clone()),
				LogicalOperator::Or => run((**left).clone(), input.clone()) || run((**right).clone(), input.clone())
			}
		}
	}

}

pub fn run(ast: AST, input: String) -> bool {
	let rt = Runtime::new(ast);

	rt.run(input)
}


#[cfg(test)]
mod tests {
	use super::run;
	use crate::into_ast;
	use crate::logical_operator::LogicalOperator;
	use crate::query::Query;

	macro_rules! runtime_test {
		($($name:ident: $value:expr,)*) => {
			$(
				#[test]
				fn $name() {
					let (query_source, test_string, result) = $value;
					pretty_assertions::assert_eq!(run(into_ast(query_source.to_string()).unwrap(), test_string.to_string()), result);
				}
			)*
		}
	}

	mod it_handles_single_and_expressions {
		use super::*;

		runtime_test! {
			numeric_and_length: (
				"numeric and length 9",
				"123456789",
				true
			),
			numeric_and_length_with_alphanumeric_input: (
				"numeric and length 9",
				"123456ABC",
				false
			),
			numeric_and_alpha: (
				"numeric and alpha",
				"123456789",
				false
			),
			numeric_and_alpha_with_alphanumeric_input: (
				"numeric and numeric",
				"123ABC",
				false
			),
			numeric_and_alphanumeric_with_numeric_input: (
				"numeric and alphanumeric",
				"123456789",
				true
			),
			numeric_and_alphanumeric_with_alphanumeric_input: (
				"numeric and alphanumeric",
				"123ABC",
				false
			),
			numeric_and_special: (
				"numeric and special",
				"123",
				false
			),
		}
	}

	mod it_handles_single_or_expressions {
		use super::*;

		runtime_test! {
			numeric_or_length: (
				"numeric or length 9",
				"123456789",
				true
			),
			numeric_or_wrong_length: (
				"numeric or length 99",
				"123456789",
				true
			),
			numeric_or_alpha: (
				"numeric or alpha",
				"123456789",
				true
			),
			numeric_or_alpha_with_alphanumeric_input: (
				"numeric or alpha",
				"123ABC",
				false
			),
			numeric_or_alphanumeric_with_numeric_input: (
				"numeric or alphanumeric",
				"123456789",
				true
			),
			numeric_or_alphanumeric_with_alphanumeric_input: (
				"numeric or alphanumeric",
				"123ABC",
				true
			),
			numeric_or_special: (
				"numeric or special",
				"123456789",
				true
			),
		}
	}

	mod it_handles_nested_expressions_correctly {
		use super::*;

		runtime_test! {
			starts_and_ends_or_length_with_correct_length: (
				"starts \"foo\" and ends \"bar\" or length 9",
				"xyzxyzxyz",
				true
			),
			starts_and_ends_or_length_with_correct_start_wrong_end: (
				"starts \"foo\" and ends \"bar\" or length 9",
				"foobaz",
				false
			),
			starts_and_ends_or_length_with_correct_start_and_end: (
				"starts \"foo\" and ends \"bar\" or length 9",
				"foobar",
				true
			),
			starts_and_ends_or_length_with_correct_start_and_end_and_length: (
				"starts \"foo\" and ends \"bar\" or length 9",
				"foobazbar",
				true
			),
			starts_and_ends_or_length_with_correct_start_and_end_and_wrong_length: (
				"starts \"foo\" and ends \"bar\" or length 9",
				"foobaaaaaaaaazbar",
				true
			),
			starts_and_ends_or_equals_or_length_with_completly_wrong_input: (
				"starts \"foo\" and ends \"bar\" or equals \"keyword\" or length 4",
				"completly wrong input :)",
				false
			),
			starts_and_ends_or_equals_or_length_with_correct_start_wrong_end: (
				"starts \"foo\" and ends \"bar\" or equals \"keyword\" or length 4",
				"foo but wrong end",
				false
			),
			starts_and_ends_or_equals_or_length_with_wrong_start_correct_end: (
				"starts \"foo\" and ends \"bar\" or equals \"keyword\" or length 4",
				"wrong start but bar",
				false
			),
			starts_and_ends_or_equals_or_length_with_correct_start_correct_end: (
				"starts \"foo\" and ends \"bar\" or equals \"keyword\" or length 4",
				"foo weird middle part bar",
				true
			),
			starts_and_ends_or_equals_or_length_with_equals: (
				"starts \"foo\" and ends \"bar\" or equals \"keyword\" or length 4",
				"keyword",
				true
			),
			starts_and_ends_or_equals_or_length_with_slightly_wrong_equals: (
				"starts \"foo\" and ends \"bar\" or equals \"keyword\" or length 4",
				"keywords",
				false
			),
			starts_and_ends_or_equals_or_length_with_wrong_length: (
				"starts \"foo\" and ends \"bar\" or equals \"keyword\" or length 4",
				"UWXYZ",
				false
			),
			starts_and_ends_or_equals_or_length_with_correct_length: (
				"starts \"foo\" and ends \"bar\" or equals \"keyword\" or length 4",
				"WXYZ",
				true
			),
		}
	}
}
