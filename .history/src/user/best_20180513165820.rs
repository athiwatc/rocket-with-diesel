pub struct Admin {
  pub name: String
}

impl Admin {
  pub fn new(name: String, token: String) -> Admin {
    Admin {name}
  }
}