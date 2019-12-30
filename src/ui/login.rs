use gtk::prelude::*;

use gtk::{ApplicationWindow, Box, Button, Label, Orientation, WidgetExt};

use crate::googleapi;

pub fn open_login_window(app: &gtk::Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Google Task Manager");
    window.set_default_size(320, 250);
    window.set_position(gtk::WindowPosition::Center);
    let title_label = Label::new(Some("Google Task Manager"));
    let authentication_button = Button::new_with_label("Allow access to Google tasks");
    let authentication_desc = Label::new(Some("Google Task Manager requires access to your Google Tasks. Any data returned is for the purpose of the apps core functions in providing the user with a way to interact with their Google Tasks."));
    authentication_desc.set_line_wrap(true);
    authentication_desc.set_justify(gtk::Justification::Center);
    WidgetExt::set_widget_name(&title_label, "title_label");
    WidgetExt::set_widget_name(&authentication_button, "authentication_button");
    WidgetExt::set_widget_name(&authentication_desc, "authentication_desc");
    let container = Box::new(Orientation::Vertical, 0);
    // We need to name it in order to apply CSS on it.
    WidgetExt::set_widget_name(&container, "login_container");
    container.add(&title_label);
    container.add(&authentication_button);
    container.add(&authentication_desc);
    window.add(&container);
    window.show_all();
    authentication_button.connect_clicked(move |_| {
        let authenticated = googleapi::authentication::authenticate();
        if authenticated {
            window.close();
        }
    });
}
