use std::process::Command;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;


#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    id: i64,
    priority: String,
    project: String,
    status: String,
    urgency: f64,
    uuid: String
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "description: {}\nid: {}\npriority: {}\nproject: {}\nstatus: {}\nurgency: {}\nuuid: {}", self.description, self.id, self.priority, self.project, self.status, self.urgency, self.uuid)
    }
}

fn serialize_tasks(raw_task: serde_json::Value) -> Option<Task> {
    if !raw_task["project"].as_str().unwrap().to_owned().contains("bm") {
        let task = Task {
            description: raw_task["description"].as_str().unwrap().to_owned(),
            id: raw_task["id"].as_i64().unwrap().to_owned(),
            priority: raw_task["priority"].as_str().unwrap_or("NOPRIO").to_owned(),
            project: raw_task["project"].as_str().unwrap().to_owned(),
            status: raw_task["status"].as_str().unwrap().to_owned(),
            urgency: raw_task["urgency"].as_f64().unwrap().to_owned(),
            uuid: raw_task["uuid"].as_str().unwrap().to_owned(),
        };
        return Some(task)
    } else {
        return None
    }
}


fn main() {
    let output = Command::new("task")
        .arg("export")
        .arg("+READY")
        .output()
        .expect("failed to execute process");

    let task_read: serde_json::Value = serde_json::from_str(&String::from_utf8(output.stdout).unwrap()).expect("JSON failed");

    let mut struct_tasks = Vec::new();

    for task in task_read.as_array().iter() {
        for t in task.iter() {
            struct_tasks.push(serialize_tasks(json!(t.to_owned())).unwrap());
        }
    }

    struct_tasks.sort_by(|a, b| b.urgency.partial_cmp(&a.urgency).unwrap());
    println!("{}", struct_tasks[0]);
    println!("{}", struct_tasks[1]);
    println!("{}", struct_tasks[2]);

    //for i in struct_tasks.iter() {
        //println!("{}", i);
    //}

}

    //let handle: notify_rust::NotificationHandle = Notification::new()
        //.summary("Notification that will go away")
        //.hint(notify_rust::Hint::Transient(true))
        //.body("Lorem\nIpsum")
        //.show()
        //.unwrap();

    //handle.wait_for_action(|action| {
        //if "__closed" == action {
            //println!("the notification window was closed")
        //}
    //});
