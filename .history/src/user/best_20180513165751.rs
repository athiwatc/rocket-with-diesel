pub struct Admin {
  pub name: String,
  token: String
}

impl Admin {
  pub fn new(name: String, token: String) -> User {
    User {name, token}
  }
}