use std::fs;
use std::time::SystemTime;
use std::io::*;

#[derive(Debug, Clone)]
pub struct ListenerTarget {
  pub modified  : SystemTime,
  pub path      : String,
}

pub struct Watcher {
  root        : String,
  files       : Vec<ListenerTarget>,
  pub updated : Vec<ListenerTarget>
}

impl Watcher {

  pub fn new(root: String) -> Self {
    Watcher { 
      root, 
      files   : Vec::new(),
      updated : Vec::new(),
    }
  }

  pub fn init(&mut self) {

    self.files = match self.collect_dir(self.root.clone()) {
      Ok(fs_snaphot) 
        => fs_snaphot,
      Err(_) 
        => Vec::new()
    }
    
  }

  fn to_update(&mut self, file: ListenerTarget) {
    self.updated.push(file);
  }

  fn collect_dir(&mut self, path: String) -> Result<Vec<ListenerTarget>> {

    let mut set: Vec<ListenerTarget> = Vec::new();

    for entry in fs::read_dir(path)? {

      let path = entry?.path();
      let key = String::from(path.as_os_str().to_string_lossy());

      if path.is_file() {

        set.push(ListenerTarget {
          modified: path.metadata()?.modified()?,
          path: key.clone(),
        });

      } else { 
        self.collect_dir(key).unwrap(); 
      }

    }

    Ok(set)

  }

  pub fn check_files(&mut self) -> bool {

    let spanshot = self.collect_dir(self.root.clone()).unwrap();

    // Check quantity of both snapshots
    if spanshot.len() != self.files.len() {
      self.files = spanshot; return true; 
    }

    let mut updated: bool = false;

    // Check files
    for j in 0..spanshot.len() {
      for i in 0..self.files.len() {
        if spanshot[j].path == self.files[i].path && spanshot[j].modified != self.files[i].modified {
          updated = true; self.to_update(spanshot[j].clone())
        }
      }
    }

    // Apply snapshot
    if updated {
      self.files = spanshot;
    };

    return updated;

  }

}