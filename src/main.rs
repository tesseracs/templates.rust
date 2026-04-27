use templates_rust::{report::render_summary, sample_project};

fn main() {
    let project = sample_project();
    println!("{}", render_summary(&project));
}
