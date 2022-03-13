use std::io::{Read, BufRead, Write};
use std::os::unix::net::UnixStream;

fn main() {
	let mut stream = UnixStream::connect("./ntim.socket").unwrap();
	let stdin = std::io::stdin();
	let mut line_iter = stdin.lock().lines();
	let mut text = Vec::new();
	loop {
		println!("> Current text: {}", text.join("|"));
		let line = line_iter.next().unwrap().unwrap();
		if !line.is_ascii() { continue }
		let ch1 = line.chars().next();
		match ch1 {
			Some(',') => {
				text.pop();
				continue
			},
			Some('.') => {
				std::fs::write("/tmp/ntim_rs.txt", text.join("").as_bytes()).unwrap();
				break
			}
			None => {
				continue
			}
			_ => {},
		}
		stream.write_all(line.as_bytes()).unwrap();
		let mut buf = [0u8; 32768];
		let words = if let Ok(buflen) = stream.read(&mut buf) {
			let string = String::from_utf8(buf[1..buflen].to_vec()).unwrap();
			let mut idx = 0;
			let mut words: Vec<String> = Vec::new();
			for word in string.split(' ') {
				if word.trim().is_empty() { continue }
				if idx > 0 && idx % 10 == 0 { println!() }
				print!("{}:{} ", idx, word);
				words.push(word.to_string());
				idx += 1;
			}
			println!();
			words
		} else {
			break
		};
		if words.is_empty() { continue }

		let line = line_iter.next().unwrap().unwrap();
		let idx = if line.is_empty() {
			0
		} else if let Ok(idx) = line.parse::<usize>() {
			idx
		} else {
			continue
		};
		if let Some(w) = words.get(idx) {
			text.push(w.clone());
		}
	}
}
