use cascade::cascade;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Box, Button, Label, Orientation, Window};

pub struct GTM {
    pub main_window: gtk::ApplicationWindow,
    pub auth_window: gtk::Window,
    pub auth_container: gtk::Box,
    pub auth_button: gtk::Button,
    pub auth_desc: gtk::Label,
    pub title: gtk::Label,
    pub tasks_access_message: gtk::Label,
    pub tasks_container: gtk::Box,
}

impl GTM {
    pub fn new(app: &gtk::Application) -> GTM {
        let auth_button = cascade! {
          auth_button: Button::new_with_label("Allow access to Google tasks");
          ..set_widget_name("auth_button");
        };

        let auth_desc = cascade! {
          auth_desc: Label::new(Some("Google Task Manager requires access to your Google Tasks. Any data returned is purely for the purpose of the apps core functionality that provides the user with a way to interact with their Google Tasks."));
          ..set_line_wrap(true);
          ..set_justify(gtk::Justification::Center);
          ..set_widget_name("auth_desc");
        };

        let title = cascade! {
          Label::new(Some("Google Task Manager"));
          ..set_widget_name("title");
        };

        let login_title = cascade! {
          Label::new(Some("Google Task Manager"));
          ..set_widget_name("title");
        };

        let auth_container = cascade! {
          auth_container: Box::new(Orientation::Vertical, 0);
          ..add(&login_title);
          ..add(&auth_button);
          ..add(&auth_desc);
          ..set_widget_name("auth_container");
        };

        let auth_window = cascade! {
          auth_window: Window::new(gtk::WindowType::Toplevel);
          ..set_title("Authenticate");
          ..set_default_size(320, 250);
          ..set_position(gtk::WindowPosition::Center);
          ..add(&auth_container);
        };

        let tasks_access_message = cascade! {
          Label::new(Some("Please allow access to Google Tasks."));
          ..set_justify(gtk::Justification::Center);
        };

        let tasks_container = cascade! {
          Box::new(Orientation::Vertical, 0);
          ..add(&title);
          ..add(&tasks_access_message);
          ..set_widget_name("app_container");
        };

        let main_window = cascade! {
          main_windows: ApplicationWindow::new(app);
          ..set_title("Google Task Manager");
          ..set_default_size(400, 550);
          ..set_position(gtk::WindowPosition::Center);
          ..add(&tasks_container);
        };

        GTM {
            main_window,
            auth_window,
            auth_container,
            auth_button,
            auth_desc,
            title,
            tasks_access_message,
            tasks_container,
        }
    }
}
