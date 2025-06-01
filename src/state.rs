use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy)]
pub enum Focused {
    Editor,
    SideBar,
}

impl Default for Focused {
    fn default() -> Self {
        Self::Editor
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Calm,
    Vibe,
    Modern,
    Professional,
    Creative,
    Warm,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Professional
    }
}

impl Theme {
    pub fn accent_color(&self) -> Style {
        match self {
            Theme::Calm => Style::new().fg(Color::LightYellow),
            Theme::Vibe => Style::new().fg(Color::LightBlue),
            Theme::Modern => Style::new().fg(Color::LightCyan),
            Theme::Professional => Style::new().fg(Color::Cyan),
            Theme::Creative => Style::new().fg(Color::LightMagenta),
            Theme::Warm => Style::new().fg(Color::LightRed),
        }
    }

    pub fn next_option(&self) -> Self {
        match self {
            Theme::Calm => Theme::Vibe,
            Theme::Vibe => Theme::Modern,
            Theme::Modern => Theme::Professional,
            Theme::Professional => Theme::Creative,
            Theme::Creative => Theme::Warm,
            Theme::Warm => Theme::Calm,
        }
    }
}
