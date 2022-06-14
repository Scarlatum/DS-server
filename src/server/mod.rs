use std::net::{ TcpStream, TcpListener };
use std::io::prelude::*;

use native_dialog::{ FileDialog };

mod response; 
mod resources;
mod pages;

use crate::watcher::Watcher;
use resources::FileLoader;
use pages::PagesDict;

const DEFAULT_HOST: &str = "0.0.0.0";

pub struct ServerParams {
	pub port: String
}

#[derive(Debug)]
enum Dist {
	Relative,
	Absolute,
}

struct Redirect {
	path: String,
	dist: Dist,
}

pub struct Server {
	root					: String,
	tcp						: TcpListener,
	watcher				: Option<Watcher>,
	resources			: Option<FileLoader>,
	redirect			: Redirect,
	pages					: PagesDict,
}

impl Server {

	pub fn new(params: ServerParams) -> Server {

		Server {
			root			: String::default(),
			tcp				: TcpListener::bind(format!("{}:{}", DEFAULT_HOST, params.port)).unwrap(),
			watcher		: None,
			resources : None,
			pages			: PagesDict::new(),
			redirect  : Redirect { 
				path: String::default(), 
				dist: Dist::Absolute 
			}
		}

	}

	pub fn init(&mut self) -> () {

		self.root 			= self.get_root().unwrap();
		self.watcher 		= Some(Watcher::new(self.root.clone()));
		self.resources 	= Some(FileLoader::new());

		println!("\n:: Collect resources ::");

		if let Some(res) = self.resources.as_mut() {
			res.collect(&self.root).unwrap();
		}

		println!("\n:: Set pages ::");

		if let Some(container) = &self.resources {
			for (path, _buffer) in &container.files {
				if path.contains(".html") {
					self.pages.set(path.clone())
				}
			}
		}

		println!("\n:: Init fs watcher ::");

		if let Some(instance) = self.watcher.as_mut() {
			instance.init()
		}

		println!("\n:: Await new connection at {} ::", self.tcp.local_addr().unwrap());

		loop {
			match self.tcp.accept() {
				Ok(( stream, _addr )) => {
					self.handler(stream);
				}, 
				Err(error) => {
					println!("{}", error)
				}
			}
		}

	}

	fn get_root(&mut self) -> Option<String> {

		match FileDialog::new()
			.set_location("~/")
			.show_open_single_dir()
			.unwrap() {
			Some(path) 
				=> Some(String::from(path.to_string_lossy())),
			None 
				=> None
		} 

	}

	fn handler(&mut self, mut stream: TcpStream) {

		use response::get_content;

		let mut buffer: [u8; 1024] = [0; 1024];

		stream.read(&mut buffer).unwrap();

		let mut resourse_path = String::new();

		for (i, line) in String::from_utf8_lossy(&buffer[..]).lines().enumerate() {
			match i {
				0 => {
					for req in String::from(line).split(" ") {

						if String::from(req).starts_with("/") {
							resourse_path = self.redirect(String::from(req))
						}

					}
				},
				_ => (),
			}
		};

    stream.write(&get_content(&self, resourse_path)).unwrap();
		stream.flush().unwrap();

	}

	fn redirect(&mut self, path: String) -> String {

		let file_req: bool = path.contains(".");

		let mut new_path: String = self.redirect.path.clone();

		self.redirect.dist = match path.as_str() {
			"/" 
				=> Dist::Absolute,
			_ 
				=> Dist::Relative,
		};

		println!("{:?}",self.redirect.dist);

		match self.redirect.dist {
			Dist::Absolute => {
				new_path = path.clone()
			},
			Dist::Relative => {
				new_path += path.as_str()
			}
		}

		if !file_req {
			new_path = self.apply_index(new_path) 
		}

		return new_path;

	}

	fn apply_index(&mut self, path: String) -> String {
		match path.as_str() {
			"/" 
				=> String::from(path + "index.html"),
			_ 
				=> String::from(path + "/index.html")
		}
	}

}