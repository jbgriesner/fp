#[derive(Clone)]
pub struct PasswordEntry {
    pub url: String,
    pub tags: Vec<String>,
    pub password: String,
}

impl PasswordEntry {
    pub fn new() -> Self {
        PasswordEntry {
            url: "".to_string(),
            tags: vec![],
            password: "".to_string(),
        }
    }
}
