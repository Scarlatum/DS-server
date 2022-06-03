use std::fs;
use std::collections::{ HashSet };
use std::time::SystemTime;
use std::io::Result;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ListenerTarget {
  modified  : SystemTime,
  path      : String,
}

pub struct Watcher {
  root  : String,
  files : HashSet<ListenerTarget>,
}

impl Watcher {

  pub fn new(root: String) -> Watcher {
    Watcher { 
      root, 
      files : HashSet::new(), 
    }
  }

  pub fn init(&mut self) {

    self.collect_dir(String::from(&self.root)).unwrap();

    for (i, file) in self.files.iter().enumerate() {
      println!("{}: {:?}", i, file)
    }

  }

  fn collect_dir(&mut self, path: String) -> Result<()> {

    for entry in fs::read_dir(path)? {

      let entry = entry?;
      let path = entry.path();
      let key = String::from(path.as_os_str().to_string_lossy());

      if path.is_file() {

        self.files.insert(ListenerTarget {
          modified: path.metadata()?.modified()?,
          path: key.clone(),
        });

      } else { 
        self.collect_dir(key).unwrap(); 
      }

    }

    Ok(())

  }

  // fn check_dir() {
  // }

}