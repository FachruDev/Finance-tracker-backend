use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub kind: String, // "income" | "expense"
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub kind: Option<String>,
    pub color: Option<String>,
}

