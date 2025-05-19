mod app;
mod ui;
mod system;

// Removed unused import: crate::system::systemctl

fn main() {
    app::run();
}
