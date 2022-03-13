use std::io::{prelude::*, BufReader};

use std::collections::{BTreeMap, HashMap};

type Dict = HashMap<String, BTreeMap<String, f32>>;

#[derive(Default)]
pub struct NtimDict {
	data: Dict,
}

impl NtimDict {
	pub fn load_txt(filename: &str) -> Self {
		let file = std::fs::File::open(filename).unwrap();
		let reader = BufReader::new(file);
		let mut data: Dict = HashMap::new();
		for line in reader.lines() {
			let line = line.unwrap();
			let words: Vec<&str> = line.split(' ').collect();
			match words.len() {
				2 => {
					let e = data.entry(words[0].to_string()).or_insert_with(Default::default);
					e.insert(words[1].to_string(), 0.0);
				},
				3 => {
					let e = data.entry(words[0].to_string()).or_insert_with(Default::default);
					e.insert(words[1].to_string(), words[2].parse::<f32>().unwrap());
				}
				_ => {
					eprintln!("WARN: word len error: {:?}", words);
					continue
				}
			}
		}
		Self { data }
	}

	pub fn query(&self, key: &str) -> Vec<String> {
		match self.data.get(key) {
			Some(map) => map.keys().rev().cloned().collect(),
			None => Vec::new(),
		}
	}
}
