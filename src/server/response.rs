use crate::Server;

pub fn get_content(instance: &Server, path: String) -> String {

  println!("");
  
  let parts = path.split(".");

  let ext = parts.last().unwrap();
  let path = format!("{}/{}", instance.root, path);

  let content_body = match std::fs::read_to_string(path) {
    Ok(value) 
      => value,
    Err(err) 
      => {
        println!("{}", err); return String::from("");
      }
  };

  let content_type = match ext {
    "svg" 
      => String::from("image/svg+xml"),
    "js"
      => String::from("application/javascript"),
    "css"
      => String::from("text/css"),
    "webp" | "png" | "jpg" | "avif"
      => format!("image/{}", ext),
    _ 
      => String::from("text/html")
  };

  println!(":: Request data ::");
  println!("type: {}", content_type);

  return format!(
      "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
      content_body.len(),
      content_type,
      content_body
  );

}