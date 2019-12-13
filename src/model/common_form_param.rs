#[derive(Deserialize, Clone)]
pub struct CommonFormParam {
    pub index: Option<i64>,
    pub limit: Option<i64>,
    pub keyword: Option<String>,
}

impl CommonFormParam {
    pub fn offset(&self) -> i64 {
        (self.index.unwrap_or(1) - 1) * self.limit.unwrap_or(100)
    }
}
