use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use std::fs;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App {
    state: AppState,
    running: bool,
    typed_text: String,
    code_index: usize,
    code_queue: Vec<char>,
    frame_count: u32,
    code_speed: usize,
}

#[derive(Debug, Default)]
enum AppState {
    #[default]
    Menu,
    Code,
}

impl App {
    pub fn new() -> Self {
        App {
            code_speed: 5,
            ..Default::default()
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        match &mut self.state {
            AppState::Menu => {
                let paragraph = Paragraph::new("Hacker Typer")
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(Color::Green))
                    .alignment(Alignment::Center);
                frame.render_widget(paragraph, frame.area());
            }

            AppState::Code => {
                let content = Text::from(self.typed_text.clone());
                let total_lines = content.lines.len() as u16;
                let visible_height = frame.area().height.saturating_sub(2);
                let scroll_offset = total_lines.saturating_sub(visible_height);

                let paragraph = Paragraph::new(content)
                    .block(Block::default().borders(Borders::ALL))
                    .scroll((scroll_offset, 0))
                    .style(Style::default().fg(Color::Green));

                frame.render_widget(paragraph, frame.area());
            }
        }
        self.frame_count += 1;
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Menu => {
                match key.code {
                    KeyCode::Enter => {
                        self.state = AppState::Code
                    }
                    _ => {}
                }
            }
            AppState::Code => {
                match (key.modifiers, key.code) {
                    (_, KeyCode::Esc | KeyCode::Char('q'))
                    | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                        self.quit()
                    }
                    _ => {
                        self.type_code()
                    }
                }
            }
        }
    }

    fn type_code(&mut self) {
        let code_text = fs::read_to_string("assets/code.txt").expect("Could not read file");

        self.typed_text
            .push(self.code_queue[0]);

        self.code_queue.remove(0);

        self.code_index += 1;

        if self.code_index >= code_text.len() {
            self.code_index = 0;
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
