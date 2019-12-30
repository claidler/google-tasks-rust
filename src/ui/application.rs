use gio::prelude::*;
use gtk::prelude::*;

use gtk::Application;

pub fn load_application() {
    let application = Application::new(Some("com.google-task-manager.app"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = gtk::CssProvider::new();
        const STYLE: &str = super::style::STYLE;
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");

        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        super::login::open_login_window(app);
    });

    application.run(&[]);
}
