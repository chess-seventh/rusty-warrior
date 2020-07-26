use std::process::Command;
use serde::{Deserialize, Serialize};
use std::fmt;
use notify_rust::{Notification};
use std::ops::Not;


#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    id: i64,
    priority: Option<String>,
    project: String,
    status: String,
    urgency: f64,
    uuid: String
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "description: {}\nid: {}\nproject: {}\nstatus: {}\nurgency: {}\nuuid: {}", self.description, self.id, self.project, self.status, self.urgency, self.uuid)
    }
}

fn main() {

    // Read taskwarrior tasks from CLI
    let output = Command::new("task")
        .arg("export")
        .arg("+READY")
        .output()
        .expect("failed to execute process");

    // Format taskwarrior tasks in JSON
    let mut tasks = serde_json::from_str::<Vec<Task>>(&String::from_utf8(output.stdout).unwrap())
        .expect("Invalid JSON")
        .into_iter()
        .filter(|task| task.project.contains("bm").not())
        .collect::<Vec<_>>();

    // Sort tasks by urgency
    tasks.sort_by(|a, b| b.urgency.partial_cmp(&a.urgency).unwrap());

    for task in tasks[0..3].iter()  {
        let mut body = String::new();
        body.push_str("id: ");
        body.push_str(&task.id.to_string());
        body.push_str("\nproject: ");
        body.push_str(&task.project);
        body.push_str("\nurgency: ");
        if task.priority.is_some() {
            body.push_str(&task.priority.as_ref().unwrap().to_owned());
        }

        Notification::new()
            .summary(&task.description)
            .hint(notify_rust::Hint::Transient(true))
            .body(&body)
            .appname("taskwarrior")
            .show()
            .unwrap();
    }
}
