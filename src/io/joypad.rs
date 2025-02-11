use crate::errors::JoypadError;

#[derive(Debug)]
pub enum Action {
    Start,
    Select,
    A,
    B,
}

#[derive(Debug)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

#[derive(Debug)]
pub enum JoypadBank {
    SelectButtons(Action),
    SelectDPad(Direction),
    AllReleased,
}

pub struct Joypad {
    register: u8,
}

impl Joypad {
    fn active(&mut self) -> std::result::Result<JoypadBank, JoypadError> {
        match self.register >> 4 {
            1 => {
                let direction = match self.register & 0xf {
                    1 => JoypadBank::SelectDPad(Direction::Right),
                    2 => JoypadBank::SelectDPad(Direction::Left),
                    3 => JoypadBank::SelectDPad(Direction::Up),
                    4 => JoypadBank::SelectDPad(Direction::Down),
                    _ => {
                        return Err(JoypadError::InvalidRegisterValue(
                            self.register,
                            "DPAD".into(),
                        ));
                    }
                };
                Ok(direction)
            }
            2 => {
                let button = match self.register & 0xf {
                    1 => JoypadBank::SelectButtons(Action::A),
                    2 => JoypadBank::SelectButtons(Action::B),
                    3 => JoypadBank::SelectButtons(Action::Select),
                    4 => JoypadBank::SelectButtons(Action::Start),
                    _ => {
                        return Err(JoypadError::InvalidRegisterValue(
                            self.register,
                            "BUTTONS".into(),
                        ));
                    }
                };
                Ok(button)
            }
            _ => Ok(JoypadBank::AllReleased),
        }
    }
}
