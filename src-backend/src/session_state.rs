#[derive(Clone, Copy, Debug)]
pub struct UserSession {
    pub account_id: i32,
}

impl UserSession {
    pub fn new(account_id: i32) -> Self {
        Self { account_id }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ApiSession {
    pub account_id: i32,
    pub project_id: i32,
}

impl ApiSession {
    pub fn new(account_id: i32, project_id: i32) -> Self {
        Self {
            account_id,
            project_id,
        }
    }
}
