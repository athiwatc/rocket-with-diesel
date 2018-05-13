mod best;

pub struct User {
  pub name: String,
  token: String
}

impl User {
  pub fn new(name: String, token: String) -> User {
    User {name, token}
  }

  pub fn getAdmin(name: String) -> best::Admin {
    best::Admin{name}
  }
}