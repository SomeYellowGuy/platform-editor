use crate::{
    AppData,
    component::{Hold, HoldBase},
    hold_impl,
    screen::Screen,
};

/// A struct providing button behavior.
pub struct ButtonBase {
    pub ty: ButtonType,

    base: HoldBase,
}

hold_impl!(ButtonBase: 600_000_000);

impl ButtonBase {
    pub const fn new(ty: ButtonType) -> Self {
        Self {
            ty,
            base: HoldBase::new(),
        }
    }

    pub const fn scale_multiplier(&self) -> f32 {
        let gradient = 1.0 - self.base.hold_time as f32 / Self::MAX_HOLD_TIME as f32;
        0.9 + 0.1 * (1.0 - gradient * gradient)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
/// A unique type of button on the title screen texture.
pub enum ButtonType {
    Play = 0,
    LevelSelect = 1,
    Options = 2,
}

impl ButtonType {
    pub const ALL: [Self; 3] = [Self::Play, Self::LevelSelect, Self::Options];

    pub fn title(self) -> &'static str {
        match self {
            Self::Play => "PLAY",
            Self::LevelSelect => "LEVEL SELECT",
            Self::Options => "OPTIONS",
        }
    }

    pub fn destination_screen(self) -> Screen {
        match self {
            Self::Play => Screen::Level,
            Self::LevelSelect => Screen::LevelSelect,
            Self::Options => Screen::Options,
        }
    }

    pub fn before_transition<E: Default>(self, data: &mut AppData<E>) {
        if self == Self::Play {
            data.level.playing_level = data.level.max_level();
        }
    }
}
