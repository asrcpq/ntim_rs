use anyhow::Result;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<()> {
	let mut stream = UnixStream::connect("./ntim.socket")?;
	let mut text: Vec<char> = Vec::new();
	let mut buffer: Vec<u8> = Vec::new();
	let mut words: Vec<String> = Vec::new();
	let stdin = std::io::stdin();
	let stdin = stdin.lock();
	let stdout = std::io::stdout();
	let mut stdout = stdout.lock().into_raw_mode()?;
	write!(stdout, "{}{}", termion::clear::All, termion::style::Reset)?;
	stdout.flush().unwrap();
	let mut page_offset: usize = 0;
	for key in stdin.keys() {
		let (_col, row) = termion::terminal_size().unwrap();
		write!(stdout, "{}{}", termion::clear::All, termion::style::Reset)?;
		match key {
			Ok(Key::Ctrl('b')) => {
				if page_offset >= 10 {
					page_offset -= 10;
				} else {
					page_offset = 0;
				}
			}
			Ok(Key::Ctrl('f')) => {
				page_offset += 10;
				if page_offset >= words.len() {
					page_offset = words.len() - 1;
				}
			}
			_ => {}
		}
		let mut send_msg = false;
		match key {
			Ok(Key::Char(ch)) => {
				if ch == '\n' {
					if buffer.is_empty() {
						break
					}
					text.extend(String::from_utf8(std::mem::take(&mut buffer)).unwrap().chars());
				} else if ch.is_ascii_alphabetic() || ch.is_ascii_punctuation() {
					buffer.push(ch as u8);
					send_msg = true;
				} else if (ch.is_ascii_digit() || ch == ' ') && buffer.is_empty() {
					text.push(ch as char);
				} else if ch.is_ascii_digit() || ch == ' ' {
					let num = if ch == ' ' {
						1
					} else {
						(ch as u8 - b'0') as usize
					};
					let num = if num == 0 { 9 } else { num - 1 };
					if let Some(word) = words.get(num + page_offset) {
						text.extend(word.chars());
						buffer.clear();
						words.clear();
					}
				}
			}
			Ok(Key::Ctrl('d')) => {
				break;
			}
			Ok(Key::Ctrl('c')) => {
				buffer.clear();
				words.clear();
			}
			Ok(Key::Backspace) => {
				if buffer.is_empty() {
					text.pop();
				} else {
					buffer.pop();
					if !buffer.is_empty() {
						send_msg = true;
					} else {
						words.clear();
					}
				}
			}
			Err(_) => continue,
			_ => {}
		}
		if send_msg {
			stream.write_all(&buffer)?;
			let mut buf = [0u8; 32768];
			words = if let Ok(buflen) = stream.read(&mut buf) {
				let string = String::from_utf8(buf[1..buflen].to_vec())?;
				let mut words: Vec<String> = Vec::new();
				for word in string.split(' ') {
					words.push(word.to_string());
				}
				words
			} else {
				break;
			};
			page_offset = 0;
		}
		write!(
			stdout,
			"{}{}",
			Goto(1, row - 1),
			String::from_utf8(buffer.clone()).unwrap()
		)?;
		write!(stdout, "{}", Goto(1, row))?;
		for di in 0..10 {
			if let Some(word) = words.get(page_offset + di) {
				write!(stdout, "{}:{} ", di + 1, word)?;
			}
		}
		write!(
			stdout,
			"{}> {}",
			Goto(1, 1),
			text.iter().collect::<String>()
		)?;
		stdout.flush().unwrap();
	}
	std::fs::write(
		"/tmp/ntim_rs.txt",
		text.into_iter().collect::<String>(),
	)?;
	Ok(())
}
