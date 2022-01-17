pub struct Route {
    pub prefix: String,
}

impl Route {
    #[warn(clippy::new_ret_no_self)]
    pub fn new(&self, route: String) -> String {
        self.prefix.to_owned() + &route
    }
}
