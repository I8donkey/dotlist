use crate::parser::ListData;
use ratatui::{
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Paragraph, Wrap},
        Terminal,
};
use std::fs;
use std::io;

pub struct App {
        pub file_path: String,
        pub content: String,
        pub command: String,
        pub output: String,
        pub data: ListData,
}

impl App {
        pub fn new(file_path: &str) -> Self {
                let content = fs::read_to_string(file_path).unwrap_or_default();
                let data = ListData::from_string(&content).unwrap_or_else(|_| ListData::new());
                App {
                file_path: file_path.to_string(),
                content: content.clone(),
                command: String::new(),
                output: String::new(),
                data,
        }
        }

        fn highlight_list_syntax(&self) -> Vec<Line<'static>> {
                let mut lines = Vec::new();
                let chars: Vec<char> = self.content.chars().collect();
                let mut i = 0;

                while i < chars.len() {
                        match chars[i] {
                                '[' => { lines.push(Line::from(vec![Span::styled("[", Style::default().fg(Color::Yellow))])); }
                                ']' => { lines.push(Line::from(vec![Span::styled("]", Style::default().fg(Color::Yellow))])); }
                                ',' => { lines.push(Line::from(vec![Span::styled(",", Style::default().fg(Color::White))])); }
                                ':' if i + 1 < chars.len() && chars[i + 1] == ':' => {
                                        lines.push(Line::from(vec![Span::styled("::", Style::default().fg(Color::Cyan))]));
                                        i += 1;
                                }
                                ':' => { lines.push(Line::from(vec![Span::styled(":", Style::default().fg(Color::Green))])); }
                                ';' => { lines.push(Line::from(vec![Span::styled(";", Style::default().fg(Color::DarkGray))])); }
                                c if c.is_alphabetic() || c == '_' || c == '\\' => {
                                        let start = i;
                                        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\\' || chars[i] == ':') {
                                                i += 1;
                                        }
                                        let word: String = chars[start..i].iter().collect();
                                        if word.contains(':') || word.contains('\\') {
                                                lines.push(Line::from(vec![Span::styled(word.clone(), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))]));
                                        } else {
                                                lines.push(Line::from(vec![Span::styled(word.clone(), Style::default().fg(Color::Blue))]));
                                        }
                                        continue;
                                }
                                c if c.is_numeric() => {
                                        let start = i;
                                        while i < chars.len() && (chars[i].is_numeric() || chars[i] == '.') {
                                                i += 1;
                                        }
                                        let num: String = chars[start..i].iter().collect();
                                        lines.push(Line::from(vec![Span::styled(num, Style::default().fg(Color::Red))]));
                                        continue;
                                }
                                _ => {}
                        }
                        i += 1;
                }

                if lines.is_empty() {
                        lines.push(Line::from(""));
                }

                lines
        }
}

pub fn run_ide(file_path: &str) -> io::Result<()> {
        let mut app = App::new(file_path);
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        loop {
                terminal.draw(|f| {
                        let size = f.size();
                        let chunks = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints([
                                        Constraint::Min(10),
                                        Constraint::Length(3),
                                        Constraint::Length(3),
                                        Constraint::Length(5),
                                ])
                                .split(size);

                        let title = format!(" .list IDE - {} ", app.file_path);
                        let editor = Paragraph::new(app.highlight_list_syntax())
                                .block(Block::new().borders(Borders::ALL).title(title))
                                .wrap(Wrap { trim: true });
                        f.render_widget(editor, chunks[0]);

                        let command_label = " 命令 > ";
                        let command_input = Paragraph::new(app.command.as_str())
                                .block(Block::new().borders(Borders::ALL).title(command_label));
                        f.render_widget(command_input, chunks[1]);

                        let output_para = Paragraph::new(app.output.as_str())
                                .block(Block::new().borders(Borders::ALL).title(" 输出 "))
                                .wrap(Wrap { trim: true });
                        f.render_widget(output_para, chunks[2]);

                        let help_text = " [ESC] 退出 | [Enter] 执行命令 | [↑↓] 浏览 ";
                        let help = Paragraph::new(help_text)
                                .block(Block::new().borders(Borders::ALL));
                        f.render_widget(help, chunks[3]);
                })?;

                if let Ok(false) = crossterm::event::poll(std::time::Duration::from_millis(100)) {
                        continue;
                }

                if let Ok(event) = crossterm::event::read() {
                        match event {
                                crossterm::event::Event::Key(key) => match key.code {
                                        crossterm::event::KeyCode::Esc => break,
                                        crossterm::event::KeyCode::Enter => {
                                                let result = app.data.execute_command(&app.command);
                                                match result {
                                                        Ok(output) => {
                                                                app.output = output;
                                                                fs::write(&app.file_path, app.data.to_string()).ok();
                                                                app.content = fs::read_to_string(&app.file_path).unwrap_or_default();
                                                        }
                                                        Err(e) => app.output = format!("错误: {}", e),
                                                }
                                                app.command.clear();
                                        }
                                        crossterm::event::KeyCode::Char(c) => {
                                                app.command.push(c);
                                        }
                                        crossterm::event::KeyCode::Backspace => {
                                                app.command.pop();
                                        }
                                        _ => {}
                                },
                                _ => {}
                        }
                }
        }

        terminal.clear()?;
        Ok(())
}
