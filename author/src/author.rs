pub struct Author {
  pub name: String,
  pub nationality: String,
}

impl Author {
    pub fn new(name: &str, country: &str) -> Self {
      Self {
        name: name.to_string(),
        nationality: country.to_string(),
      }
    }

    pub fn author_info(&self) {
      println!("Author: {0} is from {1}", self.name, self.nationality);
    }
}