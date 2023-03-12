use gtk::gdk;
use gtk::prelude::*;

fn load_css(settings: &gtk::Settings) {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
    let theme_name = settings.gtk_theme_name().expect("Could not get theme name.");

    if theme_name.to_lowercase().contains("dark") || settings.is_gtk_application_prefer_dark_theme() {
        provider.load_from_data(include_str!("../styles/dark.css"));
    } else {
        provider.load_from_data(include_str!("../styles/light.css"));
    }

    
    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
}

fn on_activate(application: &gtk::Application) {
    if let Some(settings) = gtk::Settings::default() {
        settings.connect_gtk_application_prefer_dark_theme_notify(load_css);
        settings.connect_gtk_theme_name_notify(load_css);
        load_css(&settings);
    }

    let window = gtk::ApplicationWindow::new(application);
    let label = gtk::Label::new(Some("Hello, world!"));
    let width = 275;
    let height = 50;

    window.set_child(Some(&label));
    window.set_default_size(width, height);
    window.present();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.example.gtk4-dark-mode")
        .build();

    app.connect_activate(on_activate);
    app.run();
}
