use std::io::{prelude::*, BufReader};

use std::collections::HashMap;

type Dict = HashMap<String, HashMap<String, f32>>;

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
					let e = e.entry(words[1].to_string()).or_insert(0.0);
					*e += 1.0;
				},
				3 => {
					let e = data.entry(words[0].to_string()).or_insert_with(Default::default);
					let e = e.entry(words[1].to_string()).or_insert(0.0);
					*e += words[2].parse::<f32>().unwrap();
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
			Some(map) => {
				let mut v = Vec::from_iter(map);
				v.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());
				v.iter().map(|x| x.0).cloned().collect()
			}
			None => Vec::new(),
		}
	}
}
