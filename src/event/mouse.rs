use crate::bind;
use crate::geo::Point;

pub mod cursor;
pub mod relative;

#[derive(Debug, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

impl MouseButton {
    pub(crate) fn from_bits(bits: u8) -> Option<Self> {
        use MouseButton::*;
        Some(match bits {
            1 => Left,
            2 => Middle,
            3 => Right,
            4 => X1,
            5 => X2,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
pub enum MouseEvent {
    Motion(MouseMotionEvent),
    Button(MouseButtonEvent),
    Wheel(MouseWheelEvent),
}

#[derive(Debug, Clone)]
pub struct MouseMotionEvent {
    pub timestamp: u32,
    pub window_id: u32,
    pub mouse_id: u32,
    pub button: Option<MouseButton>,
    pub pos: Point,
    pub move_amount: Point,
}

impl From<bind::SDL_MouseMotionEvent> for MouseMotionEvent {
    fn from(raw: bind::SDL_MouseMotionEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            mouse_id: raw.which,
            button: MouseButton::from_bits(raw.state as u8),
            pos: Point { x: raw.x, y: raw.y },
            move_amount: Point {
                x: raw.xrel,
                y: raw.yrel,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct MouseButtonEvent {
    pub timestamp: u32,
    pub window_id: u32,
    pub mouse_id: u32,
    pub button: Option<MouseButton>,
    pub is_pressed: bool,
    pub clicks: u8,
    pub pos: Point,
}

impl From<bind::SDL_MouseButtonEvent> for MouseButtonEvent {
    fn from(raw: bind::SDL_MouseButtonEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            mouse_id: raw.which,
            button: MouseButton::from_bits(raw.button),
            is_pressed: raw.state as u32 == bind::SDL_PRESSED,
            clicks: raw.clicks,
            pos: Point { x: raw.x, y: raw.y },
        }
    }
}

#[derive(Debug, Clone)]
pub struct MouseWheelEvent {
    pub timestamp: u32,
    pub window_id: u32,
    pub mouse_id: u32,
    pub scroll_amount: Point,
    pub is_flipped: bool,
}

impl From<bind::SDL_MouseWheelEvent> for MouseWheelEvent {
    fn from(raw: bind::SDL_MouseWheelEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            mouse_id: raw.which,
            scroll_amount: Point { x: raw.x, y: raw.y },
            is_flipped: raw.direction == bind::SDL_MouseWheelDirection_SDL_MOUSEWHEEL_FLIPPED,
        }
    }
}
