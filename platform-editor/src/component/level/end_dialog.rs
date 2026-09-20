use platform_editor_core::{
    common_util::Vec2f,
    component::{
        Hold,
        level::end_dialog::{EndDialogBase, EndDialogButtonBase, EndDialogButtonType, StarStatus},
    },
    screen::{Screen, TransitionCall, TransitionData},
};
use sdl3::{
    mouse::MouseButton,
    pixels::Color,
    render::{BlendMode, FPoint, FRect},
};

use crate::{
    HEIGHT, WIDTH,
    logic::Logic,
    render::{DrawResult, Render, RenderData},
    textures::{DynamicText, TextAlignment},
    util::{FPointExt, FRectExt, IntoFPoint},
};

pub const DIALOG_CENTER: FPoint = FPoint {
    x: WIDTH as f32 / 2.0,
    y: HEIGHT as f32 / 2.0,
};

pub const DIALOG_NICE_CENTER: FPoint = FPoint {
    x: DIALOG_CENTER.x,
    y: DIALOG_CENTER.y - 270.0,
};
pub const DIALOG_LEVEL_CENTER: FPoint = FPoint {
    x: DIALOG_CENTER.x,
    y: DIALOG_CENTER.y + 80.0,
};

impl Render for EndDialogBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let elapsed = self.start.elapsed().as_secs_f32();

        if elapsed < Self::DELAY {
            return Ok(());
        }

        data.canvas.set_blend_mode(BlendMode::Blend);

        let alpha = ((elapsed - Self::DELAY).clamp(0.0, Self::FADE_IN_TIME)
            - data.transition_offset(0.075))
        .max(0.0)
            * (255.0 / Self::FADE_IN_TIME);
        let alpha_mod = alpha as u8;
        data.textures.level.end_dialog.base.set_alpha_mod(alpha_mod);
        data.textures
            .level
            .end_dialog
            .base
            .set_color_mod(190, 190, 190);

        data.canvas
            .set_draw_color(Color::RGBA(0, 0, 0, (alpha * 0.5) as u8));
        data.canvas.fill_rect(None)?;

        // Draw the end dialog base (plate).
        data.canvas.copy_ex(
            &data.textures.level.end_dialog.base,
            None,
            FRect::from_center(DIALOG_CENTER, 550.0, 660.0),
            0.0,
            None,
            false,
            false,
        )?;

        // The NICE and level text.
        data.textures
            .level
            .end_dialog
            .nice_text
            .set_alpha_mod(alpha_mod);

        data.textures
            .level
            .end_dialog
            .nice_text
            .update(data.canvas, data.font, "NICE!")?;
        data.textures.level.end_dialog.nice_text.draw(
            data.canvas,
            TextAlignment::Center,
            DIALOG_NICE_CENTER,
            1.5,
        )?;

        data.textures
            .level
            .end_dialog
            .level_text
            .set_alpha_mod(alpha_mod);
        let tex = &mut data.textures.level.end_dialog.level_text;
        tex.update(
            data.canvas,
            data.font,
            format!("Level {}", data.extracted_data.playing_level + 1),
        )?;
        tex.draw(data.canvas, TextAlignment::Center, DIALOG_LEVEL_CENTER, 0.9)?;

        render_stars(data, elapsed, alpha_mod, &self.star_statuses)?;

        Ok(())
    }
}

/// Renders the stars with the provided arguments. Returns the time since the first star updated.
fn render_stars(
    data: &mut RenderData,
    elapsed: f32,
    alpha_mod: u8,
    star_statuses: &[StarStatus],
) -> DrawResult {
    // Update the number textures.
    if data.textures.level.end_dialog.number_texts.len() != star_statuses.len() {
        data.textures.level.end_dialog.number_texts.clear();
        let creator = data.canvas.texture_creator();
        data.textures.level.end_dialog.number_texts =
            std::iter::repeat_with(|| DynamicText::new(&creator))
                .take(star_statuses.len())
                .collect();
    }

    let elapsed_for_stars =
        elapsed - EndDialogBase::DELAY - EndDialogBase::FADE_IN_TIME - EndDialogBase::STAR_DELAY;
    let stars_updated = 1.0
        + (elapsed_for_stars / EndDialogBase::STAR_ANIMATION_DURATION).max(0.0)
            * (star_statuses.len() as f32);

    let mut angle: f64 = star_statuses.len() as f64 * -STAR_ANGLE_GAP / 2.0;
    data.textures
        .level
        .end_dialog
        .stars
        .set_alpha_mod(alpha_mod);

    render_star(
        data,
        alpha_mod,
        &mut angle,
        elapsed_for_stars,
        stars_updated,
        0,
        None,
    )?;
    for (i, status) in star_statuses.iter().enumerate() {
        render_star(
            data,
            alpha_mod,
            &mut angle,
            elapsed_for_stars,
            stars_updated,
            i + 1,
            Some(status),
        )?
    }

    Ok(())
}

pub const STAR_SIZE: f32 = 150.0;
pub const STAR_ANGLE_GAP: f64 = 10.0;
pub const STAR_CIRCLE_CENTER: FPoint = FPoint {
    x: WIDTH as f32 / 2.0,
    y: 1130.0,
};
pub const ICON_SCALE: f32 = 0.85;
pub const MAX_ICON_SIZE: u32 = 100;
pub const STAR_CIRCLE_RADIUS: f32 = 900.0;
pub const STAR_CIRCLE_CONDITION_RADIUS: f32 = 770.0;
pub const MAX_STAR_CONDITION_NUMBER_WIDTH: f32 = 60.0;

fn render_star(
    data: &mut RenderData,
    alpha_mod: u8,
    angle: &mut f64,
    elapsed_for_stars: f32,
    stars_updated: f32,
    i: usize,
    status: Option<&StarStatus>,
) -> DrawResult {
    let tex = &data.textures.level.end_dialog.stars;
    let (sin, cos) = (*angle as f32).to_radians().sin_cos();

    let star_pos = FPoint::new(
        STAR_CIRCLE_CENTER.x + STAR_CIRCLE_RADIUS * sin,
        STAR_CIRCLE_CENTER.y - STAR_CIRCLE_RADIUS * cos,
    );

    let individual_width = (tex.width() / 2) as f32;
    let collected = status.is_none_or(|s| s.collected);
    let stars_updated_diff = stars_updated - (i + 1) as f32;

    let src = FRect::new(
        if collected && elapsed_for_stars > 0.0 && stars_updated_diff > 0.0 {
            individual_width
        } else {
            0.0
        },
        0.0,
        individual_width,
        tex.height() as f32,
    );

    let t = if stars_updated_diff > 0.0 {
        1.0 - (stars_updated_diff * 1.5).min(1.0)
    } else {
        0.0
    };

    let size_multiplier = 1.0 + 0.2 * (t * t);
    let size = STAR_SIZE * size_multiplier;

    data.canvas.copy_ex(
        tex,
        src,
        FRect::from_center(star_pos, size, size),
        *angle,
        None,
        false,
        false,
    )?;

    // Draw the star condition (icon + possible number)

    let condition_texture = data
        .textures
        .icons
        .get_mut(status.as_ref().map(|s| &s.condition));
    condition_texture.set_alpha_mod(alpha_mod);
    let mut star_condition_pos = Vec2f::new(
        STAR_CIRCLE_CENTER.x + STAR_CIRCLE_CONDITION_RADIUS * sin,
        STAR_CIRCLE_CENTER.y - STAR_CIRCLE_CONDITION_RADIUS * cos,
    );
    let icon_size = Vec2f::new(
        condition_texture.width().min(MAX_ICON_SIZE) as f32,
        condition_texture.height().min(MAX_ICON_SIZE) as f32,
    ) * ICON_SCALE;

    if i > 0
        && let Some(number) = status.and_then(|s| s.condition.number_display())
    {
        let text = &mut data.textures.level.end_dialog.number_texts[i - 1];
        text.set_alpha_mod(alpha_mod);
        text.update(data.canvas, data.font, number.to_string())?;

        let shift = 18.0;
        let text_scale = (MAX_STAR_CONDITION_NUMBER_WIDTH / text.width()).min(1.0);

        star_condition_pos += Vec2f::new(shift, 0.0);
        text.draw(
            data.canvas,
            TextAlignment::Left,
            star_condition_pos.into_fpoint(),
            text_scale,
        )?;

        star_condition_pos -= Vec2f::new(2.0 * shift, 0.0);
    }

    data.canvas.copy(
        condition_texture,
        None,
        FRect::from_center(star_condition_pos.into_fpoint(), icon_size.x, icon_size.y),
    )?;

    *angle += 10.0;

    Ok(())
}

pub const BUTTON_BASE_SIZE: f32 = 160.0;
pub const BUTTON_Y: f32 = 585.0;
pub const BUTTON_GAP: f32 = 175.0;

fn fpoint_for_button(ty: EndDialogButtonType) -> FPoint {
    FPoint::new(
        WIDTH as f32 / 2.0
            + match ty {
                EndDialogButtonType::LevelSelect => 0.0,
                EndDialogButtonType::Next => BUTTON_GAP,
                EndDialogButtonType::Retry => -BUTTON_GAP,
            },
        BUTTON_Y,
    )
}

impl Logic for EndDialogBase {
    fn run_logic(&mut self, _data: &mut crate::logic::LogicData) {}
}

impl Render for EndDialogButtonBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let elapsed_for_buttons = self.start.elapsed().as_secs_f32() - Self::TOTAL_DELAY;

        if elapsed_for_buttons < 0.0 {
            return Ok(());
        }

        let alpha_mod = ((elapsed_for_buttons.clamp(0.0, Self::FADE_IN_TIME)
            - data.transition_offset(0.075))
        .max(0.0)
            * (255.0 / Self::FADE_IN_TIME)) as u8;

        let button_textures = &mut data.textures.level.end_dialog.buttons;
        let texture = match self.ty {
            EndDialogButtonType::LevelSelect => &mut button_textures.level_select,
            EndDialogButtonType::Next => &mut button_textures.next,
            EndDialogButtonType::Retry => &mut button_textures.retry,
        };
        texture.set_alpha_mod(alpha_mod);

        let size = BUTTON_BASE_SIZE * self.scale_multiplier();

        let rect = FRect::from_center(fpoint_for_button(self.ty), size, size);

        data.canvas.copy(texture, None, rect)?;

        Ok(())
    }
}

impl Logic for EndDialogButtonBase {
    fn run_logic(&mut self, data: &mut crate::logic::LogicData) {
        if self.start.elapsed().as_secs_f32() < Self::TOTAL_DELAY {
            return;
        }

        let hovered = fpoint_for_button(self.ty).distance_sqr(data.mouse_fpos())
            < (BUTTON_BASE_SIZE * BUTTON_BASE_SIZE) / 4.0;

        self.update_hold_time(data.delta_time, hovered);

        if data.is_mouse_button_up(MouseButton::Left) && hovered {
            match self.ty {
                EndDialogButtonType::LevelSelect => {
                    data.set_transition_call(TransitionCall::Start(TransitionData::new(
                        600_000_000,
                        800_000_000,
                        Screen::LevelSelect,
                    )))
                }
                _ => {
                    if self.ty == EndDialogButtonType::Next {
                        data.app_data.level.playing_level += 1;
                    }
                    data.set_transition_call(TransitionCall::Start(TransitionData::new(
                        0,
                        0,
                        Screen::Level,
                    )))
                }
            }
        }
    }
}
