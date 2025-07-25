use libadwaita::prelude::*;
use libadwaita::Application;
use crate::ui::main_window;

pub fn run() {
    let app = Application::builder()
        .application_id("com.nikelaz.ctlDash")
        .build();

    app.connect_activate(|app| {
        let main_window = main_window::MainWindow::new(app);
        main_window.present();
    });

    app.run();
}