use cascade::cascade;
use gtk::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

use gtk::WidgetExt;

use crate::googleapi;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    kind: String,
    etag: String,
    title: String,
    updated: String,
    selfLink: String,
}

pub fn open_login_window(gtm: &super::main::GTM) {
    let cloned_auth_window = gtm.auth_window.clone();
    let cloned_tasks_container = gtm.tasks_container.clone();
    let cloned_tasks_access_message = gtm.tasks_access_message.clone();
    &gtm.auth_button.connect_clicked(move |_| {
        let authenticated = googleapi::authentication::authenticate();
        if authenticated {
            match googleapi::tasks::get_lists() {
                Ok(v) => {
                    let tasks: Vec<Task> = serde_json::from_str(&v).expect("Incorrect task format");
                    for task in tasks.iter() {
                        let task_title_button = cascade! {
                            task_title_button: gtk::Button::new_with_label(&task.title.to_string());
                            ..set_widget_name("task_title_button");
                        };
                        cloned_tasks_container.add(&task_title_button);
                    }
                    cloned_tasks_container.show_all();
                    cloned_tasks_access_message.hide();
                    cloned_auth_window.close();
                }
                Err(_) => println!("No tasks recieved"),
            }
        } else {
        }
    });
    &gtm.auth_window.show_all();
}
