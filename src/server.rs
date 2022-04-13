use crate::dict::NtimDict;

use std::io::{Read, Write};
use std::os::unix::net::UnixListener;

pub struct NtimServer {
	dict: NtimDict,
}

impl NtimServer {
	pub fn new(dict_path: &str) -> Self {
		Self {
			dict: NtimDict::load_txt(dict_path),
		}
	}

	pub fn run(&mut self) {
		let _ = std::fs::remove_file("./ntim.socket");
		let listener = UnixListener::bind("./ntim.socket").unwrap();
		let mut buf = [0u8; 1024];
		for stream in listener.incoming() {
			match stream {
				Ok(mut stream) => {
					'read_loop: while let Ok(buflen) = stream.read(&mut buf) {
						if buflen == 0 {
							break;
						}
						let mut key = Vec::new();
						for byte in buf.iter().take(buflen) {
							if !(32..127).contains(byte) {
								continue 'read_loop;
							}
							key.push(*byte as char);
						}
						let key: String = key.into_iter().collect();
						let key = key.trim();
						eprintln!("{:?}", key);
						let output = self.dict.query(key).join(" ");
						let output = format!("x{}", output);
						stream.write_all(output.as_bytes()).unwrap();
					}
				}
				Err(_) => break,
			}
		}
	}
}
