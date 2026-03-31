use ratatui::widgets::ListState;
use tui_input::Input;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Login,
    ThreadList,
    MessageView,
}

#[derive(Debug, Clone)]
pub struct ChatThread {
    pub id: String,
    pub username: String,
    pub last_message: String,
    pub unread_count: u32,
}

pub struct App {
    pub current_screen: Screen,
    pub thread_list_state: ListState,
    pub threads: Vec<ChatThread>,
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
        let mut thread_list_state = ListState::default();
        thread_list_state.select(Some(0));

        App {
            current_screen: Screen::Login,
            thread_list_state,
            threads: vec![
                ChatThread {
                    id: "1".to_string(),
                    username: "john_doe".to_string(),
                    last_message: "Hey there!".to_string(),
                    unread_count: 2,
                },
                ChatThread {
                    id: "2".to_string(),
                    username: "jane_smith".to_string(),
                    last_message: "Check this out.".to_string(),
                    unread_count: 0,
                },
                ChatThread {
                    id: "3".to_string(),
                    username: "rust_lover".to_string(),
                    last_message: "TUIs are cool!".to_string(),
                    unread_count: 5,
                }
            ],
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

    pub fn logout(&mut self) {
        self.current_screen = Screen::Login;
        self.username_input = Input::default();
        self.password_input = Input::default();
        self.active_field = LoginField::Username;
    }

    pub fn prev_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Login => Screen::Login,
            Screen::ThreadList => Screen::ThreadList,
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

    pub fn next_thread(&mut self) {
        if self.threads.is_empty() {
            return;
        }
        let i = match self.thread_list_state.selected() {
            Some(i) => {
                if i >= self.threads.len() - 1 { 0 } else { i + 1 }
            }
            None => 0,
        };
        self.thread_list_state.select(Some(i));
    }

    pub fn previous_thread(&mut self) {
        if self.threads.is_empty() {
            return;
        }
        let i = match self.thread_list_state.selected() {
            Some(i) => {
                if i == 0 { self.threads.len() - 1 } else { i - 1 }
            }
            None => 0,
        };
        self.thread_list_state.select(Some(i));
    }
}
