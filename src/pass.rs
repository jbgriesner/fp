#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password {
    pub tags: Vec<String>,
    pub pwd: String,
    pub url: String,
}

impl Password {
    pub fn new() -> Self {
        return Password {
            tags: vec![],
            pwd: "".to_string(),
            url: "".to_string(),
        };
    }
}
