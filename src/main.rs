use bandcamp_downloader::{
    self,
    settings::UserSettings,
    ui::{IcedUi, Ui},
};

/// Create UI instance according to environment variable.
/// Default to gui
fn select_ui() -> Box<dyn Ui> {
    let ui = std::env::var("UI").unwrap_or_else(|_| String::from("gui"));

    match ui.as_str() {
        "tui" => todo!(),
        _ => Box::new(IcedUi::default()),
    }
}

fn main() {
    let ui = select_ui();

    let user_settings = UserSettings::default();
    ui.run(user_settings);
}
