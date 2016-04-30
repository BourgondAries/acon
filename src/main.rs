#![feature(plugin)]
#![plugin(clippy)]
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde_json;

use std::collections::BTreeMap;
use std::io::Read;
use serde_json::value::Value;

fn main() {
	let mut string = String::new();
	let stdin = std::io::stdin();
	let mut stdin = stdin.lock();
	match stdin.read_to_string(&mut string) {
		Ok(_) => trace!("Read input"),
		Err(err) => {
			error!("Unable to read from standard input, aborting, {}", err);
			panic!();
		}
	}
	let map: BTreeMap<String, Value> = match serde_json::from_str(&string) {
		Ok(map) => map,
		Err(err) => {
			error!("{:?}", err);
			panic!();
		}
	};

	fn tabs(level: usize) -> String {
		let mut string = String::new();
		for _ in 0..level {
			string.push('\t');
		}
		string
	}

	fn printSimple(value: &Value, level: usize) {
		match value {
			&Value::String(ref string) => {
				println!("{}{}", tabs(level), string);
			}
			_ => {}
		}
	}
	fn print(map: &BTreeMap<String, Value>, level: usize) {
		for (key, value) in map {
			match value {
				&Value::Null => {
					println!("{}{} {}", tabs(level), key, "null");
				}
				&Value::Bool(value) => {
					println!("{}{} {}", tabs(level), key, value);
				}
				&Value::I64(value) => {
					println!("{}{} {}", tabs(level), key, value);
				}
				&Value::U64(value) => {
					println!("{}{} {}", tabs(level), key, value);
				}
				&Value::F64(value) => {
					println!("{}{} {}", tabs(level), key, value);
				}
				&Value::String(ref string) => {
					println!("{}{} {}", tabs(level), key, string);
				}
				&Value::Array(ref array) => {
					println!("{}{} {}", tabs(level), '[', key);
					for i in array {
						printSimple(i, level + 1);
					}
					println!("{}{}", tabs(level), ']');
				}
				&Value::Object(ref submap) => {
					println!("{}{} {}", tabs(level), "{", key);
					print(submap, level + 1);
					println!("{}{}", tabs(level), "}");
				}
			}
		}
	}
	print(&map, 0);
}
