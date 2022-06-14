use std::collections::HashMap;
use std::io::Result;
use std::fs;

pub struct FileLoader {
  pub files: HashMap<String, Vec<u8>>
}

impl FileLoader {

  pub fn new() -> FileLoader {
    FileLoader {
      files: HashMap::new(),
    }
  }

  pub fn get(&self, path: &String) -> Option<&Vec<u8>> {
    return self.files.get(path);
  }

  pub fn set(&mut self, path: &String, data: Vec<u8>) {
    self.files.insert(path.clone(), data);
  }

  pub fn collect(&mut self, root: &String) -> Result<()> {

    for entry in fs::read_dir(root)? {

      let path = entry?.path();
      let key = String::from(path.as_os_str().to_string_lossy());

      println!("{}", key);

      if path.is_file() {

        match fs::read(&key) {
          Ok(buffer) 
            => self.set(&key, buffer),
          Err(err) 
            => println!("{}", err)
        }

      }

      if path.is_dir() {
        self.collect(&key).unwrap();
      }

    }

    Ok(())

  }

}