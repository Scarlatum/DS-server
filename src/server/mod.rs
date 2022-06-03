use std::net::{ TcpStream, TcpListener };
use std::io::prelude::*;
use native_dialog::{ FileDialog };

mod response; 
mod resources;

use crate::watcher::Watcher;
use resources::FileLoader;

const DEFAULT_HOST: &str = "0.0.0.0";

pub struct ServerParams {
	pub port: String
}

pub struct Server {
	root			: String,
	tcp				: TcpListener,
	watcher		: Option<Watcher>,
	resources	: Option<FileLoader>,
}

impl Server {

	pub fn new(params: ServerParams) -> Server {

		Server {
			root			: String::from(""),
			tcp				: TcpListener::bind(format!("{}:{}", DEFAULT_HOST, params.port)).unwrap(),
			watcher		: None,
			resources : None,
		}

	}

	pub fn init(&mut self) {

		self.root 			= self.get_root();
		self.watcher 		= Some(Watcher::new(self.root.clone()));
		self.resources 	= Some(FileLoader::new());

		println!("\n:: Collect resources ::");

		if let Some(res) = self.resources.as_mut() {
			res.collect(&self.root).unwrap();
		}

		// println!("{:#?}", self.resources);

		println!("\n:: Init fs watcher ::");

		if let Some(instance) = self.watcher.as_mut() {
			instance.init()
		}

		println!("\n:: Await new connection at {} ::", self.tcp.local_addr().unwrap());

		for stream in self.tcp.incoming() {
			match stream {
				Ok(stream) => {
					self.connection_handler(stream)
				},
				Err(e) => println!("{}", e),
			}
		}

	}

	fn get_root(&self) -> String {

		match FileDialog::new()
			.set_location("~/")
			.show_open_single_dir()
			.unwrap() {
			Some(path) 
				=> String::from(path.to_string_lossy()),
			None 
				=> String::from("./")
		} 

	}

	fn connection_handler(&self, mut stream: TcpStream) {

		use response::{ get_content };

		let mut buffer: [u8; 1024] = [0; 1024];

		stream.read(&mut buffer).unwrap();

		let mut resourse_path = String::new();

		for (i, line) in String::from_utf8_lossy(&buffer[..]).lines().enumerate() {
			match i {
				0 => {
					for path in String::from(line).split(" ") {

						if String::from(path).starts_with("/") {
							resourse_path = match path {
								"/" 
								=> String::from("/index.html"),
								_ 
								=> String::from(path)
							}
						}

					}
				},
				_ => (),
			}
		};

    stream.write(&get_content(self, resourse_path)).unwrap();
		stream.flush().unwrap();

	}

}