use std::time::{Duration, Instant};

use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::options::{Options, PresentMode};

pub mod options;

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Platform Editor", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .expect("Could not create window");

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context
        .event_pump()
        .expect("Could not obtain the event pump");

    let mut app = App {
        event_pump,
        canvas,
        options: Options {},
        present_mode: PresentMode::Capped(60),
    };

    app.run()
}

/// Represents the app.
///
/// Execution of the program is moved here after the program is initialized.
pub struct App {
    event_pump: EventPump,
    canvas: Canvas<Window>,
    options: Options,
    // Although an option, this is handled separately by the app.
    present_mode: PresentMode,
}

impl App {
    /// Sets the [`PresentMode`] of the app.
    pub fn set_present_mode(&mut self, mode: PresentMode) {
        self.present_mode = mode;
        self.update_with_present_mode();
    }

    fn update_with_present_mode(&mut self) {
        // Update VSync.
        let n = if matches!(self.present_mode, PresentMode::Vsync) {
            1
        } else {
            0
        };

        unsafe {
            sdl3_sys::render::SDL_SetRenderVSync(self.canvas.raw(), n);
        };
    }

    pub fn run(&mut self) {
        self.update_with_present_mode();
        'running: loop {
            match self.present_mode {
                PresentMode::Capped(max_fps) => {
                    // Calculate the minimum time for a single frame.
                    let min_time = Duration::from_nanos(1_000_000_000 / max_fps as u64);
                    let start: Instant = Instant::now();

                    if game_loop(self) {
                        break 'running;
                    }

                    let elapsed = start.elapsed();

                    let waited_time = min_time.saturating_sub(elapsed);

                    if !waited_time.is_zero() {
                        std::thread::sleep(waited_time);
                    }
                }
                PresentMode::Uncapped | PresentMode::Vsync => {
                    if game_loop(self) {
                        break 'running;
                    }
                }
            }
        }
    }
}

/// Runs the game loop.
///
/// Returns `true` if the game should be stopped.
fn game_loop(app: &mut App) -> bool {
    app.canvas.set_draw_color(Color::RGB(0, 64, 255 - 0));
    app.canvas.clear();

    for event in app.event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return true;
            }
            _ => {}
        }
    }

    // The rest of the game loop goes here...

    app.canvas.present();

    false
}
