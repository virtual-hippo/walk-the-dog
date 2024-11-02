use anyhow::Result;
use web_sys::{AudioBuffer, AudioContext};

use crate::{browser, sound};

#[derive(Clone)]
pub(crate) struct Audio {
    context: AudioContext,
}

impl Audio {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            context: sound::create_audio_context()?,
        })
    }

    pub(crate) async fn load_sound(&self, filename: &str) -> Result<Sound> {
        let array_buffer = browser::fetch_array_buffer(filename).await?;
        let audio_buffer = sound::decode_audio_data(&self.context, &array_buffer).await?;
        Ok(Sound {
            buffer: audio_buffer,
        })
    }

    pub(crate) fn play_sound(&self, sound: &Sound) -> Result<()> {
        sound::play_sound(&self.context, &sound.buffer)
    }
}

#[derive(Clone)]
pub(crate) struct Sound {
    buffer: AudioBuffer,
}
