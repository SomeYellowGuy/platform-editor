use crate::{level::scratch::levels::LEVEL_COUNT, save::SaveFile};

use crate::save::ReadFrom;

pub struct LevelSave {
    pub beat_levels: usize,
    pub playing_level: usize,
    stars: [u8; LEVEL_COUNT],
}

impl LevelSave {
    pub fn new() -> Self {
        Self {
            beat_levels: 0,
            playing_level: 0,
            stars: [0; LEVEL_COUNT],
        }
    }

    pub fn from_save(beat_levels: usize, stars: [u8; LEVEL_COUNT]) -> Self {
        Self {
            beat_levels,
            playing_level: 0,
            stars,
        }
    }

    pub fn is_beat(&self, level: usize) -> bool {
        self.stars[level] > 0
    }

    pub fn bits(&self, level: usize) -> u8 {
        self.stars[level]
    }

    pub fn award(&mut self, level: usize, stars: u8) {
        self.stars[level] |= stars
    }

    pub fn stars_collected(&self) -> u32 {
        self.stars.iter().map(|num| num.count_ones()).sum()
    }
}

impl Default for LevelSave {
    fn default() -> Self {
        Self::new()
    }
}

const CURRENT_VERSION: usize = 0;

impl SaveFile for LevelSave {
    type Version = usize;

    fn current_version() -> Self::Version {
        CURRENT_VERSION
    }

    fn write_content(&self, writer: &mut impl std::io::prelude::Write) -> std::io::Result<()> {
        writer.write_all(&self.beat_levels.to_le_bytes())?;
        writer.write_all(&self.stars.len().to_le_bytes())?;
        writer.write_all(&self.stars)?;

        Ok(())
    }

    fn read_content(
        _version: Self::Version,
        data: &mut std::io::Cursor<&[u8]>,
    ) -> std::io::Result<Self> {
        let beat_levels = usize::read(data)?;
        let stars_len = (usize::read(data)?).min(LEVEL_COUNT);
        let mut stars = Vec::new();

        for _ in 0..stars_len {
            let byte = u8::read(data)?;
            stars.push(byte);
        }
        // Pad the remaining unfilled bytes.
        stars.extend(std::iter::repeat_n(0x00, LEVEL_COUNT - stars_len));

        // SAFETY: stars has the exact number of required elements.
        let stars = unsafe { <[u8; LEVEL_COUNT]>::try_from(stars).unwrap_unchecked() };

        Ok(Self::from_save(beat_levels, stars))
    }
}
