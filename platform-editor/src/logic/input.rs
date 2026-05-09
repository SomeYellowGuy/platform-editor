use sdl3::{
    keyboard::{KeyboardState, Keycode},
    mouse::{MouseButton, MouseState},
    render::{Canvas, FPoint},
    video::Window,
};

/// Keyboard and mouse state for the current frame.
pub struct InputData<'p> {
    /// Keys that have just been pressed in the current frame.
    pub keys_down: Vec<Keycode>,
    /// Keys that have just been released in the current frame.
    pub keys_up: Vec<Keycode>,
    /// The keyboard state in the current frame.
    pub keyboard_state: KeyboardState<'p>,

    /// The mouse position (adjusted for logical presentation) in the current frame.
    pub mouse_pos: FPoint,
    /// The mouse events in the current frame.
    pub mouse_events: Vec<MouseEvent>,
    /// The mouse state in the current frame.
    pub mouse_state: MouseState,
}

pub fn converted_pos(pos: FPoint, canvas: &Canvas<Window>) -> FPoint {
    let mut x = 0.0;
    let mut y = 0.0;
    unsafe {
        sdl3_sys::render::SDL_RenderCoordinatesFromWindow(
            canvas.raw(),
            pos.x,
            pos.y,
            &mut x,
            &mut y,
        );
    }
    FPoint::new(x, y)
}

pub enum MouseEvent {
    Up(MouseButton),
    Down(MouseButton),
}
