use crate::state::{Mode, Screen, State};
use crate::ui;
use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use std::{fs, io};
pub struct App {
    // State of the app
    pub state: State,
    // Data Store for the app
    active_todos: Vec<String>,
    completed_todos: Vec<String>,
    // for responsive app
    pub active_screen_length: u16,
    pub completed_screen_length: u16,
    pub info_length: u16,
    // currently selected todos
    pub active_selected: usize,
    pub completed_selected: usize,
    // Constant INFO
    pub info_texts: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            state: State {
                mode: Mode::READ,
                active: Screen::ACTIVE,
            },
            active_todos: Vec::new(),
            completed_todos:  Vec::new(),
            active_screen_length: 0,
            completed_screen_length: 0,
            info_length:0,
            active_selected:0,
            completed_selected:0,
            info_texts: vec![
                String::from("Click TAB to switch Tabs\nClick i to Insert \nClick e to Edit\nClick g to enter Grab mode\nClick q to Exit"),
                String::from("Click TAB to switch Tabs\nClick g to enter Grab mode\nClick q to Exit")
            ]
        };
        app.load();
        app
    }
}

impl App {
    pub fn start<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<bool> {
        loop {
            terminal.draw(|frame| ui(frame, self))?;

            // Handle events

            // set states
            if let Event::Key(key) = event::read()? {
                // Skip events where keys are not pressed
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                // State is set in following way:
                // check what button user clicked
                // change state (set mode)

                // Event is handled in following pattern
                // Check which mode user is in
                // Check which screen user is in
                // Check what button user pressed

                // handle user input , in read mode (vim motions to go up and down)
                match self.state.mode {
                    Mode::READ => match self.state.active {
                        Screen::ACTIVE => match key.code {
                            KeyCode::Char('j') => {
                                let length = self.active_todos.len();
                                if length == 0 {
                                    continue;
                                }
                                self.active_selected = (self.active_selected + 1) % length;
                            }
                            KeyCode::Char('k') => {
                                let length = self.active_todos.len();
                                if length == 0 {
                                    continue;
                                }
                                if self.active_selected == 0 {
                                    self.active_selected = length;
                                }
                                self.active_selected = self.active_selected - 1;
                            }
                            KeyCode::Char('m') => {
                                if self.active_todos.len() == 0 {
                                    continue;
                                }
                                let val: String = self.active_todos.remove(self.active_selected);
                                if self.active_todos.len() != 0 {
                                    self.active_selected =
                                        self.active_selected % self.active_todos.len();
                                }
                                let mut temp: Vec<String> = Vec::from([val]);
                                temp.extend(self.completed_todos.clone());
                                self.completed_todos = temp;
                                self.completed_selected += 1;
                            }
                            _ => {}
                        },
                        Screen::COMPLETED => match key.code {
                            KeyCode::Char('j') => {
                                let length = self.completed_todos.len();
                                if length == 0 {
                                    continue;
                                }
                                self.completed_selected = (self.completed_selected + 1) % length;
                            }
                            KeyCode::Char('k') => {
                                let length = self.completed_todos.len();
                                if length == 0 {
                                    continue;
                                }
                                if self.completed_selected == 0 {
                                    self.completed_selected = length;
                                }
                                self.completed_selected = self.completed_selected - 1;
                            }
                            _ => {}
                        },
                    },
                    Mode::EDIT => match self.state.active {
                        Screen::ACTIVE => match key.code {
                            KeyCode::Esc | KeyCode::Enter => {
                                if self.active_todos[self.active_selected].trim().len() == 0 {
                                    self.active_todos.remove(self.active_selected);
                                    if self.active_todos.len() == 0 {
                                        continue;
                                    } else {
                                        self.active_selected =
                                            self.active_selected + 1 % self.active_todos.len();
                                    }
                                }
                                self.state.mode = Mode::READ
                            }
                            KeyCode::Char(item) => {
                                self.active_todos[self.active_selected].push(item);
                                continue;
                            }
                            KeyCode::Backspace => {
                                self.active_todos[self.active_selected].pop();
                                continue;
                            }
                            _ => {}
                        },
                        Screen::COMPLETED => {}
                    },
                    Mode::INSERT => {
                        match key.code {
                            KeyCode::Esc => {
                                // Cleaning empty string when leaving insert mode
                                if self.active_todos[self.active_todos.len() - 1].trim().len() == 0
                                {
                                    self.active_todos.pop();
                                    let _ = self.active_selected.checked_sub(1);
                                }
                                self.state.mode = Mode::READ;
                                continue;
                            }
                            KeyCode::Char(a) => {
                                let index = self.active_todos.len() - 1;
                                self.active_todos[index].push(a);
                                self.active_selected = self.active_todos.len() - 1;
                                continue;
                            }
                            KeyCode::Backspace => {
                                let index = self.active_todos.len() - 1;
                                self.active_todos[index].pop();
                                self.active_selected = self.active_todos.len() - 1;
                                continue;
                            }
                            KeyCode::Enter => {
                                self.active_todos.push(String::new());
                                self.active_selected += 1;
                                continue;
                            }
                            _ => continue,
                        }
                    }
                    Mode::GRAB => match self.state.active {
                        Screen::ACTIVE => match key.code {
                            KeyCode::Esc | KeyCode::Enter => {
                                self.state.mode = Mode::READ;
                                continue;
                            }
                            KeyCode::Char('j') => {
                                let index = (self.active_selected + 1) % self.active_todos.len();
                                self.active_todos.swap(self.active_selected, index);
                                self.active_selected = index;
                            }
                            KeyCode::Char('k') => {
                                let index: usize;
                                if self.active_selected == 0 {
                                    index = self.active_todos.len() - 1;
                                } else {
                                    index = self.active_selected - 1;
                                }
                                self.active_todos.swap(self.active_selected, index);
                                self.active_selected = index;
                            }
                            _ => {}
                        },
                        Screen::COMPLETED => match key.code {
                            KeyCode::Esc | KeyCode::Enter => {
                                self.state.mode = Mode::READ;
                                continue;
                            }
                            KeyCode::Char('j') => {
                                let index =
                                    (self.completed_selected + 1) % self.completed_todos.len();
                                self.completed_todos.swap(self.completed_selected, index);
                                self.completed_selected = index;
                            }
                            KeyCode::Char('k') => {
                                let index: usize;
                                if self.completed_selected == 0 {
                                    index = self.completed_todos.len() - 1;
                                } else {
                                    index = self.completed_selected - 1;
                                }
                                self.completed_todos.swap(self.completed_selected, index);
                                self.completed_selected = index;
                            }
                            _ => {}
                        },
                    },
                }

                // change screen
                if key.code == KeyCode::Tab {
                    if self.state.active == Screen::ACTIVE {
                        self.state.active = Screen::COMPLETED;
                        continue;
                    } else {
                        self.state.active = Screen::ACTIVE;
                        continue;
                    }
                }

                // Enter INSERT mode only if user presses i in active screen
                if key.code == KeyCode::Char('i') && self.state.active == Screen::ACTIVE {
                    self.state.mode = Mode::INSERT;
                    // initalize empty string to insert
                    self.active_todos.push(String::new());
                    self.active_selected = self.active_todos.len() - 1;
                    continue;
                }

                // below modes are allowed for both screens
                if key.code == KeyCode::Char('g') {
                    match self.state.active {
                        Screen::ACTIVE => {
                            if self.active_todos.len() < 2 {
                                continue;
                            } else {
                                self.state.mode = Mode::GRAB;
                                continue;
                            }
                        }
                        Screen::COMPLETED => {
                            if self.completed_todos.len() < 2 {
                                continue;
                            } else {
                                self.state.mode = Mode::GRAB;
                                continue;
                            }
                        }
                    }
                }

                if key.code == KeyCode::Char('e') {
                    match self.state.active {
                        Screen::ACTIVE => {
                            if self.active_todos.len() == 0 {
                                continue;
                            } else {
                                self.state.mode = Mode::EDIT;
                                continue;
                            }
                        }
                        Screen::COMPLETED => {
                            if self.completed_todos.len() == 0 {
                                continue;
                            } else {
                                self.state.mode = Mode::EDIT;
                                continue;
                            }
                        }
                    }
                }

                // exit if q
                if key.code == KeyCode::Char('q') {
                    self.save();
                    return Ok(true);
                }
            }
        }
    }
    pub fn update_len(&mut self, c: u16, a: u16, i: u16) {
        self.completed_screen_length = c;
        self.active_screen_length = a;
        self.info_length = i;
    }

    pub fn show_active_todos(&mut self) -> Text {
        let mut text = Text::from(vec![]);
        let mut index = 0;

        for span in &self.active_todos {
            if index == self.active_selected {
                let line = Line::from(("[ ] ".to_owned() + span + "\n").white());
                text.push_line(line);
            } else {
                let line = Line::from("[ ] ".to_owned() + span + "\n").cyan();
                text.push_line(line);
            }
            index += 1;
        }
        text
    }

    pub fn show_completed_todos(&self) -> Text {
        let mut text = Text::from(vec![]);
        let mut index = 0;

        for span in &self.completed_todos {
            if index == self.completed_selected {
                let line = Line::from(("[x] ".to_owned() + span + "\n").white());
                text.push_line(line);
            } else {
                let line = Line::from("[x] ".to_owned() + span + "\n").cyan();
                text.push_line(line);
            }
            index += 1;
        }
        text
    }

    pub fn deserialize(&self) -> String {
        let mut response = String::new();
        // serialize completed todos to follow md syntax
        self.completed_todos
            .iter()
            .rev()
            .for_each(|elem| response += format!("- [x] {}\n", elem).as_str());
        // serialize incompleted todos to follow md syntax
        self.active_todos
            .iter()
            .for_each(|elem| response += format!("- [ ] {}\n", elem).as_str());
        response
    }

    pub fn serialize(&mut self, text: String) {
        let data: Vec<&str> = text.split('\n').collect();
        for elem in data {
            if elem.trim().len() == 0 {
                continue;
            }
            let m_elem: Vec<&str> = elem.split("- [").collect();
            let content = m_elem[1].to_string();
            if content.starts_with("x") {
                self.set_completed_todos(content[3..].to_string());
            } else {
                self.set_active_todos(content[3..].to_string())
            }
        }
    }

    // reading file directly to variables as i know my readme files are not going to be huge
    pub fn save(&self) {
        let file: String;
        match fs::read_to_string("readme.md") {
            Ok(content) => {
                file = content;
            }
            Err(_) => {
                let _ = fs::File::create("readme.md");
                file = "".to_string()
            }
        };
        let mut content = "## Todo\n".to_string() + &self.deserialize();
        match file.find("## Todo") {
            Some(index) => {
                let res: Vec<&str> = file[..index].split("## Todo\n\n").collect();
                content = res[0].to_string() + &content;
                let _ = fs::write("readme.md", content);
            }
            None => {
                content = file + &content;
                let _ = fs::write("readme.md", content);
            }
        };
    }

    pub fn load(&mut self) {
        let file = fs::read_to_string("readme.md").unwrap_or("".to_string());

        match file.find("## Todo") {
            Some(index) => {
                let res: Vec<&str> = file[index..].split("## Todo\n").collect();
                self.serialize(res[1].to_string());
            }
            None => {}
        };
    }

    pub fn set_active_todos(&mut self, item: String) {
        self.active_todos.push(item);
    }

    pub fn set_completed_todos(&mut self, item: String) {
        self.completed_todos.push(item);
    }
}

mod test {
    #[allow(unused)]
    use super::App;

    #[test]
    fn deserializer() {
        let mut state = App::new();
        state.active_todos = Vec::from([String::from("class")]);
        state.completed_todos = Vec::from([String::from("for")]);

        let result = String::from("- [x] for\n- [ ] class\n");
        assert_eq!(state.deserialize(), result);
    }

    #[test]
    fn serializer() {
        let mut state = App::new();
        let result = String::from("- [x] for\n- [ ] class\n");
        state.serialize(result);

        let mut reference = App::new();
        reference.active_todos = Vec::from([String::from("class")]);
        reference.completed_todos = Vec::from([String::from("for")]);

        assert_eq!(state.active_todos, reference.active_todos);
        assert_eq!(state.completed_todos, reference.completed_todos);
    }
}
