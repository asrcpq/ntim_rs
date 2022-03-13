use crate::dict::NtimDict;

use std::io::{Read, Write};
use std::os::unix::net::{UnixStream, UnixListener};

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
					while let Ok(buflen) = stream.read(&mut buf) {
						if buflen == 0 { break }
						let key = String::from_utf8(buf[..buflen].to_vec()).unwrap();
						let key = key.trim();
						eprintln!("{:?}", key);
						let output = self.dict.query(key).join(" ");
						stream.write_all(output.as_bytes()).unwrap();
					}
				}
				Err(err) => { break },
			}
		}
	}
}
