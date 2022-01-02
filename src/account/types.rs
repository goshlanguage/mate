/// AccountType enum differentiates between what kind of account is being managed.
/// TODO
///   drop dead_code lint allow once an Exchange is implemented
#[allow(dead_code)]
pub enum AccountType {
    Brokerage,
    Exchange,
}

pub struct Account {
    pub name: String,
    pub account_type: AccountType,
}

impl Account {
    pub fn new(name: &str, account_type: AccountType) -> Account {
        Account {
            name: name.to_string(),
            account_type,
        }
    }
}
