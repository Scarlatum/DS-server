use std::net::{ TcpStream, TcpListener };
use std::io::prelude::*;
use native_dialog::{ FileDialog };

mod response; use response::{ get_content };

pub struct ServerParams {
	pub port: String
}

pub struct Server {
	root: String,
	listener: TcpListener,
}

impl Server {

	pub fn new(params: ServerParams) -> Server {

		Server {
			root: String::from(""),
			listener: TcpListener::bind(format!("0.0.0.0:{}", params.port)).unwrap()
		}

	}

	pub fn init(&mut self) {

		self.root = self.get_root();

		println!(":: Await new connection at {} ::", self.listener.local_addr().unwrap());

		for stream in self.listener.incoming() {
			match stream {
				Ok(stream) 
					=> self.connection_handler(stream),
				Err(err) 
					=> println!("{}",err)
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

    stream.write(get_content(&self, resourse_path).as_bytes()).unwrap();
		stream.flush().unwrap();

	}

}