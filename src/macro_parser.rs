use std::time::Duration;

use hidapi::HidDevice;
use macropad_protocol::{macro_protocol::MacroCommand, data_protocol::{KeyMode, LedEffect}};
use usbd_human_interface_device::page::{Consumer, Keyboard};

use crate::macropad_wrapper::{self, prime_device};

#[derive(Debug, Clone)]
pub struct MacroFrame {
    pub actions: Vec<MacroAction>,
    pub delay: Option<Duration>,
}

impl Default for MacroFrame {
    fn default() -> Self {
        Self {
            actions: vec![],
            delay: None,
        }
    }
}

impl MacroFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(actions: Vec<MacroAction>, delay: Option<Duration>) -> Self {
        Self {
            actions,
            delay,
        }
    }

    pub fn add_action(&mut self, action: MacroAction) -> &mut Self {
        self.actions.push(action);
        self
    }
}

#[derive(Debug, Clone)]
pub enum MacroAction {
    PressKey(Keyboard),
    ReleaseKey(Keyboard),
    Consumer(Consumer),
    SetLed((u8, u8, u8)),
    ClearLed,
}

#[derive(Debug, Clone)]
pub struct Macro {
    pub frames: Vec<MacroFrame>,
}

impl Default for Macro {
    fn default() -> Self {
        Self {
            frames: vec![MacroFrame {
                actions: vec![],
                delay: None,
            }],
        }
    }
}

#[derive(Debug, Clone)]
pub struct MacroConfig {
    pub tap_speed: u32,
    pub hold_speed: u32,
    pub default_delay: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyConfig {
    pub key_mode: KeyMode,
    pub keyboard_data: Keyboard,
    pub consumer_data: Consumer,
    pub key_color: (u8, u8, u8),
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            key_mode: KeyMode::MacroMode,
            keyboard_data: Keyboard::NoEventIndicated,
            consumer_data: Consumer::Unassigned,
            key_color: (0, 0, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LedConfig {
    pub base_color: (u8, u8, u8),
    pub effect: LedEffect,
    pub brightness: u8,
    pub effect_period: f32,
    pub effect_offset: f32,
}

impl Default for LedConfig {
    fn default() -> Self {
        Self {
            base_color: (0, 0, 0),
            effect: LedEffect::None,
            brightness: 0xA0,
            effect_period: 1.0,
            effect_offset: 0.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MacroCollection {
    pub tap: Macro,
    pub hold: Macro,
    pub double_tap: Macro,
    pub tap_hold: Macro,
}

#[derive(Debug, Clone)]
pub struct Macropad {
    pub macros: Vec<MacroCollection>,
    pub config: MacroConfig,
    pub key_configs: Vec<KeyConfig>,
    pub led_config: LedConfig,
}

impl Macropad {
    pub fn set_macro(&mut self, index: usize, macro_data: Macro) {
        match index & 0b11 {
            0 => self.macros[index >> 2].tap = macro_data,
            1 => self.macros[index >> 2].hold = macro_data,
            2 => self.macros[index >> 2].double_tap = macro_data,
            3 => self.macros[index >> 2].tap_hold = macro_data,
            _ => (),
        }
    }
}

pub fn get_key_config(device: &HidDevice, index: u8) -> Result<KeyConfig, ()> {
    let key_mode =  macropad_wrapper::get_key_mode(device, index)?;
    let keyboard_data = macropad_wrapper::get_keyboard_data(device, index)?;
    let consumer_data = macropad_wrapper::get_consumer_data(device, index)?;
    let key_color = macropad_wrapper::get_key_color(device, index)?;


    Ok(KeyConfig { key_mode, keyboard_data, consumer_data, key_color})
}

pub fn get_macro(device: &HidDevice, index: u8) -> Result<Macro, ()> {
    let data = macropad_wrapper::get_macro(device, index)?;
    Ok(parse_macro(&data))
}

pub fn get_macro_collection(device: &HidDevice, index: u8) -> Result<MacroCollection, ()> {
    let mut collection = MacroCollection::default();

    for m in 0..4 {
        let data = macropad_wrapper::get_macro(device, (index << 2) | m)?;
        let macro_data = parse_macro(&data);
        match m {
            0 => collection.tap = macro_data,
            1 => collection.hold = macro_data,
            2 => collection.double_tap = macro_data,
            3 => collection.tap_hold = macro_data,
            _ => (),
        }
    }

    Ok(collection)
}

pub fn get_config(device: &HidDevice) -> Result<MacroConfig, ()> {
    let tap_speed = macropad_wrapper::get_tap_speed(device)?;
    let hold_speed = macropad_wrapper::get_hold_speed(device)?;
    let default_delay = macropad_wrapper::get_default_delay(device)?;

    Ok(MacroConfig {
        tap_speed,
        hold_speed,
        default_delay,
    })
}

pub fn get_led_config(device: &HidDevice) -> Result<LedConfig, ()> {
    let base_color = macropad_wrapper::get_led_base_color(device)?;
    let effect = macropad_wrapper::get_led_effect(device)?;
    let brightness = macropad_wrapper::get_led_brightness(device)?;
    let effect_period = macropad_wrapper::get_led_effect_period(device)?;
    let effect_offset = macropad_wrapper::get_led_effect_offset(device)?;

    Ok(LedConfig {
        base_color,
        effect,
        brightness,
        effect_period,
        effect_offset,
    })
}

pub fn get_macro_pad(device: &HidDevice) -> Result<Macropad, ()> {
    prime_device(device)?;
    let mut macros = Vec::new();
    let config = get_config(device)?;
    let mut key_configs = Vec::new();
    let led_config = get_led_config(device)?;

    for index in 0..4 {
        macros.push(get_macro_collection(device, index)?);
        key_configs.push(get_key_config(device, index)?);
    }

    Ok(Macropad { macros, config, key_configs, led_config })
}

pub fn parse_macro(data: &[u8; 4092]) -> Macro {
    let mut frames = Vec::new();

    let mut i = 0;
    let mut command = MacroCommand::from(data[i]);
    while command != MacroCommand::CommandTerminator {
        let mut actions = Vec::new();
        let mut delay = None;
        while command != MacroCommand::CommandTerminator {
            match command {
                MacroCommand::CommandDelay => {
                    let delay_bytes = [data[i + 1], data[i + 2], data[i + 3], data[i + 4]];
                    let delay_millis = u32::from_le_bytes(delay_bytes);
                    delay = Some(Duration::from_micros(delay_millis as u64));
                    i += 5;
                }
                MacroCommand::CommandPressKey => {
                    let key = Keyboard::from(data[i + 1]);
                    actions.push(MacroAction::PressKey(key));
                    i += 2;
                }
                MacroCommand::CommandReleaseKey => {
                    let key = Keyboard::from(data[i + 1]);
                    actions.push(MacroAction::ReleaseKey(key));
                    i += 2;
                }
                MacroCommand::CommandConsumer => {
                    let key = Consumer::from(u16::from_le_bytes([data[i + 1], data[i + 2]]));
                    actions.push(MacroAction::Consumer(key));
                    i += 3;
                }
                MacroCommand::CommandSetLed => {
                    let r = data[i + 1];
                    let g = data[i + 2];
                    let b = data[i + 3];
                    actions.push(MacroAction::SetLed((r, g, b)));
                    i += 4;
                }
                MacroCommand::CommandClearLed => {
                    actions.push(MacroAction::ClearLed);
                    i += 1;
                }
                _ => {}
            }
            command = MacroCommand::from(data[i]);
        }
        frames.push(MacroFrame {
            actions,
            delay,
        });
        i += 1;
        command = MacroCommand::from(data[i]);
    }

    Macro {
        frames,
    }
}

impl Macro {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
        }
    }

    pub fn add_frame(&mut self, frame: MacroFrame) {
        self.frames.push(frame);
    }

    pub fn pack(&self) -> [u8; 4092] {
        let mut data = [0; 4092];

        let mut i = 0;
        for frame in self.frames.iter() {
            for action in frame.actions.iter() {
                match action {
                    MacroAction::PressKey(key) => {
                        data[i] = MacroCommand::CommandPressKey as u8;
                        data[i + 1] = *key as u8;
                        i += 2;
                    },
                    MacroAction::ReleaseKey(key) => {
                        data[i] = MacroCommand::CommandReleaseKey as u8;
                        data[i + 1] = *key as u8;
                        i += 2;
                    },
                    MacroAction::Consumer(consumer) => {
                        data[i] = MacroCommand::CommandConsumer as u8;
                        let consumer_bytes = (*consumer as u16).to_le_bytes();
                        data[i + 1] = consumer_bytes[0];
                        data[i + 2] = consumer_bytes[1];
                        i += 3;
                    },
                    MacroAction::SetLed(key) => {
                        data[i] = MacroCommand::CommandSetLed as u8;
                        data[i + 1] = key.0;
                        data[i + 2] = key.1;
                        data[i + 3] = key.2;
                        i += 4;
                    },
                    MacroAction::ClearLed => {
                        data[i] = MacroCommand::CommandClearLed as u8;
                        i += 1;
                    },
                }
            }

            if let Some(delay) = frame.delay {
                data[i] = MacroCommand::CommandDelay as u8;
                let delay_bytes = delay.as_micros().to_le_bytes();
                data[i + 1] = delay_bytes[0];
                data[i + 2] = delay_bytes[1];
                data[i + 3] = delay_bytes[2];
                data[i + 4] = delay_bytes[3];
                i += 5;
            }

            data[i] = MacroCommand::CommandTerminator as u8;
            i += 1;
        }

        data
    }
}