use crate::Server;

pub fn get_content(server: &Server, path: String) -> Vec<u8> {

  let parts = path.split(".");

  let ext = parts.last().unwrap();
  let path = format!("{}{}", server.root, path.replace("/", "\\"));

  let body  = get_file(&server, path);
  let content_type = define_extension(ext);

  println!(":: Request data :: type: {} | length: {}", content_type, body.len());

  let mut response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
    body.len(),
    content_type,
  ).into_bytes();

  response.extend(body);

  return response;

}

fn define_extension(name: &str) -> String {
  match name {
    "svg" 
      => String::from("image/svg+xml"),
    "js"
      => String::from("application/javascript"),
    "css"
      => String::from("text/css"),
    "webp" | "png" | "jpg" | "avif"
      => format!("image/{}", name),
    _ 
      => String::from("text/html")
  }
}

fn get_file(server: &Server, path: String) -> Vec<u8> {

  let buffer: Vec<u8> = Vec::new();

  match &server.resources {
    Some(loader) => {
      match loader.get(&path) {
        Some(file) => file.clone(),
        None => buffer
      }
    },
    None => buffer,
  }

}