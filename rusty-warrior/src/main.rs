extern crate config;

use std::process::Command;
use serde::{Deserialize, Serialize};
use std::fmt;
use notify_rust::{Notification};
use std::ops::Not;

use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug, Clone)]
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

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let settings_parsed = settings.try_into::<HashMap<String, String>>().unwrap();

    let nb_tasks : usize;
    let mut appname = String::new();
    let mut exclude_project = String::new();

    // Verify that there is a proper value in the Settings.toml file.
    match settings_parsed.get("tasks_to_show") {
        Some(nb) => {
            nb_tasks = nb.parse().unwrap();
        }
        None => {
            nb_tasks = 3;
        }
    }

    // Verify that there is a proper value in the Settings.toml file.
    match settings_parsed.get("exclude_project") {
        Some(proj) => {
            exclude_project = proj.parse().unwrap();
        }
        None       => {
            exclude_project.push_str("");
        }
    }

    // Verify that there is a proper value in the Settings.toml file.
    match settings_parsed.get("dunst_appname") {
        Some(app) => {
            appname = app.parse().unwrap();
        }
        None       => {
            appname.push_str("");
        }
    }

    // Read taskwarrior tasks from CLI
    let output = Command::new("task")
        .arg("export")
        .arg("+READY")
        .output()
        .expect("failed to execute process");

    // Format taskwarrior tasks in JSON
    let mut tasks : std::vec::Vec<Task>;

    // Check if there's a filter to apply
    if exclude_project.len() > 0 {
        tasks = serde_json::from_str::<Vec<Task>>(&String::from_utf8(output.stdout).unwrap())
            .expect("Invalid JSON")
            .into_iter()
            .filter(|task| task.project.contains(&exclude_project).not())
            .collect::<Vec<_>>();
    } else {
        tasks = serde_json::from_str::<Vec<Task>>(&String::from_utf8(output.stdout).unwrap())
            .expect("Invalid JSON")
            .into_iter()
            .collect::<Vec<_>>();
    }

    // Sort tasks by urgency
    tasks.sort_by(|a, b| b.urgency.partial_cmp(&a.urgency).unwrap());

    // Send tasks notification to userspace
    for task in tasks[0..nb_tasks].iter()  {
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
            .appname(&appname)
            .show()
            .unwrap();
    }
}
