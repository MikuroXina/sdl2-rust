use std::ffi::CStr;

use crate::bind;

pub enum FourButton {
    Up,
    Right,
    Down,
    Left,
}

pub enum Button {
    LeftFour(FourButton),
    RightFour(FourButton),
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
}

impl Button {
    pub(super) fn from_raw(raw: bind::SDL_GameControllerButton) -> Option<Self> {
        use Button::*;
        let val = match raw {
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_A => RightFour(FourButton::Down),
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_B => RightFour(FourButton::Right),
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_X => RightFour(FourButton::Left),
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_Y => RightFour(FourButton::Up),
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_BACK => Back,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_GUIDE => Guide,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_START => Start,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_LEFTSTICK => LeftStick,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_RIGHTSTICK => RightStick,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_LEFTSHOULDER => LeftShoulder,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_DOWN => {
                LeftFour(FourButton::Down)
            }
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_RIGHT => {
                LeftFour(FourButton::Right)
            }
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_LEFT => {
                LeftFour(FourButton::Left)
            }
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_UP => {
                LeftFour(FourButton::Up)
            }
            _ => return None,
        };
        Some(val)
    }

    pub(super) fn as_raw(&self) -> bind::SDL_GameControllerButton {
        use FourButton::*;
        match self {
            Button::LeftFour(Up) => bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_Y,
            Button::LeftFour(Right) => bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_B,
            Button::LeftFour(Down) => bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_A,
            Button::LeftFour(Left) => bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_X,
            Button::RightFour(Up) => bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_UP,
            Button::RightFour(Right) => {
                bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_RIGHT
            }
            Button::RightFour(Down) => {
                bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_DOWN
            }
            Button::RightFour(Left) => {
                bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_LEFT
            }
            Button::Back => bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_BACK,
            Button::Guide => bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_GUIDE,
            Button::Start => bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_START,
            Button::LeftStick => bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_LEFTSTICK,
            Button::RightStick => bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_RIGHTSTICK,
            Button::LeftShoulder => {
                bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_LEFTSHOULDER
            }
            Button::RightShoulder => {
                bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_RIGHTSHOULDER
            }
        }
    }
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c_str =
            unsafe { CStr::from_ptr(bind::SDL_GameControllerGetStringForButton(self.as_raw())) };
        write!(f, "{}", c_str.to_str().unwrap())
    }
}
