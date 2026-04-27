#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub status: String,
    pub owner: String,
}

impl Task {
    pub fn new(title: &str, status: &str, owner: &str) -> Self {
        Self {
            title: title.to_string(),
            status: status.to_string(),
            owner: owner.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectPlan {
    pub name: &'static str,
    pub goal: String,
    pub tasks: Vec<Task>,
}
