use std::time::Duration;

use hidapi::HidApi;
use macropad_configurator::{
    macro_parser::{self, parse_macro, Macro, MacroFrame, ActionType},
    macropad_wrapper::{self, prime_device},
};
use macropad_protocol::{
    data_protocol::{KeyMode, LedEffect},
    macro_protocol::MacroCommand,
};
use usbd_human_interface_device::page::{Consumer, Keyboard};

fn main() {
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

            // macropad_wrapper::set_led_effect(&d, LedEffect::Rainbow).unwrap();

            macropad_wrapper::set_consumer_data(&d, 0, Consumer::VolumeIncrement).unwrap();
            // macropad_wrapper::set_keyboard_data(&d, 0, Keyboard).unwrap();
            macropad_wrapper::set_key_mode(&d, 0, KeyMode::ConsumerMode).unwrap();

            let mut mac = Macro::new();
            // mac.add_frame(MacroFrame::from(
            //     vec![
            //         MacroAction::SetLed((255, 0, 0)),
            //         MacroAction::PressKey(Keyboard::A),
            //     ],
            //     Some(Duration::from_millis(200)),
            // ));
            // mac.add_frame(MacroFrame::from(
            //     vec![MacroAction::ReleaseKey(Keyboard::A)],
            //     None,
            // ));

            // mac.add_frame(MacroFrame::from(
            //     vec![
            //         MacroAction::SetLed((255, 0, 0)),
            //     ],
            //     Some(Duration::from_millis(200)),
            // ));

            // mac.add_frame(MacroFrame::from(
            //     vec![
            //         MacroAction::SetLed((255, 0, 0)),
            //         MacroAction::Consumer(Consumer::VolumeIncrement),
            //     ], None));

            // mac.add_frame(MacroFrame {
            //     action: ActionType::Chord(vec![Keyboard::A, Keyboard::B], Some(Duration::from_millis(30))),
            //     delay: Some(Duration::from_millis(200)),
            // });

            mac.add_frame(MacroFrame {
                action: macro_parser::ActionType::String(
                    "This is a test, lmao".to_string(),
                    Some(Duration::from_millis(100)),
                ),
                delay: None,
            });

            // mac.add_frame(MacroFrame {
            //     action: macro_parser::ActionType::KeyPress(Keyboard::A, Some(Duration::from_millis(30))),
            //     delay: Some(Duration::from_millis(70)),
            // });

            
            // mac.add_frame(MacroFrame {
            //     action: macro_parser::ActionType::ConsumerPress(Consumer::VolumeIncrement, Some(Duration::from_millis(40))),
            //     delay: Some(Duration::from_millis(20)),
            // });

            // mac.add_frame(MacroFrame {
            //     action: macro_parser::ActionType::KeyPress(Keyboard::A, Some(Duration::from_millis(30))),
            //     delay: Some(Duration::from_millis(70)),
            // });

            // mac.add_frame(MacroFrame {
            //     action: ActionType::Loop(vec![
            //         MacroFrame {
            //             action: ActionType::Chord(vec![Keyboard::A, Keyboard::B], Some(Duration::from_millis(30))),
            //             delay: Some(Duration::from_millis(200)),
            //         },
            //         MacroFrame {
            //             action: ActionType::String(
            //                 "This is a test, lmao".to_string(),
            //                 Some(Duration::from_millis(100)),
            //             ),
            //             delay: None,
            //         },
            //     ], 3),
            //     delay: Some(Duration::from_millis(200)),
            // });

            let mut macro_data = mac.pack().unwrap();

            macropad_wrapper::set_led_base_color(&d, (255, 0, 0)).unwrap();
            macropad_wrapper::set_led_effect(&d, LedEffect::Rainbow).unwrap();
            macropad_wrapper::set_led_effect_period(&d, 5.0).unwrap();

            macropad_wrapper::clear_macro(&d, 4).unwrap();
            macropad_wrapper::set_macro(&d, 4, &macro_data).unwrap();
            macropad_wrapper::validate_macro(&d, 4, &macro_data).unwrap();
            
            macro_data[1] = 0x00;
            let macro_2 = parse_macro(&macro_data);
            println!("Macro 2: {:?}", macro_2);
            let macro_data_2 = macro_2.pack().unwrap();
            macropad_wrapper::clear_macro(&d, 5).unwrap();
            macropad_wrapper::set_macro(&d, 5, &macro_data_2).unwrap();
            macropad_wrapper::validate_macro(&d, 5, &macro_data_2).unwrap();

            let mut mac3 = Macro::new();
            mac3.add_frame(MacroFrame {
                action: ActionType::Loop(vec![
                    MacroFrame {
                        action: ActionType::Chord(vec![Keyboard::A, Keyboard::B], Some(Duration::from_millis(30))),
                        delay: Some(Duration::from_millis(200)),
                    },
                    MacroFrame {
                        action: ActionType::String(
                            "This is a test, lmao".to_string(),
                            Some(Duration::from_millis(100)),
                        ),
                        delay: None,
                    },
                ], 3),
                delay: Some(Duration::from_millis(200)),
            });

            let macro_data_3 = mac3.pack().unwrap();

            macropad_wrapper::clear_macro(&d, 6).unwrap();
            macropad_wrapper::set_macro(&d, 6, &macro_data_3).unwrap();
            macropad_wrapper::validate_macro(&d, 6, &macro_data_3).unwrap();


            let mut mac4 = Macro::new();
            for _ in 0..5 {
                mac4.add_frame(MacroFrame {
                    action: ActionType::Loop(vec![
                        MacroFrame {
                            action: ActionType::Chord(vec![Keyboard::A, Keyboard::B], Some(Duration::from_millis(30))),
                            delay: Some(Duration::from_millis(200)),
                        },
                        MacroFrame {
                            action: ActionType::Chord(vec![Keyboard::A, Keyboard::B], Some(Duration::from_millis(30))),
                            delay: None,
                        },
                    ], 3),
                    delay: Some(Duration::from_millis(200)),
                });
            }

            let macro_data_4 = mac4.pack().unwrap();

            macropad_wrapper::clear_macro(&d, 7).unwrap();
            macropad_wrapper::set_macro(&d, 7, &macro_data_4).unwrap();
            macropad_wrapper::validate_macro(&d, 7, &macro_data_4).unwrap();


            println!("{:?}", macro_parser::get_macro_pad(&d).unwrap());
        }
    }
}
