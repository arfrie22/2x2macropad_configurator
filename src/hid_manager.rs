use hidapi::{HidApi, HidDevice};
use iced_futures::futures;
use iced_native::subscription::{self, Subscription};

use futures::channel::mpsc;

use async_std::future;
use futures::stream::StreamExt;

use std::sync::Arc;
use std::{fmt, sync::Mutex};

use crate::{
    macro_parser::{self, LedConfig},
    macropad_wrapper,
};

async fn scan_devices(api: &mut HidApi) -> Option<HidDevice> {
    api.refresh_devices().unwrap();
    for device in api.device_list() {
        if device.vendor_id() == 0x554D
            && device.product_id() == 0x2020
            && device.usage_page() == 0xff00
            && device.usage() == 1
        {
            let d = device.open_device(&api).unwrap();
            return Some(d);
        }
    }

    None
}

async fn is_connected(api: &mut HidApi) -> bool {
    api.refresh_devices().unwrap();
    for device in api.device_list() {
        if device.vendor_id() == 0x554D
            && device.product_id() == 0x2020
            && device.usage_page() == 0xff00
            && device.usage() == 1
        {
            return true;
        }
    }

    false
}

pub fn connect() -> Subscription<Event> {
    struct Connect;

    subscription::unfold(
        std::any::TypeId::of::<Connect>(),
        State::Uninitialized,
        |state| async move {
            match state {
                State::Uninitialized => {
                    let api = hidapi::HidApi::new().unwrap();
                    (Some(Event::Disconnected), State::Disconnected(api))
                }
                State::Disconnected(mut api) => {
                    if let Some(d) = scan_devices(&mut api).await {
                        let (sender, receiver) = mpsc::channel(100);
                        let macropad =
                            Arc::new(Mutex::new(macro_parser::get_macro_pad(&d).unwrap()));
                        (
                            Some(Event::Connected(Connection(sender, macropad.clone()))),
                            State::Connected(api, d, macropad.clone(), receiver),
                        )
                    } else {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                        (None, State::Disconnected(api))
                    }
                }
                State::Connected(mut api, device, macropad, mut input) => {
                    let command = future::timeout(
                        std::time::Duration::from_secs(1),
                        input.select_next_some(),
                    )
                    .await;
                    if let Ok(command) = command {
                        match command {
                            Message::Set(command) => {
                                let res = match command {
                                    MacropadCommand::Bootloader => {
                                        macropad_wrapper::enter_bootloader(&device)
                                    }
                                    MacropadCommand::KeyMode(i, mode) => {
                                        macropad_wrapper::set_key_mode(&device, i, mode).and_then(
                                            |_| {
                                                macropad.lock().unwrap().key_configs[i as usize]
                                                    .key_mode = mode;
                                                Ok(())
                                            },
                                        )
                                    }
                                    MacropadCommand::KeyboardData(i, data) => {
                                        macropad_wrapper::set_keyboard_data(&device, i, data)
                                            .and_then(|_| {
                                                macropad.lock().unwrap().key_configs[i as usize]
                                                    .keyboard_data = data;
                                                Ok(())
                                            })
                                    }
                                    MacropadCommand::ConsumerData(i, data) => {
                                        macropad_wrapper::set_consumer_data(&device, i, data)
                                            .and_then(|_| {
                                                macropad.lock().unwrap().key_configs[i as usize]
                                                    .consumer_data = data;
                                                Ok(())
                                            })
                                    }
                                    MacropadCommand::KeyColor(i, color) => {
                                        macropad_wrapper::set_key_color(&device, i, color).and_then(
                                            |_| {
                                                macropad.lock().unwrap().key_configs[i as usize]
                                                    .key_color = color;
                                                Ok(())
                                            },
                                        )
                                    }
                                    MacropadCommand::Macro(i, macro_data) => {
                                        macro_data.pack().and_then(|data| {
                                            macropad_wrapper::clear_macro(&device, i).and_then(
                                                |_| {
                                                    macropad_wrapper::set_macro(&device, i, &data)
                                                        .and_then(|_| {
                                                            macropad_wrapper::validate_macro(
                                                                &device, i, &data,
                                                            )
                                                            .and_then(|_| {
                                                                macropad.lock().unwrap().set_macro(
                                                                    i as usize, macro_data,
                                                                );
                                                                Ok(())
                                                            })
                                                        })
                                                },
                                            )
                                        })
                                    }
                                    MacropadCommand::TapSpeed(speed) => {
                                        macropad_wrapper::set_tap_speed(&device, speed).and_then(
                                            |_| {
                                                macropad.lock().unwrap().config.tap_speed = speed;
                                                Ok(())
                                            },
                                        )
                                    }
                                    MacropadCommand::HoldSpeed(speed) => {
                                        macropad_wrapper::set_hold_speed(&device, speed).and_then(
                                            |_| {
                                                macropad.lock().unwrap().config.hold_speed = speed;
                                                Ok(())
                                            },
                                        )
                                    }
                                    MacropadCommand::LedBaseColor(color) => {
                                        macropad_wrapper::set_led_base_color(&device, color)
                                            .and_then(|_| {
                                                macropad.lock().unwrap().led_config.base_color =
                                                    color;
                                                Ok(())
                                            })
                                    }
                                    MacropadCommand::LedEffect(effect) => {
                                        macropad_wrapper::set_led_effect(&device, effect).and_then(
                                            |_| {
                                                macropad.lock().unwrap().led_config.effect = effect;
                                                Ok(())
                                            },
                                        )
                                    }
                                    MacropadCommand::LedBrightness(brightness) => {
                                        macropad_wrapper::set_led_brightness(&device, brightness)
                                            .and_then(|_| {
                                                macropad.lock().unwrap().led_config.brightness =
                                                    brightness;
                                                Ok(())
                                            })
                                    }
                                    MacropadCommand::LedEffectPeriod(period) => {
                                        macropad_wrapper::set_led_effect_period(&device, period)
                                            .and_then(|_| {
                                                macropad.lock().unwrap().led_config.effect_period =
                                                    period;
                                                Ok(())
                                            })
                                    }
                                    MacropadCommand::LedEffectOffset(offset) => {
                                        macropad_wrapper::set_led_effect_offset(&device, offset)
                                            .and_then(|_| {
                                                macropad.lock().unwrap().led_config.effect_offset =
                                                    offset;
                                                Ok(())
                                            })
                                    }
                                };
                                if res.is_err() {
                                    drop(device);
                                    (Some(Event::Disconnected), State::Disconnected(api))
                                } else {
                                    (
                                        Some(Event::MacropadUpdated),
                                        State::Connected(api, device, macropad, input),
                                    )
                                }
                            }

                            _ => (None, State::Connected(api, device, macropad, input)),
                        }
                    } else {
                        if is_connected(&mut api).await {
                            (None, State::Connected(api, device, macropad, input))
                        } else {
                            (Some(Event::Disconnected), State::Disconnected(api))
                        }
                    }
                }
            }
        },
    )
}

#[allow(clippy::large_enum_variant)]
enum State {
    Uninitialized,
    Disconnected(hidapi::HidApi),
    Connected(
        hidapi::HidApi,
        hidapi::HidDevice,
        Arc<Mutex<macro_parser::Macropad>>,
        mpsc::Receiver<Message>,
    ),
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Uninitialized => write!(f, " Uninitialized"),
            State::Disconnected(_) => write!(f, "Disconnected"),
            State::Connected(_, _, _, _) => write!(f, "Connected"),
            // State::Sent(_, _, _, _, _) => write!(f, "Sent"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected(Connection),
    Disconnected,
    MacropadUpdated,
}

#[derive(Debug, Clone)]
pub struct Connection(mpsc::Sender<Message>, Arc<Mutex<macro_parser::Macropad>>);

impl Connection {
    pub fn send(&mut self, message: Message) {
        self.0
            .try_send(message)
            .expect("Send message to echo server");
    }

    pub fn get_macropad(&self) -> Arc<Mutex<macro_parser::Macropad>> {
        self.1.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Connected,
    Disconnected,
    Set(MacropadCommand),
}

#[derive(Debug, Clone)]
pub enum MacropadCommand {
    Bootloader,
    KeyMode(u8, macropad_protocol::data_protocol::KeyMode),
    KeyboardData(u8, usbd_human_interface_device::page::Keyboard),
    ConsumerData(u8, usbd_human_interface_device::page::Consumer),
    KeyColor(u8, (u8, u8, u8)),
    Macro(u8, macro_parser::Macro),
    TapSpeed(u32),
    HoldSpeed(u32),
    LedBaseColor((u8, u8, u8)),
    LedEffect(macropad_protocol::data_protocol::LedEffect),
    LedBrightness(u8),
    LedEffectPeriod(f32),
    LedEffectOffset(f32),
}

impl Message {
    pub fn new(command: MacropadCommand) -> Option<Self> {
        Some(Self::Set(command))
    }

    pub fn connected() -> Self {
        Message::Connected
    }

    pub fn disconnected() -> Self {
        Message::Disconnected
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Connected => write!(f, "Connected successfully!"),
            Message::Disconnected => {
                write!(f, "Connection lost... Retrying...")
            }
            Message::Set(command) => match command {
                MacropadCommand::Bootloader => write!(f, "Enter bootloader"),
                MacropadCommand::KeyMode(i, mode) => {
                    write!(f, "Set key mode of {:?} to {:?}", i, mode)
                }
                MacropadCommand::KeyboardData(i, data) => {
                    write!(f, "Set keyboard data of {:?} to {:?}", i, data)
                }
                MacropadCommand::ConsumerData(i, data) => {
                    write!(f, "Set consumer data of {:?} to {:?}", i, data)
                }
                MacropadCommand::KeyColor(i, color) => {
                    write!(f, "Set key color of {:?} to {:?}", i, color)
                }
                MacropadCommand::Macro(key, macro_) => {
                    write!(f, "Set macro for key {} to {:?}", key, macro_)
                }
                MacropadCommand::TapSpeed(speed) => write!(f, "Set tap speed to {}", speed),
                MacropadCommand::HoldSpeed(speed) => write!(f, "Set hold speed to {}", speed),
                MacropadCommand::LedBaseColor(color) => {
                    write!(f, "Set led base color to {:?}", color)
                }
                MacropadCommand::LedEffect(effect) => write!(f, "Set led effect to {:?}", effect),
                MacropadCommand::LedBrightness(brightness) => {
                    write!(f, "Set led brightness to {}", brightness)
                }
                MacropadCommand::LedEffectPeriod(period) => {
                    write!(f, "Set led effect period to {}", period)
                }
                MacropadCommand::LedEffectOffset(offset) => {
                    write!(f, "Set led effect offset to {}", offset)
                }
            },
        }
    }
}
