use super::schema::posts;

#[derive(Queryable)]
pub struct Broker {
    pub id: i32,
    pub name: String,
    pub userid: String,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewBroker<'a> {
    pub name: &'a str,
    pub userid: &'a str,
}
