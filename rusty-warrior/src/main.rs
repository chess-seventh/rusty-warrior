use std::process::Command;
use serde::{Deserialize, Serialize};
use std::fmt;
use notify_rust::{Notification, Hint};
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
    let output = Command::new("task")
        .arg("export")
        .arg("+READY")
        .output()
        .expect("failed to execute process");

    let tasks = serde_json::from_str::<Vec<Task>>(&String::from_utf8(output.stdout).unwrap())
        .expect("Invalid JSON")
        .into_iter()
        .filter(|task| task.project.contains("bm").not())
        .collect::<Vec<_>>();

    for task in tasks.iter()  {
        //println!("{:?}", task.description);

        let handle: notify_rust::NotificationHandle = Notification::new()
            .summary(&task.description)
            .hint(notify_rust::Hint::Transient(true))
            //.hint(Hint::Category("taskwarrior".to_owned()))
            .body(&task.project)
            .appname("taskwarrior")
            .show()
            .unwrap();

        handle.wait_for_action(|action| {
            if "__closed" == action {
                println!("the notification window was closed")
            }
        });
    }
}
