use platform_editor_core::{
    common_util::Vec2f,
    component::{
        Event,
        level::bottom_bar::{BottomBarBase, TimeState},
    },
    screen::Screen,
};
use sdl3::render::{FPoint, FRect};

use crate::{
    HEIGHT, WIDTH,
    logic::Logic,
    render::Render,
    textures::TextAlignment,
    util::{FRectExt, IntoFPoint},
};

pub const TEXTURE_CENTER: Vec2f = Vec2f::new(WIDTH as f32 / 2.0, HEIGHT as f32 + 63.0);

pub const TEXTURE_SCALE: f32 = 2.0;
pub const TEXTURE_SIZE: (f32, f32) = (TEXTURE_SCALE * 694.0, TEXTURE_SCALE * 135.0);

impl Render for BottomBarBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let offset = if data.transitioned_from(Screen::Level) {
            0.0
        } else {
            let t = data.transition_offset(0.9);
            t * t / 2.0
        };
        let text_y = TEXTURE_CENTER.y - 95.0 + offset;

        data.canvas.copy(
            &data.textures.level.bottom_bar.base,
            None,
            FRect::from_center(
                (TEXTURE_CENTER + Vec2f::new(0.0, offset)).into_fpoint(),
                TEXTURE_SIZE.0,
                TEXTURE_SIZE.1,
            ),
        )?;

        // Draw the level text.
        data.textures.level.bottom_bar.level_text.update(
            data.canvas,
            data.font,
            format!("Level {}", data.extracted_data.playing_level + 1),
        )?;
        data.textures.level.bottom_bar.level_text.draw(
            data.canvas,
            TextAlignment::Left,
            FPoint::new(20.0, text_y),
            0.9,
        )?;

        // Draw the time icon and text.
        const TIME_ICON_SIZE: f32 = 50.0;
        const TIME_TEXT_X: f32 = 450.0;
        data.textures.icons.time.set_alpha_mod(u8::MAX);
        data.canvas.copy(
            &data.textures.icons.time,
            None,
            FRect::from_center(
                FPoint::new(TIME_TEXT_X - TIME_ICON_SIZE / 2.0 - 10.0, text_y),
                TIME_ICON_SIZE,
                TIME_ICON_SIZE,
            ),
        )?;

        let time = self.state.time_counter();
        data.textures.level.bottom_bar.time_text.update(
            data.canvas,
            data.font,
            time.to_string(),
        )?;
        data.textures.level.bottom_bar.time_text.draw(
            data.canvas,
            TextAlignment::Left,
            FPoint::new(TIME_TEXT_X, text_y),
            1.0,
        )?;

        Ok(())
    }
}

impl Logic for BottomBarBase {
    fn handles_events(&self) -> bool {
        true
    }

    fn handle(&mut self, event: &Event) {
        match event {
            Event::LevelGo(instant) => self.state = TimeState::Active(*instant),
            Event::LevelFinish(time) => self.state = TimeState::Finished(*time),
        }
    }
}
