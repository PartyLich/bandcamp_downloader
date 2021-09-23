use iced::{button, progress_bar, Background, Color};

#[derive(Debug, PartialEq)]
pub enum Theme {
    Light,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

const MAX: f32 = 255.0;
impl Theme {
    pub const ALL: [Theme; 1] = [Theme::Light];
    const BUTTON_BG: Color = Color::from_rgb(221.0 / MAX, 221.0 / MAX, 221.0 / MAX);
    const PROGRESS_BAR_BG: Color = Color::from_rgb(230.0 / MAX, 230.0 / MAX, 230.0 / MAX);
    const PROGRESS_BAR_FG: Color = Color::from_rgb(6.0 / MAX, 176.0 / MAX, 37.0 / MAX);
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(theme: Theme) -> Self {
        Button(theme).into()
    }
}

impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
    fn from(theme: Theme) -> Self {
        ProgressBar(theme).into()
    }
}

#[derive(Debug)]
pub struct Button(Theme);

impl Default for Button {
    fn default() -> Self {
        Self(Theme::default())
    }
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Self(Theme::Light) => button::Style {
                background: Some(Background::Color(Theme::BUTTON_BG)),
                border_radius: 5,
                ..button::Style::default()
            },
        }
    }
}

#[derive(Debug)]
pub struct ProgressBar(Theme);

impl Default for ProgressBar {
    fn default() -> Self {
        Self(Theme::default())
    }
}

impl progress_bar::StyleSheet for ProgressBar {
    fn style(&self) -> progress_bar::Style {
        match self {
            Self(Theme::Light) => progress_bar::Style {
                background: Background::Color(Theme::PROGRESS_BAR_BG),
                bar: Background::Color(Theme::PROGRESS_BAR_FG),
                border_radius: 10,
            },
        }
    }
}
