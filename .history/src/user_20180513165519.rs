pub struct User {
  pub name: String,
  token: String
}

pubimpl User {
   fn new(name: String, token: String) -> User {
    User {name, token}
  }
}