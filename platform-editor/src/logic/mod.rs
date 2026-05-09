use sdl3::{
    keyboard::{Keycode, Scancode},
    mouse::MouseButton,
    rect::Point,
    render::{Canvas, FPoint},
    video::Window,
};

use crate::{AppData, logic::input::InputData};

pub mod input;

pub struct LogicData<'app> {
    pub app_data: &'app mut AppData,
    pub input_data: InputData<'app>,

    /// The current delta time, in nanoseconds.
    pub delta_time: u128,

    /// A reference to the canvas.
    pub canvas: &'app Canvas<Window>,
}

impl LogicData<'_> {
    /// Returns whether the given `MouseButton` is pressed.
    #[must_use]
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.input_data.mouse_state.is_mouse_button_pressed(button)
    }

    /// Returns whether the provided [`Keycode`] is down (freshly pressed).
    #[must_use]
    pub fn is_down(&self, code: &Keycode) -> bool {
        self.input_data.keys_down.contains(code)
    }

    /// Returns whether the provided [`Keycode`] is up (freshly released).
    #[must_use]
    pub fn is_up(&self, code: &Keycode) -> bool {
        self.input_data.keys_up.contains(code)
    }

    /// Returns whether the provided [`Scancode`] is held (currently held).
    #[must_use]
    pub fn is_held(&self, code: Scancode) -> bool {
        self.input_data.keyboard_state.is_scancode_pressed(code)
    }

    /// Returns the current mouse position as an [`FPoint`], adjusted for logical presentation.
    #[must_use]
    pub fn mouse_fpos(&self) -> FPoint {
        self.input_data.mouse_pos
    }

    /// Returns the current mouse position as a [`Point`], adjusted for logical presentation.
    #[must_use]
    pub fn mouse_pos(&self) -> Point {
        let mouse = self.mouse_fpos();
        Point::new(mouse.x as i32, mouse.y as i32)
    }
}

/// A trait to provide a method for a "logical tick", which may or may not
/// affect the component itself.
pub trait Logic {
    /// Performs this object's logic.
    fn run_logic(&mut self, _data: &mut LogicData) {}
}
