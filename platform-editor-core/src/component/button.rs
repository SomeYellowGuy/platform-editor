/// A struct prividing button behavior.
pub struct ButtonBase {
    pub ty: ButtonType,
    pub scale: f32,
}

impl ButtonBase {
    pub const fn new(ty: ButtonType) -> Self {
        Self { ty, scale: 0.9 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
/// A unique type of button on the title screen.texture
pub enum ButtonType {
    Play = 0,
    LevelSelect = 1,
    Options = 2,
}

impl ButtonType {
    pub const ALL: [Self; 3] = [Self::Play, Self::LevelSelect, Self::Options];

    /// Returns the component ID of the button associated with this type.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Play => "button_play",
            Self::LevelSelect => "button_level_select",
            Self::Options => "button_options",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Play => "PLAY",
            Self::LevelSelect => "LEVEL SELECT",
            Self::Options => "OPTIONS",
        }
    }
}
