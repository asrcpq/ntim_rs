use std::io::{Read, BufRead, Write};
use std::os::unix::net::UnixStream;

fn main() {
	let mut stream = UnixStream::connect("./ntim.socket").unwrap();
	let stdin = std::io::stdin();
	let mut line_iter = stdin.lock().lines();
	let mut text = Vec::new();
	loop {
		let line = line_iter.next().unwrap().unwrap();
		stream.write_all(line.as_bytes()).unwrap();
		let mut buf = [0u8; 32768];
		let words = if let Ok(buflen) = stream.read(&mut buf) {
			let string = String::from_utf8(buf[..buflen].to_vec()).unwrap();
			let words: Vec<String> = string.split(' ').map(|x| x.to_string()).collect();
			for (idx, word) in words.iter().enumerate() {
				if idx > 0 && idx % 10 == 0 { println!() }
				print!("{}:{} ", idx, word);
			}
			println!();
			words
		} else {
			break
		};

		let line = line_iter.next().unwrap().unwrap();
		if let Ok(idx) = line.parse::<usize>() {
			if let Some(w) = words.get(idx) {
				text.push(w.clone());
			}
		}
		println!("{}", text.join(""));
	}
}
