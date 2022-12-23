use std::time::Duration;

use hidapi::HidApi;
use macropad_configurator::{macropad_wrapper::{self, prime_device}, macro_parser::{parse_macro, self, Macro, MacroFrame, MacroAction}};
use macropad_protocol::{data_protocol::{LedEffect, KeyMode}, macro_protocol::MacroCommand};
use usbd_human_interface_device::page::{Consumer, Keyboard};

fn main() {
    let mut api = HidApi::new().unwrap();
    api.refresh_devices().unwrap();

    for device in api.device_list() {
        if device.vendor_id() == 4617 && device.product_id() == 1 && device.usage_page() == 0xff00 && device.usage() == 1 {
            println!("Device: {:?}", device);
            let d = device.open_device(&api).unwrap();
            prime_device(&d).unwrap();

            // macropad_wrapper::set_led_effect(&d, LedEffect::Rainbow).unwrap();

            // macropad_wrapper::set_consumer_data(&d, 0, Consumer::VolumeIncrement).unwrap();
            // macropad_wrapper::set_key_mode(&d, 0, KeyMode::ConsumerMode).unwrap();

            let mut mac = Macro::new();
            mac.add_frame(MacroFrame::from(vec![MacroAction::SetLed((255, 0 ,0)), MacroAction::PressKey(Keyboard::A)], Some(Duration::from_millis(200))));
            mac.add_frame(MacroFrame::from(vec![MacroAction::ReleaseKey(Keyboard::A)], None));

            let macro_data = mac.pack();
            
            macropad_wrapper::set_base_color(&d, (255, 0, 0)).unwrap();
            macropad_wrapper::set_led_effect(&d, LedEffect::Rainbow).unwrap();
            macropad_wrapper::set_effect_period(&d, 5.0).unwrap();

            macropad_wrapper::clear_macro(&d, 0).unwrap();
            macropad_wrapper::set_macro(&d, 0, &macro_data).unwrap();
            macropad_wrapper::validate_macro(&d, 0, &macro_data).unwrap();
            

            println!("{:?}", macro_parser::get_macro_pad(&d).unwrap());
            println!("{:?}", macro_parser::get_macro_pad(&d).unwrap());
            println!("{:?}", macro_parser::get_macro_pad(&d).unwrap());
            println!("{:?}", macro_parser::get_macro_pad(&d).unwrap());
        }
    }
}