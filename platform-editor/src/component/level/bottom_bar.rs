use platform_editor_core::{
    common_util::Vec2f,
    component::{
        Event, Hold,
        level::bottom_bar::{BottomBarBase, BottomBarButtonBase, BottomBarButtonType, TimeState},
    },
    level::state::LevelState,
    screen::Screen,
};
use sdl3::{
    mouse::MouseButton,
    render::{BlendMode, FPoint, FRect},
};

use crate::{
    HEIGHT, WIDTH,
    logic::Logic,
    render::{Render, RenderData},
    textures::TextAlignment,
    util::{FRectExt, IntoFPoint},
};

pub const TEXTURE_CENTER: Vec2f = Vec2f::new(WIDTH as f32 / 2.0, HEIGHT as f32 + 63.0);

pub const TEXTURE_SCALE: f32 = 2.0;
pub const TEXTURE_SIZE: (f32, f32) = (TEXTURE_SCALE * 694.0, TEXTURE_SCALE * 135.0);

fn render_offset(data: &RenderData) -> f32 {
    if data.transitioned_from(Screen::Level) {
        0.0
    } else {
        let t = data.transition_offset(0.9);
        t * t / 2.0
    }
}

impl Render for BottomBarBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let offset = render_offset(data);
        let text_y = TEXTURE_CENTER.y - 95.0 + offset;

        data.canvas.copy(
            &data.textures.level.bottom_bar.base,
            None,
            FRect::from_center(
                TEXTURE_CENTER.add_y(offset).into_fpoint(),
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

pub const BUTTON_BASE_SIZE: f32 = 60.0;
pub const BUTTON_Y: f32 = 687.0;
pub const BUTTON_GAP: f32 = 70.0;

fn button_pos(ty: BottomBarButtonType) -> Vec2f {
    Vec2f::new(
        WIDTH as f32 - (HEIGHT as f32 - BUTTON_Y)
            + match ty {
                BottomBarButtonType::Reset => -2.0 * BUTTON_GAP,
                BottomBarButtonType::Options => -BUTTON_GAP,
                BottomBarButtonType::LevelSelect => 0.0,
            },
        BUTTON_Y,
    )
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

impl Render for BottomBarButtonBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let offset = render_offset(data);
        let alpha_offset = offset * 0.02;

        let texture = match self.ty {
            BottomBarButtonType::Reset => &mut data.textures.level.bottom_bar.reset,
            BottomBarButtonType::Options => &mut data.textures.level.bottom_bar.options,
            BottomBarButtonType::LevelSelect => &mut data.textures.level.bottom_bar.level_select,
        };

        let size = BUTTON_BASE_SIZE * self.scale_multiplier();
        let rect = FRect::from_center(button_pos(self.ty).add_y(offset).into_fpoint(), size, size);

        texture.set_blend_mode(BlendMode::Blend);
        texture.set_alpha_mod((255.0 * (1.0 - alpha_offset)) as u8);

        data.canvas.copy(texture, None, rect)?;

        Ok(())
    }
}

impl Logic for BottomBarButtonBase {
    fn run_logic(&mut self, data: &mut crate::logic::LogicData) {
        if data
            .app_data
            .level_state
            .as_ref()
            .is_some_and(LevelState::is_finished)
        {
            // We don't want to do anything if the level is already done.
            self.update_hold_time(data.delta_time, false);
        } else {
            let hovered = button_pos(self.ty).distance_sqr(data.mouse_pos())
                < (BUTTON_BASE_SIZE * BUTTON_BASE_SIZE) / 4.0;
            self.update_hold_time(data.delta_time, hovered);

            if data.is_mouse_button_up(MouseButton::Left) && hovered {
                match self.ty {
                    BottomBarButtonType::Reset => data.reset_level_call(),
                    BottomBarButtonType::LevelSelect => data.level_select_call(),
                    BottomBarButtonType::Options => {}
                }
            }
        }
    }
}
