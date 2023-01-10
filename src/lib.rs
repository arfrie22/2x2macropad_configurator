pub mod font;
pub mod hid_manager;
pub mod led_effects;
pub mod macro_editor;
pub mod macro_parser;
pub mod macropad;
pub mod macropad_wrapper;
pub mod type_wrapper;
pub mod macropad_updater;

#[cfg(test)]
mod tests {
    // Run with `cargo test macropad -- --include-ignored`
    #[test]
    #[ignore = "requires a real macropad"]
    fn macropad() {
        use std::time::Duration;

        use hidapi::HidApi;
        use crate::macro_parser::{self, ActionType, Macro, MacroFrame};
        use crate::macropad_wrapper::{self, prime_device};

        use macropad_protocol::{
            data_protocol::{KeyMode, LedEffect},
        };
        use usbd_human_interface_device::page::{Consumer, Keyboard};

        let mut api = HidApi::new().unwrap();
        api.refresh_devices().unwrap();

        for device in api.device_list() {
            if device.vendor_id() == 4617
                && device.product_id() == 1
                && device.usage_page() == 0xff00
                && device.usage() == 1
            {
                println!("Device: {:?}", device);
                let d = device.open_device(&api).unwrap();
                prime_device(&d).unwrap();

                macropad_wrapper::set_led_effect(&d, LedEffect::Rainbow).unwrap();

                macropad_wrapper::set_consumer_data(&d, 0, Consumer::VolumeIncrement).unwrap();
                // macropad_wrapper::set_keyboard_data(&d, 0, Keyboard).unwrap();
                macropad_wrapper::set_key_mode(&d, 0, KeyMode::ConsumerMode).unwrap();

                let mut mac = Macro::new();

                mac.add_frame(MacroFrame {
                    action: macro_parser::ActionType::Loop(
                        vec![
                            MacroFrame {
                                action: ActionType::Loop(
                                    vec![MacroFrame {
                                        action: macro_parser::ActionType::ConsumerPress(
                                            Consumer::VolumeIncrement,
                                            Duration::from_millis(100),
                                        ),
                                        delay: Duration::from_millis(100),
                                    }],
                                    Duration::ZERO,
                                    5,
                                ),
                                delay: Duration::ZERO,
                            },
                            // MacroFrame {
                            //     action: ActionType::Loop(vec![MacroFrame {action: macro_parser::ActionType::String("This is a test, lmao".to_owned(), Duration::from_millis(100)), delay: Duration::from_millis(100)}], Duration::ZERO, 5),
                            //     delay: Duration::ZERO,
                            // },
                            MacroFrame {
                                action: ActionType::Loop(
                                    vec![MacroFrame {
                                        action: macro_parser::ActionType::ConsumerPress(
                                            Consumer::VolumeDecrement,
                                            Duration::from_millis(100),
                                        ),
                                        delay: Duration::from_millis(100),
                                    }],
                                    Duration::ZERO,
                                    5,
                                ),
                                delay: Duration::ZERO,
                            },
                        ],
                        Duration::ZERO,
                        10,
                    ),
                    delay: Duration::ZERO,
                });

                let macro_data = mac.pack().unwrap();
                println!("{:?}", macro_data);

                macropad_wrapper::clear_macro(&d, 4).unwrap();
                macropad_wrapper::set_macro(&d, 4, &macro_data).unwrap();
                macropad_wrapper::validate_macro(&d, 4, &macro_data).unwrap();

                println!("{:?}", macro_parser::get_macro_pad(&d).unwrap());
            }
        }
    }
}
