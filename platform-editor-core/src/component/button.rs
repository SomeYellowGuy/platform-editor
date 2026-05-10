/// A struct prividing button behavior.
pub struct ButtonBase {
    pub ty: ButtonType,
    /// The held time for this button (in nanoseconds), which increases when held and decreases when released
    /// by the current delta time.
    ///
    /// This will be from 0 to 600,000,000.
    pub held_time: u32,
}

impl ButtonBase {
    pub const MAX_HELD_TIME: u32 = 600_000_000;

    pub const fn new(ty: ButtonType) -> Self {
        Self { ty, held_time: 0 }
    }

    pub const fn scale_multiplier(&self) -> f32 {
        let gradient = 1.0 - self.held_time as f32 / Self::MAX_HELD_TIME as f32;
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
}
