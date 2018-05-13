struct User {
  pub name: String,
  token: String
}

impl User {
 fn new(name: String, token: String) -> User {
    User {name, token}
  }
}