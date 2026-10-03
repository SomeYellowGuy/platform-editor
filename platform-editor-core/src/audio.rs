#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sound {
    Back,
    Click,
    ClickFail,

    // During the level select
    Hover,

    // During a level
    Lose,
    Place,
    Disintegrate,
    Shoot,
    Collect,
    Win,
    StarAward,
    Jump,
    EnemyDefeat,
}

impl Sound {
    pub const ALL: [Self; 13] = [
        Self::Back,
        Self::Click,
        Self::ClickFail,
        Self::Hover,
        Self::Lose,
        Self::Place,
        Self::Disintegrate,
        Self::Shoot,
        Self::Collect,
        Self::Win,
        Self::StarAward,
        Self::Jump,
        Self::EnemyDefeat,
    ];

    pub fn relative_path(&self) -> &'static str {
        match self {
            Self::Back => "back.wav",
            Self::Click => "click.wav",
            Self::ClickFail => "click_fail.wav",
            Self::Hover => "hover.wav",
            Self::Lose => "lose.wav",
            Self::Place => "place.wav",
            Self::Disintegrate => "disintegrate.wav",
            Self::Shoot => "shoot.wav",
            Self::Collect => "collect.wav",
            Self::Win => "win.wav",
            Self::StarAward => "star_award.wav",
            Self::Jump => "jump.wav",
            Self::EnemyDefeat => "enemy_defeat.wav",
        }
    }
}

/// A trait for something to be able to play a sound.
pub trait AudioPlay {
    /// Plays a [`Sound`].
    fn play(&mut self, sound: Sound);

    /// Plays a [`Sound`] with the provided parameters.
    fn play_with_params(&mut self, sound: Sound, params: AudioParams);
}

#[derive(Debug, Clone, Copy)]
pub struct AudioParams {
    pub pitch: f32,
    pub volume: f32,
}

impl AudioParams {
    pub fn new() -> Self {
        Self {
            pitch: 1.0,
            volume: 1.0,
        }
    }

    pub fn pitch(mut self, pitch: f32) -> Self {
        self.pitch = pitch;
        self
    }

    pub fn volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }
}

impl Default for AudioParams {
    fn default() -> Self {
        Self::new()
    }
}
