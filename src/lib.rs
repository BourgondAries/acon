use std::collections::BTreeMap;
use std::str::CharIndices;

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
	curchar: Option<char>,
	current: usize,
	input: &'a str,
	iterator: CharIndices<'a>,
}

impl<'a> Parser<'a> {
	fn new(input: &'a str) -> Parser<'a> {
		Parser {
			curchar: None,
			current: 0usize,
			input: input,
			iterator: input.char_indices(),
		}
	}

	fn get_current_char(&self) -> Option<char> {
		self.curchar
	}

	fn consume_until_non_whitespace(&mut self) -> usize {
		if let Some(ch) = self.get_current_char() {
			if !ch.is_whitespace() {
				return self.current;
			}
		}
		while self.current < self.input.len() {
			if let Some((o, ch)) = self.iterator.next() {
				self.current = o;
				self.curchar = Some(ch);
				if !ch.is_whitespace() {
					break;
				}
			} else {
				break;
			}
		}
		self.current
	}

	fn consume_until_whitespace(&mut self) -> usize {
		if let Some(ch) = self.get_current_char() {
			if ch.is_whitespace() {
				return self.current;
			}
		}
		while self.current < self.input.len() {
			if let Some((o, ch)) = self.iterator.next() {
				self.current = o;
				self.curchar = Some(ch);
				if ch.is_whitespace() {
					break;
				}
			} else {
				break;
			}
		}
		self.current
	}

	fn find_key(&mut self) -> (usize, usize) {
		let begin = self.consume_until_non_whitespace();
		let end = self.consume_until_whitespace();
		(begin, end)
	}

	fn parse_value(&mut self) -> Value {
		let begin = self.consume_until_non_whitespace();
		if let Some(ch) = self.curchar {
			match ch {
				'\'' => {
					println!("FOUND A SINGULAR QUOTE");
				}
				'{' => {
					println!("FOUND SUBTABLE");
				}
				_ => {}
			}
		}
		Value::Boolean(false)
	}

	fn parse_literal(&mut self) -> String {

		"".into()
	}

}

/// Tests ////////////////////////////////////////

#[test]
fn test_value() {
	let mut parser = Parser::new("key 'value'");
	parser.find_key();
	parser.parse_value();
	assert!(false);
}

#[test]
fn test_whitespace() {
	let mut parser = Parser::new("Test d");
	for _ in 1..50 {
		assert_eq!(parser.consume_until_non_whitespace(), 0);
	}
}

#[test]
fn test_whitespace2() {
	let mut parser = Parser::new("Test derp");
	for _ in 1..50 {
		assert_eq!(parser.consume_until_whitespace(), 4);
	}
}

#[test]
fn test_key_1() {
	let mut parser = Parser::new("Test 'key'");
	assert_eq!(parser.find_key(), (0, 4));
}
