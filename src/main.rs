use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Note frequency calculation functions
fn get_frequency_from_note(note: &str) -> Option<f32> {
    let note = note.to_uppercase();

    // Parse note and octave
    let (note_name, octave) = if note.len() >= 2 {
        let octave_str = &note[note.len() - 1..];
        let octave: i32 = octave_str.parse().ok()?;
        let note_name = &note[..note.len() - 1];
        (note_name, octave)
    } else {
        return None;
    };

    // Note to semitone mapping (C = 0)
    let semitone = match note_name {
        "C" => 0,
        "C#" | "DB" => 1,
        "D" => 2,
        "D#" | "EB" => 3,
        "E" => 4,
        "F" => 5,
        "F#" | "GB" => 6,
        "G" => 7,
        "G#" | "AB" => 8,
        "A" => 9,
        "A#" | "BB" => 10,
        "B" => 11,
        _ => return None,
    };

    // Calculate frequency using A4 = 440 Hz as reference
    // Formula: f = 440 * 2^((n-69)/12) where n is MIDI note number
    let midi_note = (octave + 1) * 12 + semitone - 12; // Adjusted for our octave numbering
    let frequency = 440.0 * 2.0_f32.powf((midi_note as f32 - 69.0) / 12.0);

    Some(frequency)
}

fn get_frequency_and_volume(keycode: Keycode) -> Option<(f32, f32, &'static str)> {
    let (note, volume) = match keycode {
        // Most common programming letters - pleasant pentatonic scale
        Keycode::E => ("E4", 0.3), // Very common
        Keycode::T => ("G4", 0.3), // Very common
        Keycode::A => ("C4", 0.3), // Most common
        Keycode::O => ("D4", 0.3), // Very common
        Keycode::I => ("A4", 0.3), // Very common
        Keycode::N => ("E5", 0.3), // Very common
        Keycode::S => ("G5", 0.3), // Very common
        Keycode::H => ("C5", 0.3), // Very common
        Keycode::R => ("D5", 0.3), // Very common

        // Second tier common letters
        Keycode::L => ("F4", 0.25), // Common
        Keycode::U => ("A3", 0.25), // Common
        Keycode::D => ("F5", 0.25), // Common
        Keycode::C => ("B4", 0.25), // Common
        Keycode::M => ("B3", 0.25), // Common

        // Less common letters - still harmonious
        Keycode::F => ("C3", 0.2),
        Keycode::P => ("D3", 0.2),
        Keycode::B => ("E3", 0.2),
        Keycode::V => ("G3", 0.2),
        Keycode::K => ("A5", 0.2),
        Keycode::W => ("F3", 0.2),
        Keycode::Y => ("B5", 0.2),
        Keycode::G => ("C6", 0.2),
        Keycode::J => ("D6", 0.2),
        Keycode::Q => ("E6", 0.2),
        Keycode::X => ("F6", 0.2),
        Keycode::Z => ("G6", 0.2),

        // Numbers - same scale as common letters for consistency
        Keycode::Key0 => ("C4", 0.25),
        Keycode::Key1 => ("E4", 0.25),
        Keycode::Key2 => ("G4", 0.25),
        Keycode::Key3 => ("A4", 0.25),
        Keycode::Key4 => ("D4", 0.25),
        Keycode::Key5 => ("F4", 0.25),
        Keycode::Key6 => ("C5", 0.25),
        Keycode::Key7 => ("E5", 0.25),
        Keycode::Key8 => ("G5", 0.25),
        Keycode::Key9 => ("A5", 0.25),

        // Programming symbols - gentle harmonics
        Keycode::Semicolon => ("C4", 0.2),
        Keycode::LeftBracket => ("E4", 0.2),
        Keycode::RightBracket => ("G4", 0.2),
        Keycode::Comma => ("A4", 0.2),
        Keycode::Dot => ("D4", 0.2),
        Keycode::Slash => ("F4", 0.2),
        Keycode::BackSlash => ("B4", 0.2),
        Keycode::Apostrophe => ("C5", 0.2),
        Keycode::Equal => ("D5", 0.2),
        Keycode::Minus => ("E5", 0.2),

        // Common keys - quiet to not disrupt
        Keycode::Space => ("C3", 0.1),
        Keycode::Backspace => ("G2", 0.1),
        Keycode::Enter => ("C3", 0.1),
        Keycode::Tab => ("F2", 0.1),
        Keycode::Delete => ("A2", 0.1),

        // Modifiers - very quiet
        Keycode::LShift => ("C2", 0.05),
        Keycode::RShift => ("E2", 0.05),
        Keycode::LControl => ("G2", 0.05),
        Keycode::RControl => ("A2", 0.05),
        Keycode::LAlt => ("D2", 0.05),
        Keycode::RAlt => ("F2", 0.05),
        Keycode::CapsLock => ("B1", 0.05),
        Keycode::Escape => ("C2", 0.05),

        // Navigation - comfortable low range
        Keycode::Up => ("E3", 0.15),
        Keycode::Down => ("D3", 0.15),
        Keycode::Left => ("C3", 0.15),
        Keycode::Right => ("G3", 0.15),
        Keycode::Home => ("C3", 0.15),
        Keycode::End => ("G3", 0.15),
        Keycode::PageUp => ("E3", 0.15),
        Keycode::PageDown => ("A3", 0.15),

        // Function keys - bright harmonics
        Keycode::F1 => ("C6", 0.2),
        Keycode::F2 => ("D6", 0.2),
        Keycode::F3 => ("E6", 0.2),
        Keycode::F4 => ("F6", 0.2),
        Keycode::F5 => ("G6", 0.2),
        Keycode::F6 => ("A6", 0.2),
        Keycode::F7 => ("B6", 0.2),
        Keycode::F8 => ("C7", 0.2),
        Keycode::F9 => ("D7", 0.2),
        Keycode::F10 => ("E7", 0.2),
        Keycode::F11 => ("F7", 0.2),
        Keycode::F12 => ("G7", 0.2),

        _ => return None,
    };

    get_frequency_from_note(note).map(|freq| (freq, volume, note))
}

// Waveform types for different tones
#[derive(Clone, Copy, Debug)]
enum Waveform {
    Natural,    // Complex harmonic piano-like tone
    Electronic, // Pure sine wave
    Saw,        // Sawtooth wave for electronic feel
    Square,     // Square wave for retro electronic
    Cyberpunk,  // Blade Runner 2049 style analog synth
}

impl Waveform {
    fn generate_sample(&self, phase: f32, frequency: f32, sample_rate: f32) -> f32 {
        let base_phase = phase * 2.0 * std::f32::consts::PI;

        match self {
            Waveform::Electronic => {
                // Pure sine wave
                base_phase.sin()
            }
            Waveform::Natural => {
                // Piano-like tone with harmonics
                let fundamental = base_phase.sin();
                let harmonic2 = (base_phase * 2.0).sin() * 0.3;
                let harmonic3 = (base_phase * 3.0).sin() * 0.15;
                let harmonic4 = (base_phase * 4.0).sin() * 0.08;
                let harmonic5 = (base_phase * 5.0).sin() * 0.05;

                // Add slight frequency modulation for natural variation
                let vibrato = (phase * 6.0 * 2.0 * std::f32::consts::PI).sin() * 0.002;
                let modulated_phase = base_phase * (1.0 + vibrato);

                fundamental
                    + harmonic2
                    + harmonic3
                    + harmonic4
                    + harmonic5
                    + modulated_phase.sin() * 0.02
            }
            Waveform::Saw => {
                // Sawtooth wave
                2.0 * (phase - phase.floor()) - 1.0
            }
            Waveform::Square => {
                // Square wave
                if (phase % 1.0) < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Cyberpunk => {
                // Blade Runner 2049 style analog synth
                let time = phase * sample_rate / frequency;

                // Base oscillators: warm saw + sub bass
                let saw = 2.0 * (phase - phase.floor()) - 1.0;
                let sub_bass = (base_phase * 0.5).sin() * 0.4; // Sub octave

                // LFO for analog character (slow modulation)
                let lfo = (time * 0.3).sin();
                let lfo2 = (time * 0.7).sin();

                // PWM (Pulse Width Modulation) for analog warmth
                let pulse_width = 0.5 + lfo * 0.1;
                let pulse = if (phase % 1.0) < pulse_width {
                    1.0
                } else {
                    -1.0
                };

                // Mix oscillators
                let mixed = saw * 0.6 + pulse * 0.3 + sub_bass;

                // Analog-style low-pass filter (simple approximation)
                let cutoff_mod = 0.7 + lfo2 * 0.2;
                let filtered = mixed * cutoff_mod;

                // Subtle distortion for warmth
                let driven = filtered * 1.2;
                let saturated = if driven > 0.8 {
                    0.8 + (driven - 0.8) * 0.3
                } else if driven < -0.8 {
                    -0.8 + (driven + 0.8) * 0.3
                } else {
                    driven
                };

                // Add slight detuning chorus effect
                let detune1 = (base_phase * 1.003).sin() * 0.15;
                let detune2 = (base_phase * 0.997).sin() * 0.15;

                saturated + detune1 + detune2
            }
        }
    }
}

// Audio state management
#[derive(Clone)]
enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone)]
struct ADSRParams {
    attack_time: f32,   // seconds
    decay_time: f32,    // seconds
    sustain_level: f32, // 0.0 to 1.0
    release_time: f32,  // seconds
}

struct NoteState {
    frequency: f32,
    base_volume: f32,
    phase: f32,
    envelope_state: EnvelopeState,
    envelope_time: f32,
    adsr: ADSRParams,
    waveform: Waveform,
}

struct AudioState {
    active_notes: HashMap<Keycode, NoteState>,
    sample_rate: f32,
    default_adsr: ADSRParams,
    current_waveform: Waveform,
}

impl AudioState {
    fn new(sample_rate: f32) -> Self {
        Self {
            active_notes: HashMap::new(),
            sample_rate,
            default_adsr: ADSRParams {
                attack_time: 0.02,  // 20ms attack
                decay_time: 0.1,    // 100ms decay
                sustain_level: 0.7, // 70% sustain level
                release_time: 0.15, // 150ms release
            },
            current_waveform: Waveform::Natural,
        }
    }

    fn set_waveform(&mut self, waveform: Waveform) {
        self.current_waveform = waveform;

        // Update ADSR parameters based on waveform for authentic sound
        match waveform {
            Waveform::Cyberpunk => {
                // Blade Runner 2049 style: slower attack, longer release
                self.default_adsr = ADSRParams {
                    attack_time: 0.08,  // 80ms - analog synth pad feel
                    decay_time: 0.3,    // 300ms - longer decay
                    sustain_level: 0.6, // 60% - softer sustain
                    release_time: 0.4,  // 400ms - long analog tail
                };
            }
            Waveform::Natural => {
                // Piano-like response
                self.default_adsr = ADSRParams {
                    attack_time: 0.02,
                    decay_time: 0.1,
                    sustain_level: 0.7,
                    release_time: 0.15,
                };
            }
            Waveform::Electronic => {
                // Clean electronic response
                self.default_adsr = ADSRParams {
                    attack_time: 0.01,
                    decay_time: 0.05,
                    sustain_level: 0.8,
                    release_time: 0.1,
                };
            }
            Waveform::Saw | Waveform::Square => {
                // Punchy electronic
                self.default_adsr = ADSRParams {
                    attack_time: 0.005,
                    decay_time: 0.08,
                    sustain_level: 0.75,
                    release_time: 0.12,
                };
            }
        }

        println!("🎵 Switched to {:?} waveform", waveform);
    }

    fn generate_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        let dt = 1.0 / self.sample_rate;
        let mut to_remove = Vec::new();

        for (keycode, note_state) in self.active_notes.iter_mut() {
            // Update envelope time
            note_state.envelope_time += dt;

            // Calculate ADSR envelope multiplier
            let envelope_multiplier = match note_state.envelope_state {
                EnvelopeState::Attack => {
                    if note_state.envelope_time >= note_state.adsr.attack_time {
                        note_state.envelope_state = EnvelopeState::Decay;
                        note_state.envelope_time = 0.0;
                        1.0
                    } else {
                        // Exponential attack curve for more natural sound
                        let progress = note_state.envelope_time / note_state.adsr.attack_time;
                        progress * progress // Square for exponential curve
                    }
                }
                EnvelopeState::Decay => {
                    if note_state.envelope_time >= note_state.adsr.decay_time {
                        note_state.envelope_state = EnvelopeState::Sustain;
                        note_state.envelope_time = 0.0;
                        note_state.adsr.sustain_level
                    } else {
                        let progress = note_state.envelope_time / note_state.adsr.decay_time;
                        // Exponential decay from 1.0 to sustain_level
                        1.0 - (1.0 - note_state.adsr.sustain_level) * progress * progress
                    }
                }
                EnvelopeState::Sustain => note_state.adsr.sustain_level,
                EnvelopeState::Release => {
                    if note_state.envelope_time >= note_state.adsr.release_time {
                        to_remove.push(*keycode);
                        0.0
                    } else {
                        let progress = note_state.envelope_time / note_state.adsr.release_time;
                        // Exponential release curve
                        note_state.adsr.sustain_level * (1.0 - progress * progress)
                    }
                }
            };

            // Generate sample for this note using selected waveform
            let wave_sample = note_state.waveform.generate_sample(
                note_state.phase,
                note_state.frequency,
                self.sample_rate,
            );
            let note_sample = wave_sample * note_state.base_volume * envelope_multiplier;

            sample += note_sample;

            // Update phase for this note
            note_state.phase += note_state.frequency / self.sample_rate;
            if note_state.phase >= 1.0 {
                note_state.phase -= 1.0;
            }
        }

        // Remove fully released notes
        for keycode in to_remove {
            self.active_notes.remove(&keycode);
        }

        sample * 0.3 // Global volume adjustment
    }

    fn start_note(&mut self, keycode: Keycode, frequency: f32, volume: f32) {
        let note_state = NoteState {
            frequency,
            base_volume: volume,
            phase: 0.0,
            envelope_state: EnvelopeState::Attack,
            envelope_time: 0.0,
            adsr: self.default_adsr.clone(),
            waveform: self.current_waveform,
        };
        self.active_notes.insert(keycode, note_state);
    }

    fn stop_note(&mut self, keycode: Keycode) {
        if let Some(note_state) = self.active_notes.get_mut(&keycode) {
            if !matches!(note_state.envelope_state, EnvelopeState::Release) {
                note_state.envelope_state = EnvelopeState::Release;
                note_state.envelope_time = 0.0;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Check for waveform argument
    let initial_waveform = if args.len() > 1 {
        match args[1].as_str() {
            "natural" => Waveform::Natural,
            "electronic" => Waveform::Electronic,
            "saw" => Waveform::Saw,
            "square" => Waveform::Square,
            "cyberpunk" => Waveform::Cyberpunk,
            _ => {
                println!("Available waveforms: natural, electronic, saw, square, cyberpunk");
                println!("Using default: natural");
                Waveform::Natural
            }
        }
    } else {
        Waveform::Natural
    };
    // Initialize audio
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;

    let mut audio_state_init = AudioState::new(sample_rate);
    audio_state_init.set_waveform(initial_waveform);
    let audio_state = Arc::new(Mutex::new(audio_state_init));
    let audio_state_clone = audio_state.clone();

    // Create audio stream
    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut state = audio_state_clone.lock().unwrap();
            for sample in data.iter_mut() {
                *sample = state.generate_sample();
            }
        },
        |err| eprintln!("Audio stream error: {}", err),
        None,
    )?;

    stream.play()?;

    // Interactive keyboard mode
    println!(
        "🎹 Piano Keyboard Sound Simulator - {:?} Mode",
        initial_waveform
    );
    println!();
    println!("IMPORTANT - macOS Permission Required:");
    println!("If this fails, you need to grant accessibility permissions:");
    println!("1. Go to: System Preferences > Security & Privacy > Privacy > Accessibility");
    println!("2. Click the lock and enter your password");
    println!("3. Add your Terminal app (Terminal.app, iTerm2, etc.)");
    println!("4. Restart this program");
    println!();
    println!("Piano keys (works from ANY window once permissions are granted):");
    println!();
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│              PROGRAMMING-OPTIMIZED PIANO LAYOUT                │");
    println!("├─────────────────────────────────────────────────────────────────┤");
    println!("│ ⌨️  OPTIMIZED FOR CODING - Harmonic mapping based on key usage   │");
    println!("│                                                                 │");
    println!("│ 🎵 Letters (C Major Pentatonic - Pleasant & Harmonious):       │");
    println!("│   E=E4  T=G4  A=C4  O=D4  I=A4  N=E5  S=G5  H=C5  R=D5        │");
    println!("│   L=F4  U=A3  D=F5  C=B4  M=B3  F=C3  P=D3  B=E3  V=G3        │");
    println!("│   K=A5  W=F3  Y=B5  G=C6  J=D6  Q=E6  X=F6  Z=G6              │");
    println!("│                                                                 │");
    println!("│ 🔢 Numbers (Gentle Harmony - Same as common letters):          │");
    println!("│   0=C4  1=E4  2=G4  3=A4  4=D4  5=F4  6=C5  7=E5  8=G5  9=A5  │");
    println!("│                                                                 │");
    println!("│ ⚡ Symbols (Programming-friendly harmonics):                    │");
    println!("│   ;=C4  [=E4  ]=G4  ,=A4  .=D4  /=F4  \\=B4  '=C5  ==D5  -=E5   │");
    println!("│                                                                 │");
    println!("│ 🔇 Common Keys (Quiet bass - won't disrupt flow):              │");
    println!("│   SPACE=C3  BACKSPACE=G2  ENTER=C3  TAB=F2  DELETE=A2         │");
    println!("│                                                                 │");
    println!("│ 🎛️  Modifiers (Very quiet - barely audible):                   │");
    println!("│   SHIFT=C2/E2  CTRL=G2/A2  ALT=D2/F2  CAPS=B1  ESC=C2         │");
    println!("│                                                                 │");
    println!("│ 🧭 Navigation (Comfortable low range):                         │");
    println!("│   ↑=E3  ↓=D3  ←=C3  →=G3  HOME/END=C3/G3  PG_UP/DN=E3/A3     │");
    println!("│                                                                 │");
    println!("│ 🔧 Function Keys (Bright harmonics for special actions):       │");
    println!("│   F1-F6=C6-A6  F7-F12=B6-G7 (Higher for advanced functions)   │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    println!();
    println!("🎹 Programming-Optimized Music Mapping!");
    println!("🎵 Based on key frequency analysis during coding");
    println!("🎼 Creates pleasant harmonies using C major pentatonic scale");
    println!("🔇 Common keys are quieter to avoid disrupting concentration");
    println!();
    println!("🎛️  Waveform Controls:");
    println!("   Press F9  = Natural piano tone (complex harmonics)");
    println!("   Press F10 = Electronic tone (pure sine wave)");
    println!("   Press F11 = Saw wave (bright electronic)");
    println!("   Press F12 = Square wave (retro electronic)");
    println!("   Press F8  = Cyberpunk 2049 style (analog synth)");
    println!();
    println!("💡 Command line options:");
    println!("   cargo run natural    # Start with natural piano");
    println!("   cargo run electronic # Start with electronic");
    println!("   cargo run saw        # Start with saw wave");
    println!("   cargo run square     # Start with square wave");
    println!("   cargo run cyberpunk  # Start with Blade Runner 2049 style");
    println!();
    println!("Press Ctrl+C to exit");
    println!();

    // Initialize device state for global keyboard input
    let device_state = DeviceState::new();
    let mut prev_keys: Vec<Keycode> = Vec::new();

    // Main keyboard polling loop
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        // Check for newly pressed keys
        for key in &keys {
            if !prev_keys.contains(key) {
                // Check for waveform switching keys
                match key {
                    Keycode::F8 => {
                        let mut audio_state = audio_state.lock().unwrap();
                        audio_state.set_waveform(Waveform::Cyberpunk);
                        continue;
                    }
                    Keycode::F9 => {
                        let mut audio_state = audio_state.lock().unwrap();
                        audio_state.set_waveform(Waveform::Natural);
                        continue;
                    }
                    Keycode::F10 => {
                        let mut audio_state = audio_state.lock().unwrap();
                        audio_state.set_waveform(Waveform::Electronic);
                        continue;
                    }
                    Keycode::F11 => {
                        let mut audio_state = audio_state.lock().unwrap();
                        audio_state.set_waveform(Waveform::Saw);
                        continue;
                    }
                    Keycode::F12 => {
                        let mut audio_state = audio_state.lock().unwrap();
                        audio_state.set_waveform(Waveform::Square);
                        continue;
                    }
                    _ => {}
                }

                if let Some((frequency, volume, note)) = get_frequency_and_volume(*key) {
                    let mut audio_state = audio_state.lock().unwrap();
                    audio_state.start_note(*key, frequency, volume);
                    println!(
                        "Playing: {:?} -> {} ({:.2} Hz, vol: {:.2})",
                        key, note, frequency, volume
                    );
                }
            }
        }

        // Check for released keys
        for key in &prev_keys {
            if !keys.contains(key) {
                if let Some((_, _, note)) = get_frequency_and_volume(*key) {
                    let mut audio_state = audio_state.lock().unwrap();
                    audio_state.stop_note(*key);
                    println!("Stopped: {:?} -> {}", key, note);
                }
            }
        }

        prev_keys = keys;
        thread::sleep(Duration::from_millis(10)); // Small delay to prevent excessive CPU usage
    }
}
