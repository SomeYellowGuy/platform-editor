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
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

use crate::{
    logic::{Logic, LogicData},
    render::{Render, RenderData},
};

/// Stores the font texture sets for use in the button text.
pub struct ExtractedFontTextureSets<'c> {
    pub play: ExtractedFontTextureSet<'c>,
    pub level_select: ExtractedFontTextureSet<'c>,
    pub options: ExtractedFontTextureSet<'c>,
}

impl<'c> ExtractedFontTextureSets<'c> {
    pub fn new(creator: &'c TextureCreator<WindowContext>, font: &Font) -> Option<Self> {
        Some(Self {
            play: ExtractedFontTextureSet::new(creator, font, ButtonType::Play)?,
            level_select: ExtractedFontTextureSet::new(creator, font, ButtonType::LevelSelect)?,
            options: ExtractedFontTextureSet::new(creator, font, ButtonType::Options)?,
        })
    }

    fn set(&'c self, ty: ButtonType) -> &'c ExtractedFontTextureSet<'c> {
        match ty {
            ButtonType::Play => &self.play,
            ButtonType::LevelSelect => &self.level_select,
            ButtonType::Options => &self.options,
        }
    }
}

pub struct ExtractedFontTextureSet<'c> {
    top: Texture<'c>,
    bottom: Texture<'c>,
}

impl<'c> ExtractedFontTextureSet<'c> {
    fn texture(
        creator: &'c TextureCreator<WindowContext>,
        font: &Font,
        ty: ButtonType,
        color: Color,
    ) -> Option<Texture<'c>> {
        let surface = font.render(ty.title()).blended(color).ok()?;

        let texture = creator.create_texture_from_surface(&surface).ok()?;

        Some(texture)
    }

    pub fn new(
        creator: &'c TextureCreator<WindowContext>,
        font: &Font,
        ty: ButtonType,
    ) -> Option<Self> {
        Some(Self {
            top: Self::texture(creator, font, ty, Color::RGB(55, 55, 55))?,
            bottom: Self::texture(creator, font, ty, Color::RGBA(30, 30, 30, 60))?,
        })
    }
}

fn vertical_pos(ty: ButtonType) -> i32 {
    match ty {
        ButtonType::Play => 300,
        ButtonType::LevelSelect => 450,
        ButtonType::Options => 600,
    }
}

fn draw_text_texture(
    texture: &Texture,
    canvas: &mut Canvas<Window>,
    offset: i32,
    scale_multiplier: f32,
    vertical_pos: i32,
    horizontal_offset: i32,
) -> crate::render::DrawResult {
    const FONT_SIZE_MULTIPLIER: f32 = 1.43;
    let height = texture.height() as f32 * FONT_SIZE_MULTIPLIER * scale_multiplier;

    canvas.copy_ex(
        texture,
        None,
        Rect::new(
            (crate::WIDTH as i32 / 2) - (276.0 * scale_multiplier) as i32
                + offset
                + horizontal_offset,
            vertical_pos - height as i32 / 2 + offset,
            (texture.width() as f32 * FONT_SIZE_MULTIPLIER * scale_multiplier) as u32,
            height as u32,
        ),
        0.0,
        None,
        false,
        false,
    )
}

fn button_rect(base: &ButtonBase, horizontal_offset: i32) -> Rect {
    let scale = base.scale_multiplier();
    Rect::from_center(
        Point::new(
            crate::WIDTH as i32 / 2 + horizontal_offset,
            vertical_pos(base.ty),
        ),
        (600.0 * scale) as u32,
        (150.0 * scale) as u32,
    )
}

impl Render for ButtonBase {
    fn render(&self, data: &mut RenderData) -> crate::render::DrawResult {
        let scale_multiplier = self.scale_multiplier();
        let vertical_pos = vertical_pos(self.ty);
        const BUTTON_SIZE: f32 = 128.0;

        let t = data.transition_offset(2.0);
        let horizontal_offset = (t * t) as i32;

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
            Rect::new(64 * (self.ty as u8) as i32, 0, 64, 64),
            Rect::from_center(
                Point::new(
                    (crate::WIDTH as i32 / 2 + horizontal_offset)
                        + (225.0 * scale_multiplier) as i32,
                    vertical_pos - 3,
                ),
                (BUTTON_SIZE * scale_multiplier) as u32,
                (BUTTON_SIZE * scale_multiplier) as u32,
            ),
            0.0,
            None,
            false,
            false,
        )?;

        // Render the text.
        let ExtractedFontTextureSet { top, bottom } = data.textures.title.texts.set(self.ty);

        draw_text_texture(
            bottom,
            data.canvas,
            3,
            scale_multiplier,
            vertical_pos,
            horizontal_offset,
        )?;
        draw_text_texture(
            top,
            data.canvas,
            0,
            scale_multiplier,
            vertical_pos,
            horizontal_offset,
        )?;

        Ok(())
    }
}

// TODO
impl Logic for ButtonBase {
    fn run_logic(&mut self, data: &mut LogicData) {
        let point = data.mouse_pos();
        let hovered = button_rect(self, 0).contains_point(point);

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
