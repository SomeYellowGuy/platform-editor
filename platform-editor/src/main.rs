use std::time::{Duration, Instant};

use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;
use sdl3_sys::render::SDL_RendererLogicalPresentation;

use crate::component::Component;
use crate::component::title::TitleBase;
use crate::render::{Background, DrawResult, Render, RenderData};
use crate::{
    images::Images,
    options::{Options, PresentMode},
};

pub mod component;
pub mod images;
pub mod options;
pub mod render;

/// The target width of the window.
pub const WIDTH: u32 = 1280;
/// The target height of the window.
pub const HEIGHT: u32 = 720;

pub type ComponentMap = platform_editor_core::component::ComponentMap<Component>;

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
        let mut components = ComponentMap::new();

        components.insert("title", Component::Title(TitleBase), 0);

        self.update_with_present_mode();
        'running: loop {
            match self.present_mode {
                PresentMode::Capped(max_fps) => {
                    // Calculate the minimum time for a single frame.
                    let min_time = Duration::from_nanos(1_000_000_000 / max_fps as u64);
                    let start: Instant = Instant::now();

                    if self.game_loop(&mut images, &mut components) {
                        break 'running;
                    }

                    let elapsed = start.elapsed();

                    let waited_time = min_time.saturating_sub(elapsed);

                    if !waited_time.is_zero() {
                        std::thread::sleep(waited_time);
                    }
                }
                PresentMode::Uncapped | PresentMode::Vsync => {
                    if self.game_loop(&mut images, &mut components) {
                        break 'running;
                    }
                }
            }
        }
    }

    /// Runs the game loop once.
    ///
    /// Returns `true` if the game should be stopped.
    fn game_loop(&mut self, images: &mut Images, components: &mut ComponentMap) -> bool {
        if let Some(e) = self.render(images, components).err() {
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

    fn render(&mut self, images: &mut Images, components: &mut ComponentMap) -> DrawResult {
        self.canvas.set_draw_color(Color::RGB(10, 10, 10));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(60, 60, 60));
        self.canvas.fill_rect(Rect::new(0, 0, WIDTH, HEIGHT))?;

        let mut data = RenderData::new(self, images);

        Background.render(&mut data)?;

        for (_, component) in components.ascending_iter() {
            component.render(&mut data)?;
        }

        Ok(())
    }
}
