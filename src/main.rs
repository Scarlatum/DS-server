// Server Module
mod server; 
use server::{ Server, ServerParams };

// Modules;
mod watcher;
mod utils;

const DEFAULT_PORT: &str = "3000";
fn main() {

	println!(":: Eccheuma static server | Proto:000.1 ::");

	let params = ServerParams { port: String::from(DEFAULT_PORT) };
	let mut server = Server::new(params);
	
	server.init();

}





