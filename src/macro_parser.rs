use std::time::Duration;

use hidapi::HidDevice;
use macropad_protocol::{macro_protocol::{MacroCommand, MacroSectionAnnotation}, data_protocol::{KeyMode, LedEffect}};
use usbd_human_interface_device::page::{Consumer, Keyboard};

use crate::{macropad_wrapper::{self, prime_device}, type_wrapper};


#[derive(Debug, Clone)]
pub enum ActionType {
    Action(Vec<MacroAction>),
    String(String, Option<Duration>),
    Chord(Vec<Keyboard>, Option<Duration>),
    Loop(Vec<MacroFrame>, u8),
}

#[derive(Debug, Clone)]
pub struct MacroFrame {
    pub action: ActionType,
    pub delay: Option<Duration>,
}

impl Default for MacroFrame {
    fn default() -> Self {
        Self {
            action: ActionType::Action(vec![]),
            delay: None,
        }
    }
}

impl MacroFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(action: Vec<MacroAction>, delay: Option<Duration>) -> Self {
        Self {
            action: ActionType::Action(action),
            delay,
        }
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut output = Vec::new();

        match &self.action {
            ActionType::Action(actions) => {
                for action in actions {
                    match action {
                        MacroAction::PressKey(key) => {
                            output.push(MacroCommand::CommandPressKey as u8);
                            output.push(*key as u8);
                        },
                        MacroAction::ReleaseKey(key) => {
                            output.push(MacroCommand::CommandReleaseKey as u8);
                            output.push(*key as u8);
                        },
                        MacroAction::Consumer(consumer) => {
                            output.push(MacroCommand::CommandConsumer as u8);
                            output.extend_from_slice(&(*consumer as u16).to_le_bytes());
                        },
                        MacroAction::SetLed(key) => {
                            output.push(MacroCommand::CommandSetLed as u8);
                            output.push(key.0);
                            output.push(key.1);
                            output.push(key.2);
                        },
                        MacroAction::ClearLed => {
                            output.push(MacroCommand::CommandClearLed as u8);
                        },
                    }
                }

                if let Some(delay) = &self.delay {
                    output.push(MacroCommand::CommandDelay as u8);
                    output.extend_from_slice(&(delay.as_micros() as u32).to_le_bytes());
                }
    
                output.push(MacroCommand::CommandTerminator as u8);
            },
            ActionType::String(string, delay) => {
                output.push(MacroCommand::CommandSectionAnnotation as u8);
                output.push(MacroSectionAnnotation::String as u8);

                let mut caps = false;
                for (i, char) in string.chars().enumerate() {
                    let (key, caps_status) = type_wrapper::KeyboardWrapper::from_char(char);
                    if let Some(caps_status) = caps_status {
                        if caps_status != caps {
                            output.push(if caps_status {
                                    MacroCommand::CommandPressKey
                                } else {
                                    MacroCommand::CommandReleaseKey
                                } as u8);
                            output.push(Keyboard::LeftShift as u8);
                            caps = true;
                        }
                    }

                    output.push(MacroCommand::CommandPressKey as u8);
                    output.push(key as u8);

                    if let Some(delay) = delay {
                        output.push(MacroCommand::CommandDelay as u8);
                        output.extend_from_slice(&(delay.as_micros() as u32).to_le_bytes());
                    }

                    output.push(MacroCommand::CommandTerminator as u8);

                    output.push(MacroCommand::CommandReleaseKey as u8);
                    output.push(key as u8);

                    if i == string.len() - 1 {
                        if let Some(delay) = &self.delay {
                            output.push(MacroCommand::CommandDelay as u8);
                            output.extend_from_slice(&(delay.as_micros() as u32).to_le_bytes());
                        }
                    } else if let Some(delay) = delay {
                        output.push(MacroCommand::CommandDelay as u8);
                        output.extend_from_slice(&(delay.as_micros() as u32).to_le_bytes());
                    }

                    output.push(MacroCommand::CommandTerminator as u8);
                }
                
                output.push(MacroCommand::CommandSectionAnnotation as u8);
                output.push(MacroSectionAnnotation::None as u8);
            },
            ActionType::Chord(keys, delay) => {
                output.push(MacroCommand::CommandSectionAnnotation as u8);
                output.push(MacroSectionAnnotation::Chord as u8);

                for key in keys {
                    output.push(MacroCommand::CommandPressKey as u8);
                    output.push(*key as u8);
                }

                if let Some(delay) = delay {
                    output.push(MacroCommand::CommandDelay as u8);
                    output.extend_from_slice(&(delay.as_micros() as u32).to_le_bytes());
                }

                output.push(MacroCommand::CommandTerminator as u8);

                for key in keys {
                    output.push(MacroCommand::CommandReleaseKey as u8);
                    output.push(*key as u8);
                }

                if let Some(delay) = &self.delay {
                    output.push(MacroCommand::CommandDelay as u8);
                    output.extend_from_slice(&(delay.as_micros() as u32).to_le_bytes());
                }

                output.push(MacroCommand::CommandTerminator as u8);

                output.push(MacroCommand::CommandSectionAnnotation as u8);
                output.push(MacroSectionAnnotation::None as u8);
            },
            ActionType::Loop(frames, count) => {
                let mut packed_frames = Vec::new();
                for frame in frames {
                    packed_frames.extend_from_slice(&frame.pack());
                }

                output.push(MacroCommand::CommandSectionAnnotation as u8);
                output.push(MacroSectionAnnotation::LoopBegin as u8);

                for i in 0..*count {
                    output.extend_from_slice(&packed_frames);
                    output.push(MacroCommand::CommandSectionAnnotation as u8);
                    output.push(MacroSectionAnnotation::LoopIteration as u8);
                }

                output.pop();
                output.push(MacroSectionAnnotation::LoopEnd as u8);
            },
        }

        output
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
        Self {
            frames: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct MacroConfig {
    pub tap_speed: u32,
    pub hold_speed: u32,
    pub default_delay: u32,
}

impl Default for MacroConfig {
    fn default() -> Self {
        Self {
            tap_speed: 200,
            hold_speed: 200,
            default_delay: 100,
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

    let mut current_loop: Option<(Vec<MacroFrame>, u8)> = None;
    let mut action = ActionType::Action(Vec::new());
    let mut caps = false;
    let mut delay = None;
    let mut done = false;
        

    while command != MacroCommand::CommandTerminator {
        while command == MacroCommand::CommandSectionAnnotation {
            let annotation = MacroSectionAnnotation::from(data[i + 1]);
            match annotation {
                MacroSectionAnnotation::None => {
                    match &action {
                        ActionType::Action(_) => {
                            println!("Warning: Don't need to specify None annotation");
                        },
                        ActionType::String(_, d) => {
                            let delta = {
                                if let Some(delay) = delay {
                                    if let Some(d) = d {
                                        if delay > *d {
                                            Some(delay - *d)
                                        } else {
                                            Some(delay)
                                        }
                                    } else {
                                        Some(delay)
                                    }
                                } else {
                                    None
                                }
                            };

                            if let Some((frames, loop_count)) = current_loop.as_mut() {
                                if *loop_count == 1 {
                                    frames.push(MacroFrame {
                                        action,
                                        delay: delta,
                                    });
                                }
                            } else {
                                frames.push(MacroFrame {
                                    action,
                                    delay: delta,
                                });
                            }
                            

                            action = ActionType::Action(Vec::new());
                        },
                        ActionType::Chord(k, d) => {
                            if let Some((frames, loop_count)) = current_loop.as_mut() {
                                if *loop_count == 1 {
                                    frames.push(MacroFrame {
                                        action,
                                        delay,
                                    });
                                }
                            } else {
                                frames.push(MacroFrame {
                                    action,
                                    delay,
                                });
                            }

                            action = ActionType::Action(Vec::new());
                        },

                        ActionType::Loop(_, _) => unreachable!(),
                    }
                },
                MacroSectionAnnotation::String => {
                    if let ActionType::Action(_) = &action {
                        action = ActionType::String("".to_string(), None);
                        caps = false;
                    } else {
                        println!("Warning: action already set, skipping string");
                    }  
                },
                MacroSectionAnnotation::Chord => {
                    if let ActionType::Action(_) = &action {
                        action = ActionType::Chord(Vec::new(), None);
                    } else {
                        println!("Warning: action already set, skipping chord");
                    }  
                },
                MacroSectionAnnotation::LoopBegin => {
                    if current_loop.is_some() {
                        println!("Warning: Nested loop found, skipping");
                    } else {
                        current_loop = Some((Vec::new(), 1));
                    }
                },
                MacroSectionAnnotation::LoopIteration => {
                    if let Some((_, count)) = &mut current_loop {
                        *count += 1;
                    } else {
                        println!("Warning: Loop iteration found without loop, skipping");
                    }
                },
                MacroSectionAnnotation::LoopEnd => {
                    if let Some((mut loop_frames, count)) = current_loop.take() {
                        frames.push(MacroFrame {
                            action: ActionType::Loop(loop_frames, count),
                            delay,
                        });
                    } else {
                        println!("Warning: Loop end found without loop, skipping");
                    }
                },
            }
            i += 2;
            command = MacroCommand::from(data[i]);

            if command == MacroCommand::CommandTerminator {
                done = true;
                break;
            }
        }

        if done {
            break;
        }

        delay = None;

        while command != MacroCommand::CommandTerminator {
            match command {
                MacroCommand::CommandSectionAnnotation => {
                    println!("Warning: Section annotation found in command section, skipping");
                    i += 2;
                }
                MacroCommand::CommandDelay => {
                    let delay_bytes = [data[i + 1], data[i + 2], data[i + 3], data[i + 4]];
                    let delay_millis = u32::from_le_bytes(delay_bytes);
                    delay = Some(Duration::from_micros(delay_millis as u64));
                    i += 5;
                }
                MacroCommand::CommandPressKey => {
                    let key = Keyboard::from(data[i + 1]);
                    match &mut action {
                        ActionType::Action(actions) => {
                            actions.push(MacroAction::PressKey(key));
                        },
                        ActionType::String(string, _) => {
                            if key == Keyboard::LeftShift || key == Keyboard::RightShift {
                                caps = true;
                            } else {
                                string.push(type_wrapper::KeyboardWrapper::from(key).get_char(caps));
                            }
                        },
                        ActionType::Chord(keys, _) => {
                            keys.push(key);
                        },
                        ActionType::Loop(_, _) => unreachable!(),
                    }

                    i += 2;
                }
                MacroCommand::CommandReleaseKey => {
                    let key = Keyboard::from(data[i + 1]);
                    match &mut action {
                        ActionType::Action(actions) => {
                            actions.push(MacroAction::ReleaseKey(key));
                        },
                        ActionType::String(string, _) => {
                            if key == Keyboard::LeftShift || key == Keyboard::RightShift {
                                caps = false;
                            }
                        },
                        ActionType::Chord(keys, _) => {
                            // Nothing is needed here
                        },
                        ActionType::Loop(_, _) => unreachable!(),
                    }

                    i += 2;
                }
                MacroCommand::CommandConsumer => {
                    let key = Consumer::from(u16::from_le_bytes([data[i + 1], data[i + 2]]));

                    match &mut action {
                        ActionType::Action(actions) => {
                            actions.push(MacroAction::Consumer(key));
                        },
                        ActionType::Loop(_, _) => unreachable!(),
                        _ => println!("Warning: Consumer key found in string or chord, skipping"),
                    }

                    i += 3;
                }
                MacroCommand::CommandSetLed => {
                    let r = data[i + 1];
                    let g = data[i + 2];
                    let b = data[i + 3];
                    match &mut action {
                        ActionType::Action(actions) => {
                            actions.push(MacroAction::SetLed((r, g, b)));
                        },
                        ActionType::Loop(_, _) => unreachable!(),
                        _ => println!("Warning: Set LED found in string or chord, skipping"),
                    }

                    i += 4;
                }
                MacroCommand::CommandClearLed => {
                    match &mut action {
                        ActionType::Action(actions) => {
                            actions.push(MacroAction::ClearLed);
                        },
                        ActionType::Loop(_, _) => unreachable!(),
                        _ => println!("Warning: Clear LED found in string or chord, skipping"),
                    }

                    i += 1;
                }
                _ => {}
            }
            command = MacroCommand::from(data[i]);
        }

        match &mut action {
            ActionType::Action(actions) => {
                if actions.len() > 0 {
                    if let Some((frames, loop_count)) = current_loop.as_mut() {
                        if *loop_count == 1 {
                            frames.push(MacroFrame {
                                action,
                                delay,
                            });

                            action = ActionType::Action(Vec::new());
                        }
                    } else {
                        frames.push(MacroFrame {
                            action,
                            delay,
                        });

                        action = ActionType::Action(Vec::new());
                    }
                } else {
                    println!("Warning: Empty action found, skipping");
                }
            },
            ActionType::String(_, key_delay) => {
                if key_delay.is_none() {
                    *key_delay = delay;
                }
            },
            ActionType::Chord(_, key_delay) => {
                if key_delay.is_none() {
                    *key_delay = delay;
                }
            },
            ActionType::Loop(_, _) => unreachable!(),
        }
        
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

    pub fn pack(&self) -> Result<[u8; 4092], ()> {
        let mut data = [0; 4092];

        let mut i = 0;
        for frame in self.frames.iter() {
            let packed = frame.pack();

            if i + packed.len() > 4092 {
                return Err(());
            }

            data[i..i + packed.len()].copy_from_slice(&packed);
            i += packed.len();
        }

        Ok(data)
    }
}