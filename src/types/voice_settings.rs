//! Interface for viewing and updating voice settings
//! via [`DiscordIpc::get_voice_settings`](crate::DiscordIpc::get_voice_settings)
//! and [`DiscordIpc::set_voice_settings`](crate::DiscordIpc::set_voice_settings).

use serde_derive::{Deserialize, Serialize};

/// Discord voice settings. See [Discord RPC docs](https://discord.com/developers/docs/topics/rpc#getvoicesettings-get-voice-settings-response-structure) for details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VoiceSettings {
    /// Voice Input Settings
    pub input: Option<VoiceInputSettings>,
    /// Voice Output Settings
    pub output: Option<VoiceOutputSettings>,
    /// Voice Mode Settings
    pub mode: Option<VoiceModeSettings>,
    /// State of automatic gain control
    pub automatic_gain_control: Option<bool>,
    /// State of echo cancellation
    pub echo_cancellation: Option<bool>,
    /// State of noise suppression
    pub noise_suppression: Option<bool>,
    /// State of "quality of service" setting
    pub qos: Option<bool>,
    /// State of silence warning
    pub silence_warning: Option<bool>,
    /// State of deafening
    pub deaf: Option<bool>,
    /// State of muting
    pub mute: Option<bool>,
}
impl VoiceSettings {
    /// Creates a new empty `VoiceSettings`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the Voice Input Settings.
    pub fn input(mut self, input: VoiceInputSettings) -> Self {
        self.input = Some(input);
        self
    }

    /// Sets the Voice Output Settings.
    pub fn output(mut self, output: VoiceOutputSettings) -> Self {
        self.output = Some(output);
        self
    }

    /// Sets the Voice Mode Settings.
    pub fn mode(mut self, mode: VoiceModeSettings) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Sets the state of automatic gain control.
    pub fn automatic_gain_control(mut self, automatic_gain_control: bool) -> Self {
        self.automatic_gain_control = Some(automatic_gain_control);
        self
    }

    /// Sets the state of echo cancellation.
    pub fn echo_cancellation(mut self, echo_cancellation: bool) -> Self {
        self.echo_cancellation = Some(echo_cancellation);
        self
    }

    /// Sets the state of noise suppression.
    pub fn noise_suppression(mut self, noise_suppression: bool) -> Self {
        self.noise_suppression = Some(noise_suppression);
        self
    }

    /// Sets the state of "quality of service" setting.
    pub fn qos(mut self, qos: bool) -> Self {
        self.qos = Some(qos);
        self
    }

    /// Sets the state of silence warning.
    pub fn silence_warning(mut self, silence_warning: bool) -> Self {
        self.silence_warning = Some(silence_warning);
        self
    }

    /// Sets the state of deafening.
    ///
    /// Note that using deafening may require you to use both deaf and mute fields.
    pub fn deaf(mut self, deaf: bool) -> Self {
        self.deaf = Some(deaf);
        self
    }

    /// Sets the state of muting.
    pub fn mute(mut self, mute: bool) -> Self {
        self.mute = Some(mute);
        self
    }
}

/// Voice input settings. See [Discord RPC docs](https://discord.com/developers/docs/topics/rpc#getvoicesettings-voice-settings-input-object) for details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VoiceInputSettings {
    /// Device ID of the current input device
    pub device_id: Option<String>,
    /// Input voice level percentage (min: 0, max: 100)
    pub volume: Option<f32>,
    /// List of available input devices
    pub available_devices: Option<Vec<VoiceAvailableDevice>>,
}
impl VoiceInputSettings {
    /// Creates a new empty `VoiceInputSettings`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the input device by its ID.
    pub fn device_id(mut self, device_id: impl ToString) -> Self {
        self.device_id = Some(device_id.to_string());
        self
    }

    /// Sets the volume of the device.
    pub fn volume(mut self, volume: f32) -> Self {
        self.volume = Some(volume);
        self
    }
}

/// Voice output settings. See [Discord RPC docs](https://discord.com/developers/docs/topics/rpc#getvoicesettings-voice-settings-output-object) for details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VoiceOutputSettings {
    /// Device ID of the current output device
    pub device_id: Option<String>,
    /// Output voice level percentage (min: 0, max: 200)
    pub volume: Option<f32>,
    /// List of available output devices
    pub available_devices: Option<Vec<VoiceAvailableDevice>>,
}
impl VoiceOutputSettings {
    /// Creates a new empty `VoiceOutputSettings`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the input device by its ID.
    pub fn device_id(mut self, device_id: impl ToString) -> Self {
        self.device_id = Some(device_id.to_string());
        self
    }

    /// Sets the volume of the device.
    pub fn volume(mut self, volume: f32) -> Self {
        self.volume = Some(volume);
        self
    }
}

/// An available audio device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VoiceAvailableDevice {
    /// ID of device for setting
    pub id: String,
    /// Name of device for displaying
    pub name: String,
}

/// Voice mode settings. See [Discord RPC docs](https://discord.com/developers/docs/topics/rpc#getvoicesettings-voice-settings-mode-object) for details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VoiceModeSettings {
    #[serde(rename = "type")]
    /// Voice Mode setting
    pub voice_mode: Option<VoiceMode>,
    /// whether the Voice Activity threshold is set automatically
    pub auto_threshold: Option<bool>,
    /// Voice Activity threshold (in dB) (min: -100, max: 0)
    pub threshold: Option<f32>,
    /// Shortcut Key Combo for activating this voice mode
    pub shortcut: Option<Vec<ShortcutKeyCombo>>,
    /// Push-to-Talk release delay (in ms) (min: 0, max: 2000)
    pub delay: Option<f32>,
}
impl VoiceModeSettings {
    /// Creates a new empty `VoiceModeSettings`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the state of voice mode.
    pub fn voice_mode(mut self, voice_mode: VoiceMode) -> Self {
        self.voice_mode = Some(voice_mode);
        self
    }

    /// Sets the state of automatic voice activity threshold.
    pub fn auto_threshold(mut self, auto_threshold: bool) -> Self {
        self.auto_threshold = Some(auto_threshold);
        self
    }

    /// Sets the voice activity threshold. Overridden by [`Self::auto_threshold`](Self::auto_threshold).
    pub fn threshold(mut self, threshold: f32) -> Self {
        self.threshold = Some(threshold);
        self
    }

    /// Sets the push-to-talk release delay.
    pub fn delay(mut self, delay: f32) -> Self {
        self.delay = Some(delay);
        self
    }
}

/// Voice mode.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VoiceMode {
    /// Push to Talk
    PushToTalk,
    /// Voice Activity
    VoiceActivity,
}

/// A shortcut key combo.
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ShortcutKeyCombo {
    #[serde(rename = "type")]
    /// Key Type of shortcut combo
    pub key_type: ShortcutKeyType,
    /// Key scan code
    pub code: usize,
    /// Key name
    pub name: String,
}
impl ShortcutKeyCombo {
    /// Creates a new `ShortcutKeyCombo`.
    pub fn new(key_type: ShortcutKeyType, code: usize, name: impl ToString) -> Self {
        Self {
            key_type: key_type,
            code: code,
            name: name.to_string(),
        }
    }
}

/// Shortcut key type for a shortcut key combo.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum ShortcutKeyType {
    /// Keyboard Key
    KeyboardKey = 0,
    /// Mouse Button
    MouseButton = 1,
    /// Keyboard Modifier Key (Shift, Ctrl, Alt, Super)
    KeyboardModifierKey = 2,
    /// Gamepad Button
    GamepadButton = 3,
}
