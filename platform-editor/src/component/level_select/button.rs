use platform_editor_core::{
    common_util::{self, Vec2f},
    component::{Hold, level_select::button::LevelSelectButtonBase},
    level::scratch::levels::LEVEL_COUNT,
    screen::{Screen, TransitionCall, TransitionData},
};
use sdl3::{mouse::MouseButton, render::FRect};

use crate::{
    HEIGHT,
    component::level_select::header::HEADER_HEIGHT,
    logic::Logic,
    render::Render,
    util::{DualImageDimensions, FRectExt, IntoFPoint},
};

pub const SPACING: f32 = 220.0;
pub const SIDE: f32 = 190.0;

pub const LEVELS_PER_ROW: usize = 4;

fn normal_pos(base: &LevelSelectButtonBase, scroll: f32, last_level: bool) -> Vec2f {
    let horizontal = (base.level % 4) as f32 + if last_level { 0.5 } else { 0.0 };
    let vertical = (base.level / 4) as f32;

    Vec2f::new(
        crate::WIDTH as f32 / 2.0 + SPACING * (horizontal - (LEVELS_PER_ROW - 1) as f32 / 2.0),
        200.0 + SPACING * vertical + scroll,
    )
}

const STAR_OFFSETS: [((f32, f32), f64); 3] = [
    ((-65.0, 42.0), 20.0),
    ((0.0, 60.0), 0.0),
    ((65.0, 42.0), -20.0),
];

impl Render for LevelSelectButtonBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let is_last_level = self.level == LEVEL_COUNT - 1;

        let mut pos = normal_pos(self, data.extracted_data.y_scroll, is_last_level);
        let t = data.transition_offset(1.8);
        pos.x += t * t;

        let scale_multiplier = self.scale_multiplier();

        // If the button is currently held, add rotation.
        let rotation = if self.held {
            6.0_f64 * data.oscillation_angle(2.6).sin()
        } else {
            0.0
        };

        if is_last_level {
            let pos = pos.into_fpoint();

            // 1: Draw the level button itself
            data.canvas.copy_ex(
                &data.textures.level_select.last_level_button,
                None,
                FRect::from_center(pos, 2.0 * SIDE * scale_multiplier, SIDE * scale_multiplier),
                rotation,
                None,
                false,
                false,
            )?;

            // 2: Draw the crown.
            const CROWN_SCALE: f32 = 0.85 * 2.5;
            data.canvas.copy(
                &data.textures.level_select.crown,
                None,
                FRect::from_center(pos, 43.0 * CROWN_SCALE, 28.0 * CROWN_SCALE),
            )?;

            return Ok(());
        }

        let star_byte = data.extracted_data.level_save.bits(self.level);
        let all_stars_collected = star_byte == ((1 << STAR_OFFSETS.len()) - 1);

        let level_buttons = &data.textures.level_select.level_buttons;
        let level_button_dimensions = DualImageDimensions::new(level_buttons);
        let level_button_rect = level_button_dimensions.frect(all_stars_collected);

        let digits_texture = if all_stars_collected {
            &mut data.textures.level_select.golden_digits
        } else {
            &mut data.textures.level_select.digits
        };

        // 1: Draw the level button itself
        data.canvas.copy_ex(
            &data.textures.level_select.level_buttons,
            level_button_rect,
            FRect::from_center(
                pos.into_fpoint(),
                SIDE * scale_multiplier,
                SIDE * scale_multiplier,
            ),
            rotation,
            None,
            false,
            false,
        )?;

        // 2: Draw the level number
        let mut number = self.level + 1;
        const NUMBER_SCALE: f32 = 0.85;
        const PERFECT_STAR_HIGHLIGHT_SCALE: f32 = 1.8;
        let mut digit_pos = pos
            + Vec2f::new(
                30.0 * NUMBER_SCALE * (common_util::digit_count(number) - 1) as f32 - 1.0,
                -10.0,
            );
        while number > 0 {
            let digit = number % 10;
            number /= 10;
            // Draw digits for the level.
            // The original texture is 600 by 80.
            digits_texture.set_color_mod(120, 120, 120);
            data.canvas.copy(
                digits_texture,
                FRect::new(60.0 * digit as f32, 0.0, 60.0, 80.0),
                FRect::from_center(
                    digit_pos.into_fpoint(),
                    60.0 * NUMBER_SCALE,
                    80.0 * NUMBER_SCALE,
                ),
            )?;
            digit_pos.x -= 60.0 * NUMBER_SCALE;
        }

        // 3: Draw the perfect star highlight, if required
        if all_stars_collected {
            let highlight_pos = pos + Vec2f::new(-2.0, 50.0);

            data.canvas.copy(
                &data.textures.level_select.perfect_star_highlight,
                None,
                FRect::from_center(
                    highlight_pos.into_fpoint(),
                    120.0 * PERFECT_STAR_HIGHLIGHT_SCALE,
                    53.0 * PERFECT_STAR_HIGHLIGHT_SCALE,
                ),
            )?;
        }

        let star_dimensions = DualImageDimensions::new(&data.textures.level_select.stars);

        const STAR_SIZE: f32 = 70.0;

        // 4: draw the stars
        for (i, star_offset) in STAR_OFFSETS.iter().enumerate() {
            let collected = (star_byte & (1 << i)) != 0;
            let star_src = star_dimensions.frect(collected);
            let star_pos = pos + star_offset.0.into();
            data.canvas.copy_ex(
                &data.textures.level_select.stars,
                star_src,
                FRect::from_center(star_pos.into_fpoint(), STAR_SIZE, STAR_SIZE),
                star_offset.1,
                None,
                false,
                false,
            )?;
        }

        Ok(())
    }
}

impl Logic for LevelSelectButtonBase {
    fn run_logic(&mut self, data: &mut crate::logic::LogicData) {
        let is_last_level = self.level == LEVEL_COUNT - 1;

        let hitbox = FRect::from_center(
            normal_pos(self, data.app_data.level_select_scroll, is_last_level).into_fpoint(),
            (if is_last_level { 2.0 } else { 1.0 }) * SIDE,
            SIDE,
        );
        let mouse_pos = data.mouse_pos();
        let hovered = (HEADER_HEIGHT as f32..(HEIGHT as f32 - HEADER_HEIGHT as f32))
            .contains(&mouse_pos.y)
            && hitbox.contains_point(mouse_pos);

        if data.is_mouse_button_up(MouseButton::Left) && hovered {
            // Play the level.
            data.app_data.level.playing_level = self.level;
            data.set_transition_call(TransitionCall::Start(TransitionData::new(
                800_000_000,
                500_000_000,
                Screen::Level,
            )));
        }

        if !hovered && self.hold_time > Self::MAX_DISPLAY_HOLD_TIME {
            self.hold_time = Self::MAX_DISPLAY_HOLD_TIME
        } else {
            self.update_hold_time(data.delta_time, hovered);
        }

        self.held = hovered;
    }
}
