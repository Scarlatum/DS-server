use std::collections::HashSet;

pub struct PagesDict {
  pub list: HashSet<String>
}

impl PagesDict {

  pub fn new() -> Self {
    PagesDict { list: HashSet::new() }
  }

  pub fn is_page(&self, path: String) -> bool {
    self.list.contains(&path)
  }

  pub fn set(&mut self, path: String) -> () {
    self.list.insert(path.clone());
  }

}