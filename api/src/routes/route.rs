pub struct Route {
  pub prefix: String
}

impl Route {
  pub fn new(&self, route: String) -> String {
    return self.prefix.to_owned() + &route
  }
}
