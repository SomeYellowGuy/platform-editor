use sdl3::{
    keyboard::{KeyboardState, Keycode},
    mouse::{MouseButton, MouseState},
};

/// Keyboard and mouse state for the current frame.
pub struct InputData<'p> {
    /// Keys that have just been pressed in the current frame.
    pub keys_down: Vec<Keycode>,
    /// Keys that have just been released in the current frame.
    pub keys_up: Vec<Keycode>,
    /// The keyboard state in the current frame.
    pub keyboard_state: KeyboardState<'p>,

    /// The mouse state in the current frame.
    pub mouse_state: MouseState,
    /// The mouse mouse_events in the current frame.
    pub mouse_events: Vec<MouseEvent>,
}

pub enum MouseEvent {
    Up(MouseButton),
    Down(MouseButton),
}
