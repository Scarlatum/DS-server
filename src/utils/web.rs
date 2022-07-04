pub mod path {
  pub fn apply_index(path: &String) -> String {
    match path.as_str() {
      "/" 
        => format!("{}{}", path, "index.html"),
      _ 
        => format!("{}{}", path, "/index.html")
    }
  }
}
