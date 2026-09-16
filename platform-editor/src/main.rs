use std::time::{Duration, Instant};

use platform_editor_core::LevelSave;
use platform_editor_core::component::ComponentMapQueryType;
use platform_editor_core::options::Options;
use platform_editor_core::screen::{Screen, ScreenManager, TransitionCall};
use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::{BlendMode, Canvas, FPoint};
use sdl3::ttf::Font;
use sdl3::video::Window;
use sdl3_sys::render::SDL_RendererLogicalPresentation;

use crate::component::Component;
use crate::logic::input::{InputData, MouseEvent, converted_pos};
use crate::logic::{Logic, LogicData};
use crate::render::{Background, DrawResult, Render, RenderData};
use crate::textures::Textures;

pub mod component;
pub mod logic;
pub mod render;
pub mod screen;
pub mod textures;
pub mod util;

/// The target width of the window.
pub const WIDTH: u32 = 1280;
/// The target height of the window.
pub const HEIGHT: u32 = 720;

/// A priority for components with no logic.
pub const NO_LOGIC_PRIORITY: i32 = i32::MIN;

pub type ComponentMap = platform_editor_core::component::ComponentMap<Component>;
pub type QueuedComponent = platform_editor_core::component::QueuedComponent<Component>;
pub type QueuedData = platform_editor_core::QueuedData<Component>;

pub type AppData = platform_editor_core::AppData<ExtraAppData>;

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

    let context = sdl3::ttf::init().expect("could not initialize TTF context");
    let font = context
        .load_font("assets/font.ttf", 48.0)
        .expect("could not load font");

    let images = Textures::load(&texture_creator, &font).expect("could not create images");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context
        .event_pump()
        .expect("could not obtain the event pump");

    let mut app = App { event_pump, canvas };

    app.run(images, font)
}

/// Represents the app.
///
/// Execution of the program is moved here after the program is initialized.
pub struct App {
    event_pump: EventPump,
    canvas: Canvas<Window>,
}

/// A mode deciding the maximum FPS or VSync.
#[derive(Debug, Copy, Clone)]
pub enum PresentMode {
    /// No VSync, and the frame rate is capped to the provided number.
    Capped(u32),
    /// No VSync, and the frame rate is uncapped.
    Uncapped,
    /// VSync, so the frame rate is matched to that of the monitor.
    Vsync,
}

pub struct ExtraAppData {
    // Although an option, this is handled separately by the app.
    present_mode: PresentMode,
    /// The last Y mouse position, if any, which is used for scrolling in the level select.
    last_y_mouse_pos: Option<f32>,
}

impl Default for ExtraAppData {
    fn default() -> Self {
        Self {
            present_mode: PresentMode::Capped(60),
            last_y_mouse_pos: None,
        }
    }
}

pub struct ExtractedData {
    pub y_scroll: f32,
}

impl App {
    /// Sets the [`PresentMode`] of the app.
    pub fn set_present_mode(&mut self, data: &mut AppData, mode: PresentMode) {
        data.extra.present_mode = mode;
        self.update_with_present_mode(data);
    }

    fn update_with_present_mode(&mut self, data: &mut AppData) {
        // Update VSync.
        let n = if matches!(data.extra.present_mode, PresentMode::Vsync) {
            1
        } else {
            0
        };

        unsafe {
            sdl3_sys::render::SDL_SetRenderVSync(self.canvas.raw(), n);
        };
    }

    pub fn run(&mut self, mut textures: Textures, font: Font<'static>) {
        let mut data: AppData = AppData {
            options: Options::default(),
            start: Instant::now(),
            extra: ExtraAppData::default(),
            level_select_scroll: screen::STARTING_LEVEL_SELECT_SCROLL,
            level_select_scroll_velocity: 0.0,
            level: LevelSave::new(),
        };

        let mut components = ComponentMap::new();

        self.update_with_present_mode(&mut data);

        let font_ref = Box::leak(Box::new(font));
        let mut last_instant = Instant::now();

        let mut screen_manager = ScreenManager::new();
        screen::on_enter(screen_manager.screen, &mut components, None);

        'running: loop {
            let start: Instant = Instant::now();
            let delta = (start - last_instant).as_nanos();
            match data.extra.present_mode {
                PresentMode::Capped(max_fps) => {
                    // Calculate the minimum time for a single frame.
                    let min_time = Duration::from_nanos(1_000_000_000 / max_fps as u64);

                    if self.game_tick(
                        &mut textures,
                        font_ref,
                        &mut components,
                        &mut data,
                        &mut screen_manager,
                        delta,
                    ) {
                        break 'running;
                    }

                    let elapsed = start.elapsed();

                    let waited_time = min_time.saturating_sub(elapsed);

                    if !waited_time.is_zero() {
                        std::thread::sleep(waited_time);
                    }
                }
                PresentMode::Uncapped | PresentMode::Vsync => {
                    if self.game_tick(
                        &mut textures,
                        font_ref,
                        &mut components,
                        &mut data,
                        &mut screen_manager,
                        delta,
                    ) {
                        break 'running;
                    }
                }
            }
            last_instant = start
        }
    }

    /// Runs the game loop once.
    ///
    /// Returns `true` if the game should be stopped.
    fn game_tick(
        &mut self,
        images: &mut Textures,
        font: &'static Font,
        components: &mut ComponentMap,
        app_data: &mut AppData,
        transition_manager: &mut ScreenManager,
        delta_time: u128,
    ) -> bool {
        // Logic

        let mut keys_up = Vec::new();
        let mut keys_down = Vec::new();
        let mut mouse_button_events = Vec::new();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return true;
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    keys_up.push(key);
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    keys_down.push(key);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    mouse_button_events.push(MouseEvent::Up(mouse_btn));
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    mouse_button_events.push(MouseEvent::Down(mouse_btn));
                }
                _ => {}
            }
        }

        let mouse_state = self.event_pump.mouse_state();
        let keyboard_state = self.event_pump.keyboard_state();

        let mut logic_data = LogicData {
            app_data,
            input_data: InputData {
                keys_down,
                keys_up,
                keyboard_state,

                mouse_pos: converted_pos(
                    FPoint::new(mouse_state.x(), mouse_state.y()),
                    &self.canvas,
                ),
                mouse_events: mouse_button_events,
                mouse_state,
            },
            delta_time,
            canvas: &self.canvas,
            transition_call: TransitionCall::None,
            queued: QueuedData::new(),
        };

        let call = if !transition_manager.is_transitioning() {
            self.run_game_logic(components, &mut logic_data, transition_manager.screen);
            logic_data.transition_call
        } else {
            TransitionCall::None
        };

        if let Some((old_screen, new_screen)) = transition_manager.tick(call) {
            screen::on_exit(old_screen, components);
            screen::on_enter(new_screen, components, Some(&mut logic_data));
        }

        // Rendering

        let extracted = ExtractedData {
            y_scroll: app_data.level_select_scroll,
        };

        if let Some(e) = self
            .render(
                app_data,
                images,
                font,
                components,
                transition_manager.time(),
                &extracted,
            )
            .err()
        {
            println!("Error occured during rendering: {e}");
        }

        self.canvas.present();

        false
    }

    fn run_game_logic(
        &self,
        components: &mut ComponentMap,
        logic_data: &mut LogicData<'_>,
        screen: Screen,
    ) {
        screen::tick(screen, logic_data);

        components.descending_iter_mut(ComponentMapQueryType::Logic, |_, component| {
            component.run_logic(logic_data);
        });

        // Handle events and newly-added components.
        let (events, new_components) = logic_data.queued.extract();

        for (_, component) in &mut components.into_iter() {
            if component.handles_events() {
                for event in &events {
                    component.handle(event);
                }
            }
        }

        let new_components_is_empty = new_components.is_empty();
        for queued in new_components {
            components.insert_queued(queued);
        }
        if !new_components_is_empty {
            components.update_cache();
        }
    }

    fn render(
        &mut self,
        data: &AppData,
        images: &mut Textures,
        font: &'static Font,
        components: &mut ComponentMap,
        transition_time: Option<u64>,
        extracted: &ExtractedData,
    ) -> DrawResult {
        self.canvas.set_draw_color(Color::RGB(10, 10, 10));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(60, 60, 60));
        self.canvas.fill_rect(Rect::new(0, 0, WIDTH, HEIGHT))?;

        let mut data = RenderData::new(self, data, images, font, transition_time, extracted);

        data.canvas.set_blend_mode(BlendMode::None);
        Background.render(&mut data)?;

        for (_, component) in components.ascending_iter(ComponentMapQueryType::Render) {
            data.canvas.set_blend_mode(BlendMode::None);
            component.render(&mut data)?;
        }

        Ok(())
    }
}
