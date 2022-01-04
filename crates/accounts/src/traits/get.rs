pub trait Get {
    fn get(&self, name: &str) -> String {
        format!("LoL: {}", name)
    }
}
