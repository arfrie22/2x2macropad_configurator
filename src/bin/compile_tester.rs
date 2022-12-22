use hidapi::HidApi;
use macropad_configurator::{macropad_wrapper, macro_parser::{parse_macro, self}};
use macropad_protocol::data_protocol::{LedEffect, KeyMode};
use usbd_human_interface_device::page::Consumer;

fn main() {
    let mut api = HidApi::new().unwrap();
    api.refresh_devices().unwrap();

    for device in api.device_list() {
        if device.vendor_id() == 4617 && device.product_id() == 1 && device.usage_page() == 0xff00 && device.usage() == 1 {
            println!("Device: {:?}", device);
            let d = device.open_device(&api).unwrap();

            macropad_wrapper::set_led_effect(&d, LedEffect::Rainbow).unwrap();

            macropad_wrapper::set_consumer_data(&d, 0, Consumer::VolumeIncrement).unwrap();
            macropad_wrapper::set_key_mode(&d, 0, KeyMode::ConsumerMode).unwrap();

            println!("{:?}", macro_parser::get_macro_pad(&d).unwrap());
        }
    }
}