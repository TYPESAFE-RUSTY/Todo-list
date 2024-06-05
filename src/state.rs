#[derive(Debug)]
pub struct State {
    pub mode: Mode,
    pub active: Screen,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    READ, //default mode read mode
    EDIT,
    INSERT,
    GRAB,
}

impl Mode {
    pub fn get(&self) -> &str {
        match self {
            Mode::EDIT => "EDIT MODE",
            Mode::GRAB => "GRAB MODE",
            Mode::INSERT => "INSERT MODE",
            Mode::READ => "READ MODE",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Screen {
    COMPLETED, //Allows only edit and read
    ACTIVE,    //Allows every mode
}

impl Screen {
    pub fn get(&self) -> &str {
        match self {
            Screen::COMPLETED => "COMPLETED TODO'S SECTION",
            Screen::ACTIVE => "INCOMPLETE TODO'S SECTION",
        }
    }
}
