use std::time::Duration;

use macropad_protocol::{macro_protocol::MacroCommand, hid_wrapper};
use usbd_human_interface_device::page::{Consumer, Keyboard};

#[derive(Debug, Clone)]
pub struct MacroFrame {
    pub actions: Vec<MacroAction>,
    pub delay: Option<Duration>,
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

pub fn parse_macro(data: [u8; 4092]) -> Macro {
    let mut frames = Vec::new();

    let mut i = 0;
    let mut command = MacroCommand::from_u8(data[i]).unwrap();
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
                    let key = hid_wrapper::keyboard_from_u8(data[i + 1]).unwrap();
                    actions.push(MacroAction::PressKey(key));
                    i += 2;
                }
                MacroCommand::CommandReleaseKey => {
                    let key = hid_wrapper::keyboard_from_u8(data[i + 1]).unwrap();
                    actions.push(MacroAction::ReleaseKey(key));
                    i += 2;
                }
                MacroCommand::CommandConsumer => {
                    let key = hid_wrapper::consumer_from_u16(u16::from_le_bytes([data[i + 1], data[i + 2]])).unwrap();
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
            command = MacroCommand::from_u8(data[i]).unwrap();
        }
        frames.push(MacroFrame {
            actions,
            delay,
        });
        i += 1;
        command = MacroCommand::from_u8(data[i]).unwrap();
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