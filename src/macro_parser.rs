use std::time::Duration;

use hidapi::HidDevice;
use macropad_protocol::{
    data_protocol::{KeyMode, LedEffect},
    macro_protocol::MacroCommand,
};
use semver::Version;
use usbd_human_interface_device::page::{Consumer, Keyboard};

use crate::{
    hid_manager::MacropadCommand,
    macropad_wrapper::{self, prime_device},
    type_wrapper,
};

#[derive(Debug, Clone)]
pub enum ActionType {
    Empty,
    SetLed((u8, u8, u8)),
    ClearLed,
    KeyDown(Keyboard),
    KeyUp(Keyboard),
    KeyPress(Keyboard, Duration),
    ConsumerPress(Consumer, Duration),
    String(String, Duration),
    Chord(Vec<Keyboard>, Duration),
    Loop(Vec<MacroFrame>, Duration, u8),
}

#[derive(Debug, Clone)]
pub struct MacroFrame {
    pub action: ActionType,
    pub delay: Duration,
}

impl Default for MacroFrame {
    fn default() -> Self {
        Self {
            action: ActionType::Empty,
            delay: Duration::ZERO,
        }
    }
}

impl MacroFrame {
    pub fn new() -> Self {
        Self::default()
    }

    fn add_command(command: MacroCommand, delay: &Duration, output: &mut Vec<u8>) {
        let delay = (delay.as_micros() as u32).to_le_bytes();

        let delay_count = if delay[3] == 0 {
            if delay[2] == 0 {
                if delay[1] == 0 {
                    0
                } else {
                    1
                }
            } else {
                2
            }
        } else {
            3
        };

        output.push(((command as u8) << 2) | delay_count);

        for i in 0..(delay_count + 1) {
            output.push(delay[i as usize]);
        }
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut output = Vec::new();

        match &self.action {
            ActionType::Empty => {
                MacroFrame::add_command(MacroCommand::Empty, &self.delay, &mut output)
            }
            ActionType::SetLed((r, g, b)) => {
                MacroFrame::add_command(MacroCommand::SetLed, &self.delay, &mut output);
                output.push(*r);
                output.push(*g);
                output.push(*b);
            }
            ActionType::ClearLed => {
                MacroFrame::add_command(MacroCommand::ClearLed, &self.delay, &mut output);
            }
            ActionType::KeyDown(key) => {
                MacroFrame::add_command(MacroCommand::KeyDown, &self.delay, &mut output);
                output.push(*key as u8);
            }
            ActionType::KeyUp(key) => {
                MacroFrame::add_command(MacroCommand::KeyUp, &self.delay, &mut output);
                output.push(*key as u8);
            }
            ActionType::KeyPress(key, delay) => {
                MacroFrame::add_command(MacroCommand::KeyPress, &self.delay, &mut output);
                output.push(*key as u8);

                output.extend_from_slice(&(delay.as_micros() as u32).to_le_bytes());
            }
            ActionType::ConsumerPress(consumer, delay) => {
                MacroFrame::add_command(MacroCommand::ConsumerPress, &self.delay, &mut output);
                output.extend_from_slice(&(*consumer as u16).to_le_bytes());

                output.extend_from_slice(&(delay.as_micros() as u32).to_le_bytes());
            }
            ActionType::String(string, delay) => {
                MacroFrame::add_command(MacroCommand::TypeString, &self.delay, &mut output);
                output.extend_from_slice(&(delay.as_micros() as u32).to_le_bytes());
                output.extend_from_slice(string.as_bytes());
                output.push(0);
            }
            ActionType::Chord(keys, delay) => {
                MacroFrame::add_command(MacroCommand::Chord, &self.delay, &mut output);
                output.extend_from_slice(&(delay.as_micros() as u32).to_le_bytes());
                for key in keys {
                    output.push(*key as u8);
                }
                output.push(0);
            }
            ActionType::Loop(frames, delay, count) => {
                MacroFrame::add_command(MacroCommand::LoopBegin, delay, &mut output);
                for frame in frames {
                    output.extend_from_slice(&frame.pack());
                }
                MacroFrame::add_command(MacroCommand::LoopEnd, &self.delay, &mut output);
                output.push(*count);
            }
        }

        output
    }
}

#[derive(Debug, Clone)]
pub enum MacroType {
    Tap,
    Hold,
    DoubleTap,
    TapHold,
}

#[derive(Debug, Clone)]
pub struct Macro {
    pub frames: Vec<MacroFrame>,
}

impl Default for Macro {
    fn default() -> Self {
        Self { frames: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct MacroConfig {
    pub tap_speed: u32,
    pub hold_speed: u32,
}

impl Default for MacroConfig {
    fn default() -> Self {
        Self {
            tap_speed: 200,
            hold_speed: 200,
        }
    }
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

#[derive(Debug, Clone, Default)]
pub struct BuildInfo {
    pub firmware_version: String,
    pub build_date: String,
    pub build_timestamp: String,
    pub build_profile: String,
    pub git_hash: String,
    pub git_branch: String,
    pub git_semver: String,
}

#[derive(Debug, Clone)]
pub struct Macropad {
    pub version: Version,
    pub macros: Vec<MacroCollection>,
    pub config: MacroConfig,
    pub key_configs: Vec<KeyConfig>,
    pub led_config: LedConfig,
    pub build_info: BuildInfo,
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
    let key_mode = macropad_wrapper::get_key_mode(device, index)?;
    let keyboard_data = macropad_wrapper::get_keyboard_data(device, index)?;
    let consumer_data = macropad_wrapper::get_consumer_data(device, index)?;
    let key_color = macropad_wrapper::get_key_color(device, index)?;

    Ok(KeyConfig {
        key_mode,
        keyboard_data,
        consumer_data,
        key_color,
    })
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

    Ok(MacroConfig {
        tap_speed,
        hold_speed,
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

pub fn get_build_info(device: &HidDevice) -> Result<BuildInfo, ()> {
    let firmware_version = macropad_wrapper::get_firmware_version(device)?;
    let build_date = macropad_wrapper::get_build_date(device)?;
    let build_timestamp = macropad_wrapper::get_build_timestamp(device)?;
    let build_profile = macropad_wrapper::get_build_profile(device)?;
    let git_hash = macropad_wrapper::get_git_hash(device)?;
    let git_branch = macropad_wrapper::get_git_branch(device)?;
    let git_semver = macropad_wrapper::get_git_semver(device)?;

    Ok(BuildInfo {
        firmware_version,
        build_date,
        build_timestamp,
        build_profile,
        git_hash,
        git_branch,
        git_semver,
    })
}

pub fn get_macro_pad(device: &HidDevice) -> Result<Macropad, ()> {
    prime_device(device)?;
    let mut macros = Vec::new();
    let config = get_config(device)?;
    let mut key_configs = Vec::new();
    let led_config = get_led_config(device)?;
    let build_info = get_build_info(device)?;

    for index in 0..4 {
        macros.push(get_macro_collection(device, index)?);
        key_configs.push(get_key_config(device, index)?);
    }

    let version = Version::parse(&build_info.git_semver).unwrap();

    Ok(Macropad {
        version,
        macros,
        config,
        key_configs,
        led_config,
        build_info,
    })
}

pub fn parse_macro(data: &[u8; 4092]) -> Macro {
    let mut frames = Vec::new();
    let mut parents = Vec::new();

    let mut offset = 0;
    let mut command = MacroCommand::Empty;
    let mut delay = 0;
    let mut delay_bytes = [0; 4];
    let mut delay_bytes_count = 0;

    let mut action: Option<ActionType> = None;
    let mut caps = false;

    while data[offset] != MacroCommand::Empty as u8 {
        command = MacroCommand::from(data[offset] >> 2);
        delay_bytes_count = (data[offset] & 0b11) + 1;

        offset += 1;

        delay_bytes = [0; 4];
        delay_bytes[0..delay_bytes_count as usize]
            .copy_from_slice(&data[offset..offset + delay_bytes_count as usize]);
        offset += delay_bytes_count as usize;
        delay = u32::from_le_bytes(delay_bytes);

        let frame_list = if parents.is_empty() {
            &mut frames
        } else {
            let mut temp_parents = parents.clone();
            let mut output = &mut frames;
            while !temp_parents.is_empty() {
                let parent_index = temp_parents.remove(0);
                match &mut output[parent_index] {
                    MacroFrame {
                        action: ActionType::Loop(loop_frames, _, _),
                        ..
                    } => {
                        output = &mut *loop_frames;
                    }
                    _ => panic!("Parent is not a loop"),
                }
            }

            output
        };

        match command {
            MacroCommand::Empty => {
                frame_list.push(MacroFrame {
                    action: ActionType::Empty,
                    delay: Duration::from_micros(delay as u64),
                });
            }
            MacroCommand::LoopBegin => {
                let loop_frame = MacroFrame {
                    action: ActionType::Loop(Vec::new(), Duration::from_micros(delay as u64), 1),
                    delay: Duration::ZERO,
                };

                parents.push(frame_list.len());
                frame_list.push(loop_frame);
            }
            MacroCommand::LoopEnd => {
                let loop_count = data[offset];
                offset += 1;

                let parent_index = parents.pop().unwrap();

                let frame_list = if parents.is_empty() {
                    &mut frames
                } else {
                    let mut temp_parents = parents.clone();
                    let mut output = &mut frames;
                    while !temp_parents.is_empty() {
                        let parent_index = temp_parents.remove(0);
                        match &mut output[parent_index] {
                            MacroFrame {
                                action: ActionType::Loop(loop_frames, _, _),
                                ..
                            } => {
                                output = &mut *loop_frames;
                            }
                            _ => panic!("Parent is not a loop"),
                        }
                    }

                    output
                };

                if let Some(MacroFrame {
                    action: ActionType::Loop(_, duration, count),
                    ..
                }) = frame_list.get_mut(parent_index)
                {
                    *count = loop_count;
                    *duration = Duration::from_micros(delay as u64);
                } else {
                    panic!("Loop end without loop begin");
                }
            }
            MacroCommand::SetLed => {
                let color = (data[offset], data[offset + 1], data[offset + 2]);
                offset += 3;

                frame_list.push(MacroFrame {
                    action: ActionType::SetLed(color),
                    delay: Duration::from_micros(delay as u64),
                });
            }
            MacroCommand::ClearLed => {
                frame_list.push(MacroFrame {
                    action: ActionType::ClearLed,
                    delay: Duration::from_micros(delay as u64),
                });
            }
            MacroCommand::KeyDown => {
                let key = Keyboard::from(data[offset]);
                offset += 1;

                frame_list.push(MacroFrame {
                    action: ActionType::KeyDown(key),
                    delay: Duration::from_micros(delay as u64),
                });
            }
            MacroCommand::KeyUp => {
                let key = Keyboard::from(data[offset]);
                offset += 1;

                frame_list.push(MacroFrame {
                    action: ActionType::KeyUp(key),
                    delay: Duration::from_micros(delay as u64),
                });
            }
            MacroCommand::KeyPress => {
                let key = Keyboard::from(data[offset]);
                offset += 1;

                let key_delay = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]);
                offset += 4;

                frame_list.push(MacroFrame {
                    action: ActionType::KeyPress(
                        Keyboard::from(key),
                        Duration::from_micros(key_delay as u64),
                    ),
                    delay: Duration::from_micros(delay as u64),
                });
            }
            MacroCommand::ConsumerPress => {
                let key = Consumer::from(u16::from_le_bytes([data[offset], data[offset + 1]]));
                offset += 2;

                let key_delay = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]);
                offset += 4;

                frame_list.push(MacroFrame {
                    action: ActionType::ConsumerPress(
                        Consumer::from(key),
                        Duration::from_micros(key_delay as u64),
                    ),
                    delay: Duration::from_micros(delay as u64),
                });
            }
            MacroCommand::TypeString => {
                let key_delay = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]);
                offset += 4;

                let mut string = String::new();

                while data[offset] != 0 {
                    string.push(data[offset] as char);
                    offset += 1;
                }

                offset += 1;

                frame_list.push(MacroFrame {
                    action: ActionType::String(string, Duration::from_micros(key_delay as u64)),
                    delay: Duration::from_micros(delay as u64),
                });
            }
            MacroCommand::Chord => {
                let key_delay = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]);
                offset += 4;

                let mut keys = Vec::new();

                while data[offset] != 0 {
                    keys.push(Keyboard::from(data[offset]));
                    offset += 1;
                }

                offset += 1;

                frame_list.push(MacroFrame {
                    action: ActionType::Chord(keys, Duration::from_micros(key_delay as u64)),
                    delay: Duration::from_micros(delay as u64),
                });
            }
        };
    }

    Macro { frames }
}

impl Macro {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn add_frame(&mut self, frame: MacroFrame) {
        self.frames.push(frame);
    }

    pub fn size(&self) -> usize {
        let mut i = 0;
        for frame in self.frames.iter() {
            i += frame.pack().len();
        }

        i
    }

    pub fn pack(&self) -> Result<([u8; 4092]), ()> {
        let mut data = [0; 4092];

        let mut i = 0;
        for frame in self.frames.iter() {
            let packed = frame.pack();

            if i + packed.len() > 4092 {
                return Err(());
            }

            data[i..i + packed.len()].copy_from_slice(packed.as_slice());
            i += packed.len();
        }

        Ok(data)
    }
}
