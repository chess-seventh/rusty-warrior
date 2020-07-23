use std::process::Command;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    entry: String,
    id: u8,
    modified: String,
    priority: String,
    project: String,
    status: String,
    tags: Vec<String>,
    uuid: String
}

fn main() {
    let output = Command::new("task")
                 .arg("export")
                 .arg("+READY")
                 .output()
                 .expect("failed to execute process");

        let handle: notify_rust::NotificationHandle = Notification::new()
            .summary("Notification that will go away")
            .hint(notify_rust::Hint::Transient(true))
            .body("Lorem\nIpsum")
            .show()
            .unwrap();
        handle.wait_for_action(|action| {
            if "__closed" == action {
                println!("the notification window was closed")
            }
        });
    let json_read: serde_json::Value = serde_json::from_str(&String::from_utf8(output.stdout).unwrap()).expect("JSON failed");
    println!("{:?}", json_read);
}

