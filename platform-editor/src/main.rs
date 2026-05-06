use std::time::{Duration, Instant};

use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::{Point, Rect};
use sdl3::render::Canvas;
use sdl3::video::Window;
use sdl3_sys::render::SDL_RendererLogicalPresentation;

use crate::render::{DrawResult, Render, RenderData};
use crate::{
    images::Images,
    options::{Options, PresentMode},
};

pub mod images;
pub mod options;
pub mod render;

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem
        .window("Platform Editor", 1280, 720)
        .position_centered()
        .resizable()
        .build()
        .expect("could not create window");

    window
        .set_minimum_size(200, 150)
        .expect("could not set minimum window size");

    let mut canvas = window.into_canvas();

    unsafe {
        sdl3_sys::render::SDL_SetRenderLogicalPresentation(
            canvas.raw(),
            WIDTH as i32,
            HEIGHT as i32,
            SDL_RendererLogicalPresentation::LETTERBOX,
        );
    }

    let texture_creator = canvas.texture_creator();
    let images = Images::load(&texture_creator).expect("could not create images");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context
        .event_pump()
        .expect("could not obtain the event pump");

    let mut app = App {
        event_pump,
        canvas,
        options: Options {},
        present_mode: PresentMode::Capped(60),
        start: Instant::now(),
    };

    app.run(images)
}

/// Represents the app.
///
/// Execution of the program is moved here after the program is initialized.
pub struct App {
    event_pump: EventPump,
    canvas: Canvas<Window>,
    #[allow(unused)]
    options: Options,
    start: Instant,
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

    pub fn run(&mut self, mut images: Images) {
        self.update_with_present_mode();
        'running: loop {
            match self.present_mode {
                PresentMode::Capped(max_fps) => {
                    // Calculate the minimum time for a single frame.
                    let min_time = Duration::from_nanos(1_000_000_000 / max_fps as u64);
                    let start: Instant = Instant::now();

                    if self.game_loop(&mut images) {
                        break 'running;
                    }

                    let elapsed = start.elapsed();

                    let waited_time = min_time.saturating_sub(elapsed);

                    if !waited_time.is_zero() {
                        std::thread::sleep(waited_time);
                    }
                }
                PresentMode::Uncapped | PresentMode::Vsync => {
                    if self.game_loop(&mut images) {
                        break 'running;
                    }
                }
            }
        }
    }

    /// Runs the game loop once.
    ///
    /// Returns `true` if the game should be stopped.
    fn game_loop(&mut self, images: &mut Images) -> bool {
        if let Some(e) = self.render(images).err() {
            println!("Error occured diring rendering: {e}");
        }

        for event in self.event_pump.poll_iter() {
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

        self.canvas.present();

        false
    }

    fn render(&mut self, images: &mut Images) -> DrawResult {
        self.canvas.set_draw_color(Color::RGB(10, 10, 10));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(60, 60, 60));
        self.canvas.fill_rect(Rect::new(0, 0, WIDTH, HEIGHT))?;

        // Draw the background.
        Background.render(RenderData::new(self, images))?;

        Ok(())
    }
}

/// The game's background, which is always drawn.
struct Background;

impl Render for Background {
    fn render(&self, mut data: RenderData) -> DrawResult {
        let oscillation_angle = data.start.elapsed().as_nanos() as f64 / 1_000_000_000.0 * 2.1;

        data.images.strip.set_alpha_mod(170);

        for i in 0..7 {
            Self::draw_strip(oscillation_angle, i, &mut data)?;
        }

        Ok(())
    }
}

impl Background {
    fn draw_strip(angle: f64, i: usize, data: &mut RenderData) -> DrawResult {
        const SIZE: u32 = 1060;
        const SPACING: i32 = 400;
        const ANGLE_SPEED: f64 = 100.0;

        let offset =
            (i as i32 - 3) * SPACING + ((angle * ANGLE_SPEED) as i32 % SPACING - SPACING / 2);
        let center = Point::new(WIDTH as i32 / 2 + offset, HEIGHT as i32 / 2 + 10);

        data.canvas.copy_ex(
            &data.images.strip,
            None,
            Rect::from_center(center, SIZE, SIZE),
            6.0_f64 * angle.sin() - 3.0_f64,
            Some(center.into()),
            false,
            false,
        )?;

        Ok(())
    }
}
