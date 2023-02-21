use sdl2::audio::{AudioDevice, AudioSpecDesired, AudioCallback, AudioStatus};

use crate::audible::Audible;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct SDLSoundDevice {
    sdl_sound_device: AudioDevice<SquareWave>,
}

impl SDLSoundDevice {
    pub fn new(sdl_context: &sdl2::Sdl) -> SDLSoundDevice {
        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired{
            freq: Some(44_100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave{
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        }).unwrap();

        SDLSoundDevice { sdl_sound_device: device }
    }
}

impl Audible for SDLSoundDevice {
    fn enable_sound(&mut self) {
        if self.sdl_sound_device.status() == AudioStatus::Paused {
            self.sdl_sound_device.resume();
        }
    }

    fn disable_sound(&mut self) {
        if self.sdl_sound_device.status() == AudioStatus::Playing {
            self.sdl_sound_device.pause();
        }
    }
}
