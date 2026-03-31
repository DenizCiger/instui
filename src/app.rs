use ratatui::widgets::ListState;
use tui_input::Input;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Login,
    ThreadList,
    MessageView,
}

pub struct App {
    pub current_screen: Screen,
    pub thread_list_state: ListState,
    pub should_quit: bool,
    pub username_input: Input,
    pub password_input: Input,
    pub active_field: LoginField,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoginField {
    Username,
    Password,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: Screen::Login,
            thread_list_state: ListState::default(),
            should_quit: false,
            username_input: Input::default(),
            password_input: Input::default(),
            active_field: LoginField::Username,
            session_id: None,
        }
    }

    pub fn next_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Login => Screen::ThreadList,
            Screen::ThreadList => Screen::MessageView,
            Screen::MessageView => Screen::MessageView,
        };
    }

    pub fn prev_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Login => Screen::Login,
            Screen::ThreadList => Screen::Login,
            Screen::MessageView => Screen::ThreadList,
        };
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn switch_field(&mut self) {
        self.active_field = match self.active_field {
            LoginField::Username => LoginField::Password,
            LoginField::Password => LoginField::Username,
        };
    }
}
