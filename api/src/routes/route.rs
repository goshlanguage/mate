pub struct Router {
    pub prefix: String,
}

impl Router {
    pub fn new_route(&self, route: String) -> String {
        let prefix = self.prefix.to_owned();
        prefix + &route
    }
}
