//! Reusable modules for the Rust template.

pub mod domain;
pub mod report;

use domain::{ProjectPlan, Task};

pub fn sample_project() -> ProjectPlan {
    ProjectPlan {
        name: "Tesseracs template refresh",
        goal: "Offer a realistic multi-file Rust starter".to_string(),
        tasks: vec![
            Task::new("Add reusable modules", "done", "template"),
            Task::new("Improve run instructions", "done", "docs"),
            Task::new("Replace sample data", "next", "you"),
        ],
    }
}
