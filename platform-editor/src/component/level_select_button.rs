use platform_editor_core::{
    common_util,
    component::{Hold, level_select_button::LevelSelectButtonBase},
};
use sdl3::{
    rect::Rect,
    render::{FPoint, FRect},
};

use crate::{
    logic::Logic,
    render::Render,
    util::{self, frect_from_center},
};

pub const SPACING: f32 = 220.0;
pub const SIDE: f32 = 190.0;

pub const LEVELS_PER_ROW: usize = 4;

fn normal_pos(base: &LevelSelectButtonBase, scroll: f32) -> FPoint {
    let horizontal = base.level % 4;
    let vertical = base.level / 4;

    FPoint::new(
        crate::WIDTH as f32 / 2.0
            + SPACING * (horizontal as f32 - (LEVELS_PER_ROW - 1) as f32 / 2.0),
        200.0 + SPACING * vertical as f32 + scroll,
    )
}

impl Render for LevelSelectButtonBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let pos = normal_pos(self, data.extracted_data.y_scroll);
        let scale_multiplier = self.scale_multiplier();

        // If the button is currently held, add rotation.
        let rotation = if self.held {
            6.0_f64 * data.oscillation_angle(2.6).sin()
        } else {
            0.0
        };

        data.canvas.copy_ex(
            &data.images.level_select.level_buttons,
            Rect::new(0, 0, 90, 90),
            util::frect_from_center(pos, SIDE * scale_multiplier, SIDE * scale_multiplier),
            rotation,
            None,
            false,
            false,
        )?;
        let mut number = self.level + 1;
        const NUMBER_SCALE: f32 = 0.85;
        let mut digit_pos = pos;

        digit_pos.x += 30.0 * NUMBER_SCALE * (common_util::digit_count(number) - 1) as f32;
        digit_pos.x -= 1.0;
        while number > 0 {
            let digit = number % 10;
            number /= 10;
            // Draw digits for the level.
            // The original texturte is 600 by 80.
            data.images.level_select.digits.set_color_mod(120, 120, 120);
            data.canvas.copy_ex(
                &data.images.level_select.digits,
                FRect::new(60.0 * digit as f32, 0.0, 60.0, 80.0),
                frect_from_center(digit_pos, 60.0 * NUMBER_SCALE, 80.0 * NUMBER_SCALE),
                0.0,
                None,
                false,
                false,
            )?;
            digit_pos.x -= 60.0 * NUMBER_SCALE;
        }

        Ok(())
    }
}

impl Logic for LevelSelectButtonBase {
    fn run_logic(&mut self, data: &mut crate::logic::LogicData) {
        let hitbox = util::frect_to_rect(frect_from_center(
            normal_pos(self, data.app_data.level_select_scroll),
            SIDE,
            SIDE,
        ));
        let hovered = hitbox.contains_point(data.mouse_pos());

        if !hovered && self.hold_time > Self::MAX_DISPLAY_HOLD_TIME {
            self.hold_time = Self::MAX_DISPLAY_HOLD_TIME
        } else {
            self.update_hold_time(data.delta_time, hovered);
        }

        self.held = hovered;
    }
}
