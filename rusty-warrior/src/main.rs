use std::process::Command;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    entry: String,
    id: i64,
    modified: String,
    priority: String,
    project: String,
    status: String,
    //tags: Vec<String>,
    urgency: f64,
    uuid: String
}

//Object({"description": String("configure swisscom tv to get access to the internet"), "entry": String("20191110T070112Z"), "id": Number(2), "modified": String("20200308T010523Z"), "priority": String("M"), "project": String("lab.network"), "status": String("pending"), "tags": Array([String("tv"), String("routing"), String("vlan")]), "urgency": Number(18.4027), "uuid": String("ce64ab4f-a647-44e1-9890-c16c63034837")}),

fn sort_tasks() {

}

fn serialize_tasks(raw_task: serde_json::Value) -> Task {
    let task = Task {
        description: raw_task["description"].as_str().unwrap().to_owned(),
        entry: raw_task["entry"].as_str().unwrap().to_owned(),
        id: raw_task["id"].as_i64().unwrap().to_owned(),
        modified: raw_task["modified"].as_str().unwrap().to_owned(),
        priority: raw_task["priority"].as_str().unwrap().to_owned(),
        project: raw_task["project"].as_str().unwrap().to_owned(),
        status: raw_task["status"].as_str().unwrap().to_owned(),
        //tags: ["tag1", "tag2"],
        urgency: raw_task["urgency"].as_f64().unwrap().to_owned(),
        uuid: raw_task["uuid"].as_str().unwrap().to_owned(),
    };

    //let task = Task {
        //description: String::from("configure swisscom tv to get access to the internet"),
        //entry: String::from("20191110T070112Z"),
        //id: 2,
        //modified: String::from("20200308T010523Z"),
        //priority: String::from("M"),
        //project: String::from("lab.network"),
        //status: String::from("pending"),
        ////tags: ["tag1", "tag2"],
        //urgency: 18.4027,
        //uuid: String::from("ce64ab4f-a647-44e1-9890-c16c63034837")
    //};
    return task
}


fn main() {
    let output = Command::new("task")
        .arg("export")
        .arg("+READY")
        .output()
        .expect("failed to execute process");

    let task_read: serde_json::Value = json!(serde_json::from_str(&String::from_utf8(output.stdout).unwrap()).expect("JSON failed"));

    //println!("{:?}", json_read);
    for task in task_read.as_array().iter() {
        let mut ser_task = serialize_tasks(task);
        println!("{:?}", task);
    }

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

