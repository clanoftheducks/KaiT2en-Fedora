mod collector;
mod diagnostics;
mod model;
mod ui;

use adw::prelude::*;

const APP_ID: &str = "org.t2powerexplorer.gtk";

fn install_quit_accelerator(app: &adw::Application) {
    let quit_action = gtk4::gio::SimpleAction::new("quit", None);
    let app_weak = app.downgrade();
    quit_action.connect_activate(move |_, _| {
        if let Some(app) = app_weak.upgrade() {
            app.quit();
        }
    });
    app.add_action(&quit_action);
    app.set_accels_for_action("app.quit", &["<Control>q"]);
}

fn main() {
    let app = adw::Application::builder().application_id(APP_ID).build();
    install_quit_accelerator(&app);
    app.connect_activate(ui::build);
    app.run();
}
