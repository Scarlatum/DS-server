// STD
use std::net::{ TcpStream, TcpListener };
use std::thread;
use std::time;
use std::io::prelude::*;
use std::sync::mpsc::{ Receiver, Sender, channel };

// Vendor
use native_dialog::FileDialog;

// Watcher module
use crate::watcher::{ Watcher };
use crate::utils;

// todo Connection module •
// mod connection;
// use connection::Connection;

// Response module
mod response; 
use response::get_content;

// Resources
mod resources;
use resources::FileLoader;

// Page module
mod pages;
use pages::PagesDict;

const DEFAULT_HOST: &str = "0.0.0.0";

pub struct ServerParams {
	pub port: String
}

struct Redirect {
	path: String,
}

pub struct Server {
	root					: String,
	tcp						: TcpListener,
	redirect			: Redirect,
	pages					: PagesDict,
	resources			: Option<FileLoader>,
}

impl Server {

	pub fn new(params: ServerParams) -> Server {

		Server {
			root				: String::default(),

			tcp					: TcpListener::bind(format!("{}:{}", DEFAULT_HOST, params.port)).unwrap(),
			redirect 		: Redirect { path: String::default() },

			pages				: PagesDict::new(),
			resources 	: None,
		}

	}

	pub fn init(&mut self) -> () {

		self.root 			= self.get_root().unwrap();
		self.resources 	= Some(FileLoader::new());

		println!("\n:: Collect resources ::");

		if let Some(res) = self.resources.as_mut() {
			res.collect(&self.root).unwrap();
		}

		println!("\n:: Set pages ::");
		self.set_pages();

		// let ( app_tx, app_rx ) = channel();
		// let ( watcher_tx, watcher_rx ) = channel();

		// println!("\n:: Init fs watcher ::");
		// self.watcher_task(self.root.clone(), app_rx, watcher_tx);

		println!("\n:: Await new connection at {} ::", self.tcp.local_addr().unwrap());
		self.application_loop();

	}

	// !todo rewrite whole method as non-blocking.
	fn application_loop(&mut self) {
		
		// ! Так как TCP.accept() блокирует поток, то получить что-то из соседнего потока нет никакой-то возможности.
		// ! Но из-за `self.tcp.set_nonblocking(true)` нужно решать проблемы сокетов...
		// self.tcp.set_nonblocking(true)

		loop {
			
			// match rx.recv() {
			// 	Ok(status) => {
			// 		if status { 
			// 			if let Some(res) = self.resources.as_mut() {
			// 				println!(":: Update files ::");
			// 				res.collect(&self.root).unwrap();
			// 			}
			// 		};
			// 	},
			// 	Err(err) => println!("{}", err),
			// }

			match self.tcp.accept() {
				Ok(( stream, _ )) => {
					self.handler(stream);
				},
				Err(_) => (),
			}

		}
	}

	fn watcher_task(&mut self, root: String, tx: Sender<bool>) {

		thread::spawn(move || {

			let mut watcher = Watcher::new(root); watcher.init();

			loop {

				thread::sleep(time::Duration::from_secs(1));

				if watcher.check_files() {
					tx.send(true).unwrap(); watcher.updated = Vec::new();
				} else {
					tx.send(false).unwrap();
				};

			}

		});

	}

	fn set_pages(&mut self) {
		if let Some(container) = &self.resources {
			for (path, _buffer) in &container.files {
				if path.contains(".html") {

					let mut catalog = path
						.replace(&self.root, "")
						.replace("\\", "/")
						.replace("/index.html", "");

					if catalog.is_empty() {
						catalog += "/"
					}

					self.pages.set(catalog);

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

		let mut stream_buffer: [u8; 1024] = [0; 1024];
		let mut resourse_path = String::new();

		stream.read(&mut stream_buffer).expect("sdfsdfsdf");

		for (i, line) in String::from_utf8_lossy(&stream_buffer[..]).lines().enumerate() {
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

		// Send requested file 
    stream.write(&get_content(&self, resourse_path)).unwrap();
		stream.flush().unwrap();

	}

	fn redirect(&mut self, path: String) -> String {

		let is_file: bool = path.contains(".");
		let is_page: bool = self.pages.is_page(&path);
		
		let mut new_path = self.redirect.path.clone() + &path;

		if is_page { 

			self.redirect.path = path.clone();
			
			new_path = utils::web::path::apply_index(&path); 

		}

		if is_file {
			new_path = match self.redirect.path.as_str() {
				"/" 
					=> path,
				_ 
					=> format!("{}{}", &self.redirect.path, &path)
			}
		}

		return new_path;

	}

}