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

//Object({"description": String("configure swisscom tv to get access to the internet"), "entry": String("20191110T070112Z"), "id": Number(2), "modified": String("20200308T010523Z"), "priority": String("M"), "project": String("lab.network"), "status": String("pending"), "tags": Array([String("tv"), String("routing"), String("vlan")]), "urgency": Number(18.4027), "uuid": String("ce64ab4f-a647-44e1-9890-c16c63034837")}),

fn main() {
    let output = Command::new("task")
        .arg("export")
        .arg("+READY")
        .output()
        .expect("failed to execute process");

    let task_read: serde_json::Value = serde_json::from_str(&String::from_utf8(output.stdout).unwrap()).expect("JSON failed");

    //println!("{:?}", json_read);
    //for task in task_read {
        //println!("{:?}", task);
    //}

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
}

