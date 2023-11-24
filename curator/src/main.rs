use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

fn main() {
    let app = Application::builder()
        .application_id("com.piflite.gtkapp")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("PiFlite")
            .build();

        let label = Label::new(Some("PiFlite"));

        window.set_child(Some(&label));
        // window.fullscreen();
        window.show();
    });

    app.run();
}
