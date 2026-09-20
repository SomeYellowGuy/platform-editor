use platform_editor_core::{
    component::{
        Hold,
        title::button::{ButtonBase, ButtonType},
    },
    screen::{TransitionCall, TransitionData},
};
use sdl3::{
    mouse::MouseButton,
    pixels::Color,
    render::{FPoint, FRect, TextureCreator},
    video::WindowContext,
};

use crate::{
    logic::{Logic, LogicData},
    render::{Render, RenderData},
    textures::{DynamicText, TextAlignment},
    util::FRectExt,
};

/// Stores the font texture sets for use in the button text.
pub struct ExtractedFontTextureSets<'c> {
    pub play: ExtractedFontTextureSet<'c>,
    pub level_select: ExtractedFontTextureSet<'c>,
    pub options: ExtractedFontTextureSet<'c>,
}

impl<'c> ExtractedFontTextureSets<'c> {
    pub fn new(creator: &'c TextureCreator<WindowContext>) -> Self {
        Self {
            play: ExtractedFontTextureSet::new(creator),
            level_select: ExtractedFontTextureSet::new(creator),
            options: ExtractedFontTextureSet::new(creator),
        }
    }

    fn set(&mut self, ty: ButtonType) -> &mut ExtractedFontTextureSet<'c> {
        match ty {
            ButtonType::Play => &mut self.play,
            ButtonType::LevelSelect => &mut self.level_select,
            ButtonType::Options => &mut self.options,
        }
    }
}

pub struct ExtractedFontTextureSet<'c> {
    top: DynamicText<'c>,
    bottom: DynamicText<'c>,
}

impl<'c> ExtractedFontTextureSet<'c> {
    pub fn new(creator: &'c TextureCreator<WindowContext>) -> Self {
        Self {
            top: DynamicText::with_color(creator, Color::RGB(55, 55, 55)),
            bottom: DynamicText::with_color(creator, Color::RGBA(30, 30, 30, 60)),
        }
    }
}

fn vertical_pos(ty: ButtonType) -> f32 {
    match ty {
        ButtonType::Play => 300.0,
        ButtonType::LevelSelect => 450.0,
        ButtonType::Options => 600.0,
    }
}

fn button_rect(base: &ButtonBase, horizontal_offset: f32) -> FRect {
    let scale = base.scale_multiplier();
    FRect::from_center(
        FPoint::new(
            crate::WIDTH as f32 / 2.0 + horizontal_offset,
            vertical_pos(base.ty),
        ),
        600.0 * scale,
        150.0 * scale,
    )
}

impl Render for ButtonBase {
    fn render(&self, data: &mut RenderData) -> crate::render::DrawResult {
        let scale_multiplier = self.scale_multiplier();
        let vertical_pos = vertical_pos(self.ty);
        const BUTTON_SIZE: f32 = 128.0;

        let t = data.transition_offset(2.0);
        let horizontal_offset = t * t;

        data.canvas.copy_ex(
            &data.textures.title.button,
            None,
            button_rect(self, horizontal_offset),
            0.0,
            None,
            false,
            false,
        )?;
        data.canvas.copy_ex(
            &data.textures.title.button_icons,
            FRect::new(64.0 * (self.ty as u8) as f32, 0.0, 64.0, 64.0),
            FRect::from_center(
                FPoint::new(
                    (crate::WIDTH as f32 / 2.0 + horizontal_offset) + 225.0 * scale_multiplier,
                    vertical_pos - 3.0,
                ),
                BUTTON_SIZE * scale_multiplier,
                BUTTON_SIZE * scale_multiplier,
            ),
            0.0,
            None,
            false,
            false,
        )?;

        // Render the text.
        let ExtractedFontTextureSet { top, bottom } = data.textures.title.texts.set(self.ty);

        const FONT_SIZE_MULTIPLIER: f32 = 1.43;
        let text_size_multiplier = FONT_SIZE_MULTIPLIER * scale_multiplier;
        let horizontal_pos =
            crate::WIDTH as f32 / 2.0 - 270.0 * scale_multiplier + horizontal_offset;

        bottom.update(data.canvas, data.font, self.ty.title())?;
        bottom.draw(
            data.canvas,
            TextAlignment::Left,
            FPoint::new(horizontal_pos + 3.0, vertical_pos + 3.0),
            text_size_multiplier,
        )?;

        top.update(data.canvas, data.font, self.ty.title())?;
        top.draw(
            data.canvas,
            TextAlignment::Left,
            FPoint::new(horizontal_pos, vertical_pos),
            text_size_multiplier,
        )?;

        Ok(())
    }
}

impl Logic for ButtonBase {
    fn run_logic(&mut self, data: &mut LogicData) {
        let point = data.mouse_fpos();
        let hovered = button_rect(self, 0.0).contains_point(point);

        self.update_hold_time(data.delta_time, hovered);

        if hovered && data.is_mouse_button_up(MouseButton::Left) {
            self.ty.before_transition(data.app_data);
            data.set_transition_call(TransitionCall::Start(TransitionData::new(
                800_000_000,
                700_000_000,
                self.ty.destination_screen(),
            )));
        }
    }
}
