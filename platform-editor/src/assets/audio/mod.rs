use platform_editor_core::audio::{AudioParams, AudioPlay, Sound};
use sdl3::{
    AudioSubsystem, Sdl,
    iostream::IOStream,
    libc,
    mixer::{
        Audio, Mixer, MixerContext,
        sys::{
            MIX_CreateTrack, MIX_DestroyTrack, MIX_PlayTrack, MIX_SetTrackAudio,
            MIX_SetTrackFrequencyRatio, MIX_SetTrackGain, MIX_Track,
        },
    },
};
use sdl3_sys::properties::SDL_PropertiesID;

use crate::assets::bundled;

type BoxedAudioSlice = Box<[Audio]>;

const SOUND_EFFECT_TRACKS: usize = 16;

pub struct AudioPlayer {
    _subsystem: AudioSubsystem,
    _mixer_context: MixerContext,

    _mixer: Mixer,
    audios: BoxedAudioSlice,

    sound_effect_tracks: Box<[*mut MIX_Track]>,

    target_track: usize,
}

impl AudioPlayer {
    fn create_track(mixer: &Mixer) -> Result<*mut MIX_Track, sdl3::Error> {
        let raw = unsafe { MIX_CreateTrack(mixer.raw()) };
        if raw.is_null() {
            Err(sdl3::get_error())
        } else {
            Ok(raw)
        }
    }

    pub fn init(context: &Sdl) -> Result<Self, sdl3::Error> {
        let _subsystem = context.audio()?;
        let _mixer_context = sdl3::mixer::init()?;

        let mixer = Mixer::open_device(None)?;
        let audios = Self::load_all_audio(&mixer)?;

        let mut tracks = Vec::new();
        for _ in 0..SOUND_EFFECT_TRACKS {
            tracks.push(Self::create_track(&mixer)?);
        }
        let tracks = tracks.into_boxed_slice();

        Ok(Self {
            _subsystem,
            _mixer_context,

            _mixer: mixer,
            audios,

            sound_effect_tracks: tracks,
            target_track: 0,
        })
    }

    fn load_all_audio(mixer: &Mixer) -> Result<BoxedAudioSlice, sdl3::Error> {
        let mut audios = Vec::new();
        for sound in Sound::ALL {
            audios.push(Self::load(mixer, sound)?);
        }
        Ok(audios.into_boxed_slice())
    }

    fn load(mixer: &Mixer, sound: Sound) -> Result<Audio, sdl3::Error> {
        let Some(bytes) = bundled::get_audio_data(sound) else {
            sdl3::set_error(&format!("Could not get buffer data for sound {sound:?}")).expect(
                "we provided a path without NUL bytes, so setting the error should not fail",
            );
            return Err(sdl3::get_error());
        };

        let io = unsafe {
            let ll_io = sdl3_sys::iostream::SDL_IOFromMem(
                bytes.as_ptr() as *mut libc::c_void,
                bytes.len()
            );

            IOStream::from_ll(ll_io)
        };

        mixer.load_audio_io(&io, true)
    }

    fn target_track(&mut self) -> *mut MIX_Track {
        let track = self.sound_effect_tracks[self.target_track];

        // Increment the internal track index.
        self.target_track = (self.target_track + 1) % SOUND_EFFECT_TRACKS;

        track
    }

    fn audio(&self, sound: Sound) -> &Audio {
        &self.audios[sound as usize]
    }

    /// Tries to play the provided sound.
    pub fn play_with_params(&mut self, sound: Sound, params: AudioParams) {
        let track = self.target_track();
        unsafe {
            if !MIX_SetTrackAudio(track, self.audio(sound).raw()) {
                tracing::warn!("Could not set audio for track: {}", sdl3::get_error());
                return;
            }
            if !MIX_SetTrackGain(track, params.volume) {
                tracing::warn!("Could not set pitch gain for track: {}", sdl3::get_error());
                return;
            }
            if !MIX_SetTrackFrequencyRatio(track, params.pitch) {
                tracing::warn!("Could not set pitch ratio for track: {}", sdl3::get_error());
                return;
            }
            if !MIX_PlayTrack(track, SDL_PropertiesID(0)) {
                tracing::warn!("Could not play track: {}", sdl3::get_error());
            }
        }
    }

    /// Tries to play the provided sound.
    pub fn play(&mut self, sound: Sound) {
        self.play_with_params(sound, AudioParams::new());
    }
}

impl AudioPlay for AudioPlayer {
    fn play(&mut self, sound: Sound) {
        self.play(sound)
    }

    fn play_with_params(&mut self, sound: Sound, params: AudioParams) {
        self.play_with_params(sound, params);
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        // Free the manually-managed tracks.
        for track in &self.sound_effect_tracks {
            unsafe {
                MIX_DestroyTrack(*track);
            }
        }
    }
}
