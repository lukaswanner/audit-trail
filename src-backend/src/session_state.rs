#[derive(Clone, Copy, Debug)]
pub struct UserSession {
    pub account_id: i32,
}

impl UserSession {
    pub fn new(account_id: i32) -> Self {
        Self { account_id }
    }
}



