use gtk4::gdk::ModifierType;
use gtk4::subclass::prelude::ApplicationImpl;
use gtk4::{prelude::*, EventControllerKey, Shortcut, ShortcutAction, ShortcutTrigger};
use gtk4::{Application, ApplicationWindow, Label};

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

        build_window(&window);
    });

    app.run();
}

fn build_window(window: &ApplicationWindow) {
    // window.fullscreen();
    window.set_default_width(500);
    window.set_default_height(500);
    window.show();
}
