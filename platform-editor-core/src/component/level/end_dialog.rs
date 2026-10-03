use std::time::Instant;

use crate::{
    audio::{AudioParams, AudioPlay, Sound},
    component::{Hold, HoldBase},
    hold_impl,
    level::{StarCondition, state::LevelState},
};

/// The pop-up that shows when you beat a level.
#[derive(Debug, Clone)]
pub struct EndDialogBase {
    pub start: Instant,
    /// All statuses of the stars starting from the second (the first is guaranteed to be collected).
    pub star_statuses: Vec<StarStatus>,

    /// The last number of stars shown to be collected or uncollected to the player.
    ///
    /// For example:
    /// - `1.5` means that the first star has been shown, and we are halfway to the second.
    /// - `2.2` means that the first two stars have been shown, and we are 20% of the way to the third.
    pub last_shown_stars: f32,

    /// The number of stars shown to be collected by the player.
    pub collected_stars: usize,
}

impl EndDialogBase {
    pub const DELAY: f32 = 1.0;
    pub const FADE_IN_TIME: f32 = 0.5;

    pub const STAR_DELAY: f32 = 1.0;
    pub const STAR_ANIMATION_DURATION: f32 = 1.6;

    fn params(collected: usize) -> AudioParams {
        let extra = collected as f32 - 1.0;
        AudioParams::new().volume(1.0 + extra * 0.2)
    }

    pub fn play_star_sound(&self, audio_play: &mut impl AudioPlay) {
        audio_play.play_with_params(Sound::StarAward, Self::params(self.collected_stars));
    }
}

#[derive(Debug, Clone)]
pub struct StarStatus {
    pub collected: bool,
    pub condition: StarCondition,
}

impl StarStatus {
    /// Converts an array of star statuses into a packed byte, assuming that the first
    /// star (the flag finish star) is collected.
    pub fn to_bits(statuses: &[StarStatus]) -> u8 {
        let mut bits = 1;
        let checked_statuses = (std::mem::size_of::<u8>() * 8).min(statuses.len());
        for (i, status) in statuses.iter().enumerate().take(checked_statuses) {
            if status.collected {
                bits |= 1 << (i + 1);
            }
        }

        bits
    }
}

impl EndDialogBase {
    pub fn from_level_state(state: &LevelState) -> Self {
        Self {
            start: state.finish_instant.unwrap_or_else(Instant::now),
            star_statuses: state.star_statuses(),
            last_shown_stars: 0.0,
            collected_stars: 0,
        }
    }

    /// The lifetime of this dialog in seconds.
    pub fn lifetime(&self) -> f32 {
        self.start.elapsed().as_secs_f32()
    }
}

/// Represents a button on the end dialog.
pub struct EndDialogButtonBase {
    pub start: Instant,
    pub ty: EndDialogButtonType,

    base: HoldBase,
}

impl EndDialogButtonBase {
    pub const TOTAL_DELAY: f32 = EndDialogBase::DELAY
        + EndDialogBase::FADE_IN_TIME
        + EndDialogBase::STAR_DELAY
        + EndDialogBase::STAR_ANIMATION_DURATION;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndDialogButtonType {
    LevelSelect,
    Next,
    Retry,
}

impl EndDialogButtonType {
    pub const ALL: [Self; 3] = [Self::LevelSelect, Self::Next, Self::Retry];
}

hold_impl!(EndDialogButtonBase: 400_000_000);

impl EndDialogButtonBase {
    pub const FADE_IN_TIME: f32 = 0.5;

    pub fn new(state: &LevelState, ty: EndDialogButtonType) -> Self {
        Self {
            start: state.finish_instant.unwrap_or_else(Instant::now),
            ty,
            base: HoldBase::new(),
        }
    }

    pub const fn scale_multiplier(&self) -> f32 {
        let gradient = 1.0 - self.base.hold_time as f32 / Self::MAX_HOLD_TIME as f32;
        0.9 + 0.1 * (1.0 - gradient * gradient)
    }
}
