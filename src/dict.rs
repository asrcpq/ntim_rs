use std::io::{self, prelude::*, BufReader};

use std::collections::HashMap;

#[derive(Default)]
pub struct NtimDict {
	data: HashMap<String, Vec<String>>,
}

impl NtimDict {
	pub fn load_txt(filename: &str) -> Self {
		let file = std::fs::File::open(filename).unwrap();
		let reader = BufReader::new(file);
		let mut data = HashMap::new();
		for line in reader.lines() {
			let line = line.unwrap();
			let words: Vec<&str> = line.split(' ').collect();
			if words.len() != 2 {
				eprintln!("WARN: word len error: {:?}", words);
				continue
			}
			let e = data.entry(words[0].to_string()).or_insert_with(Vec::new);
			e.push(words[1].to_string());
		}
		Self { data }
	}

	pub fn query(&self, key: &str) -> Vec<String> {
		self.data.get(key).unwrap_or(&Vec::new()).clone()
	}
}
