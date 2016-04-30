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
	match env_logger::init() {
		Ok(_) => {}
		Err(_) => {
			println!("Env logger was already initialized");
		}
	}
	let mut string = String::new();
	let stdin = std::io::stdin();
	let mut stdin = stdin.lock();
	match stdin.read_to_string(&mut string) {
		Ok(bytes) => trace!("Read {} bytes", bytes),
		Err(err) => {
			error!("Unable to read from standard input, aborting, {}", err);
			panic!();
		}
	}
	let map: Value = match serde_json::from_str(&string) {
		Ok(map) => map,
		Err(err) => {
			error!("{:?}", err);
			panic!();
		}
	};
	print_value(&map, 0);
}


fn tabs(level: usize) -> String {
	let mut string = String::new();
	for _ in 0..level {
		string.push('\t');
	}
	string
}

fn escape(string: &str) -> String {
	string
		.replace("\n", "\\n")
		.replace("\t", "\\t")
}

fn print_value(value: &Value, level: usize) {
	match *value {
		Value::Null => {
			println!("{}{}", tabs(level), "null");
		}
		Value::Bool(value) => {
			println!("{}{}", tabs(level), value);
		}
		Value::I64(value) => {
			println!("{}{}", tabs(level), value);
		}
		Value::U64(value) => {
			println!("{}{}", tabs(level), value);
		}
		Value::F64(value) => {
			println!("{}{}", tabs(level), value);
		}
		Value::String(ref string) => {
			println!("{}{}", tabs(level), escape(string));
		}
		Value::Array(ref array) => {
			println!("{}{}", tabs(level), '[');
			for i in array {
				print_value(i, level + 1);
			}
			println!("{}{}", tabs(level), ']');
		}
		Value::Object(ref submap) => {
			println!("{}{}", tabs(level), "{");
			print(submap, level + 1);
			println!("{}{}", tabs(level), "}");
		}
	}
}

fn print(map: &BTreeMap<String, Value>, level: usize) {
	for (key, value) in map {
		match *value {
			Value::Null => { println!("{}{} {}", tabs(level), key, "null"); }
			Value::Bool(value) => {
				println!("{}{} {}", tabs(level), key, if value { "true" } else { "false" });
			}
			Value::I64(value) => {
				println!("{}{} {}", tabs(level), key, value);
			}
			Value::U64(value) => {
				println!("{}{} {}", tabs(level), key, value);
			}
			Value::F64(value) => {
				println!("{}{} {}", tabs(level), key, value);
			}
			Value::String(ref string) => {
				println!("{}{} {}", tabs(level), key, escape(string));
			}
			Value::Array(ref array) => {
				println!("{}{} {}", tabs(level), '[', key);
				for i in array {
					print_value(i, level + 1);
				}
				println!("{}{}", tabs(level), ']');
			}
			Value::Object(ref submap) => {
				println!("{}{} {}", tabs(level), "{", key);
				print(submap, level + 1);
				println!("{}{}", tabs(level), "}");
			}
		}
	}
}

