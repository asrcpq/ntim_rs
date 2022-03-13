use ntim_rs::server::NtimServer;

fn main() {
	let mut iter = std::env::args();
	iter.next();
	let path = iter.next().unwrap();
	let mut server = NtimServer::new(&path);
	server.run();
}
