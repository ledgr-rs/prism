use ratatui::style::{Color, Modifier, Style};

pub const BG: Color = Color::Rgb(3, 5, 12);
pub const PANEL: Color = Color::Rgb(8, 10, 18);
pub const BORDER: Color = Color::Rgb(58, 62, 82);
pub const TEXT: Color = Color::Rgb(224, 228, 236);
pub const MUTED: Color = Color::Rgb(132, 138, 154);
pub const PURPLE: Color = Color::Rgb(139, 92, 246);
pub const ORANGE: Color = Color::Rgb(251, 146, 60);
pub const GREEN: Color = Color::Rgb(74, 222, 128);
pub const RED: Color = Color::Rgb(248, 113, 113);

pub fn base() -> Style {
    Style::default().fg(TEXT).bg(BG)
}

pub fn panel() -> Style {
    Style::default().fg(TEXT).bg(PANEL)
}

pub fn title() -> Style {
    Style::default()
        .fg(TEXT)
        .bg(BG)
        .add_modifier(Modifier::BOLD)
}

pub fn selected() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(ORANGE)
        .add_modifier(Modifier::BOLD)
}
