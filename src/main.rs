use std::{ error::Error, io };

mod app;
use crate::app::{ App, Screen, LoginField };
use figlet_rs::FIGfont;
use tui_input::backend::crossterm::EventHandler;

use crossterm::{
    event::{ self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};
use ratatui::{
    backend::{ Backend, CrosstermBackend },
    layout::{ Constraint, Direction, Layout },
    style::{ Color, Modifier, Style },
    widgets::{ Block, Borders, Paragraph },
    Terminal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App
) -> Result<(), Box<dyn Error>>
    where B::Error: 'static
{
    loop {
        if app.should_quit {
            return Ok(());
        }

        terminal.draw(|f| {
            let size = f.area();

            // Main Shell Wrapper
            let main_block = Block::default()
                .title(format!(" InsTUI [{:?}] ", app.current_screen))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            let inner_area = main_block.inner(size);
            f.render_widget(main_block, size);

            match app.current_screen {
                Screen::Login => {
                    let vertical = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(7), // Space for Logo
                            Constraint::Length(3), // Username
                            Constraint::Length(3), // Password
                            Constraint::Length(3), // Help text
                            Constraint::Min(0),
                        ])
                        .split(inner_area);

                    let font = FIGfont::standard().unwrap();
                    let figure = font.convert("InsTUI").unwrap();
                    let logo = figure.to_string();

                    let logo_para = Paragraph::new(logo)
                        .style(Style::default().fg(Color::Magenta))
                        .alignment(ratatui::layout::Alignment::Center);
                    f.render_widget(logo_para, vertical[0]);

                    let username_block = Block::default()
                        .title(" Username ")
                        .borders(Borders::ALL)
                        .border_style(
                            if app.active_field == LoginField::Username {
                                Style::default().fg(Color::Yellow)
                            } else {
                                Style::default()
                            }
                        );

                    let password_block = Block::default()
                        .title(" Password ")
                        .borders(Borders::ALL)
                        .border_style(
                            if app.active_field == LoginField::Password {
                                Style::default().fg(Color::Yellow)
                            } else {
                                Style::default()
                            }
                        );

                    let username_para = Paragraph::new(app.username_input.value()).block(
                        username_block
                    );
                    let password_placeholder = "*".repeat(app.password_input.value().len());
                    let password_para = Paragraph::new(password_placeholder).block(password_block);

                    f.render_widget(username_para, vertical[1]);
                    f.render_widget(password_para, vertical[2]);

                    let help = Paragraph::new("Tab: Switch Field | Enter: Login | q: Quit");
                    f.render_widget(help, vertical[3]);

                    let (active_rect, active_input) = match app.active_field {
                        LoginField::Username => (vertical[1], &app.username_input),
                        LoginField::Password => (vertical[2], &app.password_input),
                    };
                    f.set_cursor_position((
                        active_rect.x + (active_input.visual_cursor() as u16) + 1,
                        active_rect.y + 1,
                    ));
                }
                _ => {
                    let text = match app.current_screen {
                        Screen::Login => unreachable!(),
                        Screen::ThreadList => "Thread List (Tab: Messages, Esc: Logout, q: Quit)",
                        Screen::MessageView => "Messages (Esc: Back to List, q: Quit)",
                    };

                    let paragraph = Paragraph::new(text);
                    f.render_widget(paragraph, inner_area);
                }
            }
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if app.current_screen == Screen::Login {
                        match key.code {
                            KeyCode::Char('q') => app.quit(),
                            KeyCode::Tab | KeyCode::Down | KeyCode::Up => app.switch_field(),
                            KeyCode::Enter => {
                                match app.active_field {
                                    LoginField::Username => app.switch_field(),
                                    LoginField::Password => app.next_screen(),
                                }
                            }
                            _ => {
                                match app.active_field {
                                    LoginField::Username =>
                                        app.username_input.handle_event(&Event::Key(key)),
                                    LoginField::Password =>
                                        app.password_input.handle_event(&Event::Key(key)),
                                };
                            }
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') => app.quit(),
                            KeyCode::Tab => app.next_screen(),
                            KeyCode::Esc => app.prev_screen(),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
