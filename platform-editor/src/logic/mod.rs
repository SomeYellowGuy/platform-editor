use platform_editor_core::{
    common_util::Vec2f,
    component::Event,
    screen::{Screen, TransitionCall, TransitionData},
};
use sdl3::{
    keyboard::{Keycode, Scancode},
    mouse::MouseButton,
    render::Canvas,
    video::Window,
};

use crate::{
    AppData, QueuedComponent, QueuedData,
    logic::input::{InputData, MouseEvent},
};

pub mod input;

pub struct LogicData<'app> {
    pub app_data: &'app mut AppData,
    pub input_data: InputData<'app>,

    /// The current delta time, in nanoseconds.
    pub delta_time: u128,

    /// A reference to the canvas.
    pub canvas: &'app Canvas<Window>,

    /// The current transition call.
    pub transition_call: TransitionCall,

    /// Any queued things to handle.
    pub queued: QueuedData,
}

impl LogicData<'_> {
    pub fn set_transition_call(&mut self, call: TransitionCall) {
        self.transition_call = call;
    }

    /// A type of transition specifically to reset a level.
    pub fn reset_level_call(&mut self) {
        self.set_transition_call(TransitionCall::Start(TransitionData::new(
            0,
            300_000_000,
            Screen::Level,
        )));
    }

    /// Returns whether the given `MouseButton` is held.
    #[must_use]
    pub fn is_mouse_button_held(&self, button: MouseButton) -> bool {
        self.input_data.mouse_state.is_mouse_button_pressed(button)
    }

    /// Returns whether the given `MouseButton` is down (freshly pressed).
    #[must_use]
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.input_data
            .mouse_events
            .iter()
            .any(|e| matches!(e, MouseEvent::Down(b) if *b == button))
    }

    /// Returns whether the given `MouseButton` is up (freshly released).
    #[must_use]
    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        self.input_data
            .mouse_events
            .iter()
            .any(|e| matches!(e, MouseEvent::Up(b) if *b == button))
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

    /// Returns the current mouse position as a [`Vec2f`], adjusted for logical presentation.
    #[must_use]
    pub fn mouse_pos(&self) -> Vec2f {
        Vec2f::new(self.input_data.mouse_pos.x, self.input_data.mouse_pos.y)
    }

    /// Adds an event to queue, handled after the current logical tick.
    pub fn queue_event(&mut self, event: Event) {
        self.queued.add_event(event);
    }

    /// Adds a component to queue to add after the current logical tick.
    pub fn queue_component(&mut self, component: QueuedComponent) {
        self.queued.add_component(component);
    }
}

/// A trait to provide a method for a "logical tick", which may or may not
/// affect the component itself.
pub trait Logic {
    /// Performs this object's logic.
    fn run_logic(&mut self, _data: &mut LogicData) {}

    /// Whether this component handles events.
    fn handles_events(&self) -> bool {
        false
    }

    /// Handle an event (if this component handles events at the time of event handling).
    fn handle(&mut self, _event: &Event) {}
}
