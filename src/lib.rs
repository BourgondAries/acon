use std::collections::BTreeMap;
use std::str::Chars;

/// Interface ////////////////////////////////////

pub type Array = Vec<Value>;
pub type Table = BTreeMap<String, Value>;

pub enum Value {
	Array(Array),
	Boolean(bool),
	Float(f64),
	Integer(i64),
	String(String),
	Table(Table),
}

pub fn parse(input: &str) -> Result<Table, String> {
	Parser::new(input);
	Ok(Table::new())
}

/// Parser //////////////////////////////////////

struct Parser<'a> {
	current: usize,
	input: &str,
}

impl<'a> Parser<'a> {
	fn new(input: &'a str) -> Parser<'a> {
		Parser {
			current: 0usize,
			input: input,
		}
	}

	fn find_key(&mut self) -> (usize, usize) {
		while self.current < self.input.len() {
			if self.input[self.current].is_whitespace() {
				self.current += 1;
			} else {
				break;
			}
		}

		let begin = self.current;

		while self.current < self.input.len() {
			if !self.input[self.current].is_whitespace() {
				self.current += 1;
			} else {
				break;
			}
		}

		let end = self.current;
		(begin, end)
	}

	fn find_non_whitespace(&mut self) -> Option<char> {
		self.iterator.peek();
		while let Some(x) = self.iterator.next() {
			if x.is_whitespace() {
				continue;
			} else {
				return Some(x);
			}
		}
		None
	}

	fn parse(&mut self) {
		/*consume_whitespace();
		loop {
			capture_word();
			capture_variable();
		}
		*/
	}

	fn get_key(&mut self) {
	}
}

/// Tests ////////////////////////////////////////

#[test]
fn test_whitespace() {
	let mut parser = Parser::new("Test");
	assert!(parser.find_non_whitespace() == Some('T'));
}
