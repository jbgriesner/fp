#[derive(Clone)]
pub struct Engine {}

impl Engine {
    pub fn search(&self, query: String) -> Vec<String> {
        let results = vec![query.clone(), query];
        results
    }
}
