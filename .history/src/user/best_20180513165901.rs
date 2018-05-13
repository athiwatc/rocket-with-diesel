pub struct Admin {
  pub name: String
}

impl Admin {
   fn new(name: String, token: String) -> Admin {
    Admin {name}
  }
}