use crate::domain::ProjectPlan;

pub fn render_summary(project: &ProjectPlan) -> String {
    let completed = project
        .tasks
        .iter()
        .filter(|task| task.status == "done")
        .count();

    let mut lines = vec![
        format!("Project: {}", project.name),
        format!("Goal: {}", project.goal),
        format!(
            "Progress: {completed}/{} tasks completed",
            project.tasks.len()
        ),
        String::from("Open work:"),
    ];

    for task in project.tasks.iter().filter(|task| task.status != "done") {
        lines.push(format!("- {} ({})", task.title, task.owner));
    }

    lines.join("\n")
}
