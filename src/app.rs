use crate::state::{Focused, Theme};
use ratatui::crossterm::event::{self, *};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Padding;
use ratatui::{
    DefaultTerminal, Frame,
    layout::Rect,
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};
use std::path::Path;
use std::{fs, io};
use tui_textarea::TextArea;
#[derive(Debug, Default)]
pub struct App<'a> {
    pub text: TextArea<'a>,
    pub current_path: String,
    pub recent_files: Vec<String>,
    pub dirty: bool,
    pub exit: bool,
    pub current_recent_file_index: usize,
    pub focused: Focused,
    pub theme: Theme,
}
impl<'a> App<'a> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(20)])
            .split(frame.area());

        self.draw_editor(frame, layout[0]);
        self.draw_sidebar(frame, layout[1]);
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) => {
                match self.focused {
                    Focused::Editor => {
                        self.text.input(key);
                    }
                    Focused::SideBar => {}
                }
                if key.kind == KeyEventKind::Press {
                    self.dirty = true;

                    self.handle_key_event(key);
                }
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('c') | KeyCode::Char('q')
                if key_event.modifiers.contains(event::KeyModifiers::CONTROL) =>
            {
                self.exit()
            }
            KeyCode::Char('n') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.create_file()
            }
            KeyCode::Char('l') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.save_file()
            }
            KeyCode::Char('o') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.open_file()
            }
            KeyCode::Char('p') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.save_file_as()
            }
            KeyCode::Char(' ') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.change_focus();
            }
            KeyCode::Up => {
                if let Focused::SideBar = self.focused {
                    self.current_recent_file_index =
                        (self.current_recent_file_index.saturating_sub(1))
                            % self.recent_files.iter().take(10).len()
                }
            }
            KeyCode::Down => {
                if let Focused::SideBar = self.focused {
                    self.current_recent_file_index =
                        (self.current_recent_file_index.saturating_add(1))
                            % self.recent_files.iter().take(10).len()
                }
            }
            KeyCode::Enter => {
                if let Focused::SideBar = self.focused {
                    self.current_path = String::from(
                        self.recent_files[self.current_recent_file_index as usize].clone(),
                    );
                    if let Ok(content) = fs::read_to_string(&self.current_path) {
                        self.text = TextArea::from(content.lines());
                    }
                }
            }
            KeyCode::Char('t') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.theme = self.theme.next_option();
            }
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
    fn change_path(&mut self, path: String) {
        if self.current_path != path {
            if !self.recent_files.contains(&self.current_path) {
                self.recent_files.insert(0, self.current_path.clone());
            }
        }
        self.current_path = String::from(path);
        if let Ok(content) = fs::read_to_string(&self.current_path) {
            self.text = TextArea::from(content.lines());
        }
    }

    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            if let Some(name) = path.to_str() {
                fs::write(name, self.text.lines().join("\n").clone()).unwrap();
                if let Ok(content) = fs::read_to_string(name) {
                    self.text = TextArea::from(content.lines());
                }
                self.change_path(String::from(name));
                self.dirty = false;
            }
        }
    }
    fn create_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            if let Some(name) = path.to_str() {
                fs::write(name, "").unwrap();
                if let Ok(content) = fs::read_to_string(name) {
                    self.text = TextArea::from(content.lines());
                }
                self.change_path(String::from(name));
                self.dirty = false;
            }
        }
    }
    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Some(name) = path.to_str() {
                if let Ok(content) = fs::read_to_string(name) {
                    self.text = TextArea::from(content.lines());
                }
                self.change_path(String::from(name));
                self.dirty = false;
            }
        }
    }

    fn save_file(&mut self) {
        fs::write(
            self.current_path.clone(),
            self.text.lines().join("\n").clone(),
        )
        .unwrap();
        self.dirty = false;
    }
    fn change_focus(&mut self) {
        self.focused = match &self.focused {
            Focused::Editor => Focused::SideBar,
            Focused::SideBar => Focused::Editor,
        };
    }
    fn draw_editor(&mut self, frame: &mut Frame, area: Rect) {
        let title = Line::from(
            format!(
                " lightning - text editor {}| {} |",
                if self.dirty { "*" } else { "" },
                self.current_path,
            )
            .bold(),
        );
        let instructions = Line::from(vec![
            " Quit ".into(),
            " <Ctrl+C> ".cyan().bold(),
            " Open ".into(),
            " <Ctrl+O> ".cyan().bold(),
            " New ".into(),
            " <Ctrl+N> ".cyan().bold(),
            " Save ".into(),
            " <Ctrl+L> ".cyan().bold(),
            " Save As ".into(),
            " <Ctrl+P> ".cyan().bold(),
        ]);
        let top_title = Line::from(vec![
            "      Change Theme ".into(),
            " <Ctrl+T> ".cyan().bold(),
        ]);
        let mut block = Block::bordered()
            .title(title.white())
            .title_top(top_title)
            .title_bottom(instructions)
            .padding(Padding::symmetric(2, 1))
            .border_set(border::ROUNDED);
        block = match self.focused {
            Focused::Editor => block.border_style(self.theme.accent_color()),
            _ => block,
        };
        self.text.set_block(block);
        self.text
            .set_line_number_style(Style::default().on_white().black().bold());
        self.text.set_max_histories(10);
        frame.render_widget(&self.text, area);
    }
    fn draw_sidebar(&self, frame: &mut Frame, area: Rect) {
        let names: Vec<String> = self
            .recent_files
            .iter()
            .take(10)
            .filter_map(|str| {
                Path::new(&str.clone())
                    .file_name()
                    .and_then(|val| val.to_str())
                    .map(|x| x.to_string())
            })
            .collect();

        let lines = names
            .iter()
            .enumerate()
            .map(|(ind, str)| {
                if ind == self.current_recent_file_index as usize {
                    Line::from(str.clone()).bold().on_white().black()
                } else {
                    Line::from(str.clone())
                }
            })
            .collect::<Vec<Line>>();

        let mut block = Block::bordered()
            .border_set(border::ROUNDED)
            .title(" Recent ");
        block = match self.focused {
            Focused::SideBar => block.border_style(self.theme.accent_color()),
            _ => block,
        };
        let paragraph = Paragraph::new(Text::from(lines)).block(block);

        frame.render_widget(paragraph, area);
    }
}
