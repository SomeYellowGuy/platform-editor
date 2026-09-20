use std::{borrow::Cow, ffi::CString, marker::PhantomData, ptr};

use platform_editor_core::common_util::Vec2;
use sdl3::{
    get_error,
    pixels::Color,
    render::{Canvas, FPoint, FRect, TextureCreator},
    ttf::{Font, sys::TTF_RenderText_Blended},
    video::{Window, WindowContext},
};
use sdl3_sys::{
    render::{
        SDL_CreateTextureFromSurface, SDL_DestroyTexture, SDL_GetTextureSize, SDL_RenderTexture,
        SDL_SetTextureAlphaMod, SDL_SetTextureColorMod, SDL_Texture,
    },
    surface::{SDL_DestroySurface, SDL_Surface},
};

use crate::{render::DrawResult, util::FRectExt};

/// A text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl TextAlignment {
    /// Provides the shift decimal of this alignment.
    pub fn shift_decimal(self) -> f32 {
        match self {
            Self::Left => 0.0,
            Self::Center => 0.5,
            Self::Right => 1.0,
        }
    }
}

/// A special type of texture that allows setting its text content
/// while rendering without borrow issues.
pub struct DynamicText<'c> {
    phantom: PhantomData<&'c ()>,
    texture: *mut SDL_Texture,
    cached_text: Cow<'static, str>,
    color: Color,
}

impl DynamicText<'_> {
    /// Creates a new, empty [`DynamicText`] that does not render anything yet.
    ///
    /// Use [`DynamicText::update`] while rendering to provide something to
    /// display for this texture.
    pub fn new(creator: &TextureCreator<WindowContext>) -> Self {
        Self::with_color(creator, Color::WHITE)
    }

    /// Creates a new, empty [`DynamicText`], with the provided color, that does not render anything yet.
    ///
    /// Use [`DynamicText::update`] while rendering to provide something to
    /// display for this texture.
    #[expect(unused_variables, reason = "used to ensure safety")]
    pub fn with_color(creator: &TextureCreator<WindowContext>, color: impl Into<Color>) -> Self {
        Self {
            phantom: PhantomData,
            texture: ptr::null::<SDL_Texture>() as *mut SDL_Texture,
            cached_text: Cow::Borrowed(""),
            color: color.into(),
        }
    }

    /// Updates this texture if the provided (or targeted) text content is different
    /// from the one stored in this texture.
    pub fn update(
        &mut self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        new_text: impl Into<Cow<'static, str>>,
    ) -> DrawResult {
        let new_text = new_text.into();
        if !self.texture.is_null() && self.cached_text == new_text {
            // No change: reuse the existing texture
            return Ok(());
        }

        if !self.texture.is_null() {
            // SAFETY: texture is not a null pointer.
            unsafe { SDL_DestroyTexture(self.texture) };
            self.texture = ptr::null_mut();
        }

        let c_text = CString::new(new_text.to_string()).unwrap();
        self.cached_text = new_text;

        unsafe {
            let surface: *mut SDL_Surface =
                TTF_RenderText_Blended(font.raw(), c_text.as_ptr(), 0, Color::WHITE.into());

            if surface.is_null() {
                return Err(get_error());
            }

            self.texture = SDL_CreateTextureFromSurface(canvas.raw(), surface);

            SDL_DestroySurface(surface);
        }

        Ok(())
    }

    /// Renders the texture (rendering the text stored in this texture) to the screen
    /// if this texture has some loaded text.
    ///
    /// [`DynamicText::update`] will have to be used before calling this method.
    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        alignment: TextAlignment,
        position: impl Into<FPoint>,
        scale: f32,
    ) -> DrawResult {
        if self.texture.is_null() {
            return Ok(());
        }

        let Vec2 {
            x: width,
            y: height,
        } = self.dimensions();

        let width = scale * width;
        let mut position = position.into();
        position.x -= (alignment.shift_decimal() - 0.5) * width;
        let dst = FRect::from_center(position, width, scale * height);

        if unsafe {
            !SDL_SetTextureColorMod(self.texture, self.color.r, self.color.g, self.color.b)
        } {
            return Err(sdl3::get_error());
        }
        if unsafe { !SDL_SetTextureAlphaMod(self.texture, self.color.a) } {
            return Err(sdl3::get_error());
        }

        if unsafe { !SDL_RenderTexture(canvas.raw(), self.texture, ptr::null(), &dst.to_ll()) } {
            return Err(sdl3::get_error());
        }

        Ok(())
    }
}

impl DynamicText<'_> {
    /// Sets the alpha value of the color of this texture.
    #[inline]
    pub fn set_alpha_mod(&mut self, alpha: u8) {
        self.color.a = alpha;
    }

    /// Sets the color of this texture.
    pub fn set_color_mod(&mut self, color: impl Into<Color>) {
        self.color = color.into();
    }

    /// Returns the dimensions (both width and height) of this text
    /// (or `(0, 0)` if nothing has been loaded by this texture yet).
    pub fn dimensions(&self) -> Vec2<f32> {
        if self.texture.is_null() {
            return Vec2::new(0.0, 0.0);
        };

        let (mut width, mut height) = (0.0, 0.0);
        if unsafe { !SDL_GetTextureSize(self.texture, &mut width, &mut height) } {
            return Vec2::new(0.0, 0.0);
        }
        Vec2::new(width, height)
    }

    /// Returns the width of this text
    /// (or `0` if nothing has been loaded by this texture yet).
    pub fn width(&self) -> f32 {
        self.dimensions().x
    }

    /// Returns the height of this text
    /// (or `0` if nothing has been loaded by this texture yet).
    pub fn height(&self) -> f32 {
        self.dimensions().y
    }
}

impl Drop for DynamicText<'_> {
    fn drop(&mut self) {
        unsafe {
            if !self.texture.is_null() {
                SDL_DestroyTexture(self.texture);
            }
        }
    }
}
