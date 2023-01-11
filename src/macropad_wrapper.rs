use crc::{Crc, CRC_32_CKSUM};
use hidapi::HidDevice;
use macropad_protocol::data_protocol::{
    BuildInfoElements, ConfigElements, DataCommand, KeyConfigElements, KeyMode, LedCommand,
    LedEffect,
};
use usbd_human_interface_device::page::{Consumer, Keyboard};

use crate::macro_parser::KeyConfig;

pub const CKSUM: Crc<u32> = Crc::<u32>::new(&CRC_32_CKSUM);
pub const MACRO_SIZE: usize = 4092;

pub const EFFECTS: [LedEffect; 6] = [
    LedEffect::None,
    LedEffect::Static,
    LedEffect::Breathing,
    LedEffect::BreathingSpaced,
    LedEffect::ColorCycle,
    LedEffect::Rainbow,
];

pub fn send_command(device: &HidDevice, command: [u8; 65]) -> Result<[u8; 64], ()> {
    let mut response = [0u8; 64];
    device.write(&command).unwrap();
    if (device.read_timeout(&mut response, 1000)).is_err() {
        Err(())
    } else {
        Ok(response)
    }
}

pub fn prime_device(device: &HidDevice) -> Result<(), ()> {
    send_command(device, [0u8; 65])?;
    Ok(())
}

pub fn enter_bootloader(device: &HidDevice) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::EnterBootloader as u8;
    let buf = send_command(device, data)?;

    Ok(())
}

pub fn get_key_mode(device: &HidDevice, index: u8) -> Result<KeyMode, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::ReadKeyConfig as u8;
    data[2] = KeyConfigElements::KeyMode as u8;
    data[3] = index;
    let buf = send_command(device, data)?;

    if data[1..4] != buf[0..3] {
        Err(())
    } else {
        Ok(KeyMode::from(buf[3]))
    }
}

pub fn set_key_mode(device: &HidDevice, index: u8, mode: KeyMode) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::WriteKeyConfig as u8;
    data[2] = KeyConfigElements::KeyMode as u8;
    data[3] = index;
    data[4] = mode as u8;
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_keyboard_data(device: &HidDevice, index: u8) -> Result<Keyboard, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::ReadKeyConfig as u8;
    data[2] = KeyConfigElements::KeyboardData as u8;
    data[3] = index;
    let buf = send_command(device, data)?;

    if data[1..4] != buf[0..3] {
        Err(())
    } else {
        Ok(Keyboard::from(buf[3]))
    }
}

pub fn set_keyboard_data(device: &HidDevice, index: u8, keyboard: Keyboard) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::WriteKeyConfig as u8;
    data[2] = KeyConfigElements::KeyboardData as u8;
    data[3] = index;
    data[4] = keyboard as u8;
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_consumer_data(device: &HidDevice, index: u8) -> Result<Consumer, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::ReadKeyConfig as u8;
    data[2] = KeyConfigElements::ConsumerData as u8;
    data[3] = index;
    let buf = send_command(device, data)?;

    if data[1..4] != buf[0..3] {
        Err(())
    } else {
        Ok(Consumer::from(u16::from_le_bytes([buf[3], buf[4]])))
    }
}

pub fn set_consumer_data(device: &HidDevice, index: u8, consumer: Consumer) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::WriteKeyConfig as u8;
    data[2] = KeyConfigElements::ConsumerData as u8;
    data[3] = index;
    data[4..6].copy_from_slice(&(consumer as u16).to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_key_color(device: &HidDevice, index: u8) -> Result<(u8, u8, u8), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::ReadKeyConfig as u8;
    data[2] = KeyConfigElements::KeyColor as u8;
    data[3] = index;
    let buf = send_command(device, data)?;

    if data[1..4] != buf[0..3] {
        Err(())
    } else {
        Ok((buf[3], buf[4], buf[5]))
    }
}

pub fn set_key_color(device: &HidDevice, index: u8, color: (u8, u8, u8)) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::WriteKeyConfig as u8;
    data[2] = KeyConfigElements::KeyColor as u8;
    data[3] = index;
    data[4] = color.0;
    data[5] = color.1;
    data[6] = color.2;
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_macro(device: &HidDevice, index: u8) -> Result<[u8; 4092], ()> {
    let mut output = [0u8; 4092];
    for i in 0..(MACRO_SIZE / 59) {
        let mut data = [0u8; 65];
        let offset: u16 = i as u16 * 59;
        let size = if offset as usize + 59 > MACRO_SIZE {
            MACRO_SIZE % 59
        } else {
            59
        };
        data[1] = DataCommand::ReadMacro as u8;
        data[2] = index;
        data[3..5].copy_from_slice(&offset.to_le_bytes());
        data[5] = size as u8;

        let buf = send_command(device, data)?;
        if buf[5..(5 + size)] == [0u8; 64][5..(5 + size)] {
            break;
        } else {
            output[(i * 59)..((i * 59) + size)].copy_from_slice(&buf[5..(5 + size)]);
        }
    }

    Ok(output)
}

pub fn set_macro(device: &HidDevice, index: u8, macro_data: &[u8; 4092]) -> Result<(), ()> {
    for i in 0..(MACRO_SIZE / 59) {
        let mut data = [0u8; 65];
        let offset: u16 = i as u16 * 59;
        let size = if offset as usize + 59 > MACRO_SIZE {
            MACRO_SIZE % 59
        } else {
            59
        };
        data[1] = DataCommand::WriteMacro as u8;
        data[2] = index;
        data[3..5].copy_from_slice(&offset.to_le_bytes());
        data[5] = size as u8;
        data[6..(6 + size)].copy_from_slice(&macro_data[(i * 59)..((i * 59) + size)]);
        if data[6..(6 + size)] == [0u8; 65][6..(6 + size)] {
            break;
        }

        let buf = send_command(device, data)?;
        if data[1..65] != buf {
            return Err(());
        }
    }

    Ok(())
}

pub fn validate_macro(device: &HidDevice, index: u8, macro_data: &[u8; 4092]) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::ValidateMacro as u8;
    data[2] = index;
    data[3..7].copy_from_slice(&CKSUM.checksum(macro_data).to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] || buf[2..6] != buf[6..10] {
        Err(())
    } else {
        Ok(())
    }
}

pub fn clear_macro(device: &HidDevice, index: u8) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::ClearMacro as u8;
    data[2] = index;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_tap_speed(device: &HidDevice) -> Result<u32, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::ReadConfig as u8;
    data[2] = ConfigElements::TapSpeed as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]))
    }
}

pub fn set_tap_speed(device: &HidDevice, speed: u32) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::WriteConfig as u8;
    data[2] = ConfigElements::TapSpeed as u8;
    data[3..7].copy_from_slice(&speed.to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_hold_speed(device: &HidDevice) -> Result<u32, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::ReadConfig as u8;
    data[2] = ConfigElements::HoldSpeed as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]))
    }
}

pub fn set_hold_speed(device: &HidDevice, speed: u32) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::WriteConfig as u8;
    data[2] = ConfigElements::HoldSpeed as u8;
    data[3..7].copy_from_slice(&speed.to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_led_base_color(device: &HidDevice) -> Result<(u8, u8, u8), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetLed as u8;
    data[2] = LedCommand::BaseColor as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok((buf[2], buf[3], buf[4]))
    }
}

pub fn set_led_base_color(device: &HidDevice, color: (u8, u8, u8)) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::SetLed as u8;
    data[2] = LedCommand::BaseColor as u8;
    data[3] = color.0;
    data[4] = color.1;
    data[5] = color.2;
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_led_effect(device: &HidDevice) -> Result<LedEffect, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetLed as u8;
    data[2] = LedCommand::Effect as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(LedEffect::from(buf[2]))
    }
}

pub fn set_led_effect(device: &HidDevice, effect: LedEffect) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::SetLed as u8;
    data[2] = LedCommand::Effect as u8;
    data[3] = effect as u8;
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_led_brightness(device: &HidDevice) -> Result<u8, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetLed as u8;
    data[2] = LedCommand::Brightness as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(buf[2])
    }
}

pub fn set_led_brightness(device: &HidDevice, brightness: u8) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::SetLed as u8;
    data[2] = LedCommand::Brightness as u8;
    data[3] = brightness;
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_led_effect_period(device: &HidDevice) -> Result<f32, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetLed as u8;
    data[2] = LedCommand::EffectPeriod as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(f32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]))
    }
}

pub fn set_led_effect_period(device: &HidDevice, period: f32) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::SetLed as u8;
    data[2] = LedCommand::EffectPeriod as u8;
    data[3..7].copy_from_slice(&period.to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_led_effect_offset(device: &HidDevice) -> Result<f32, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetLed as u8;
    data[2] = LedCommand::EffectOffset as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(f32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]))
    }
}

pub fn set_led_effect_offset(device: &HidDevice, offset: f32) -> Result<(), ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::SetLed as u8;
    data[2] = LedCommand::EffectOffset as u8;
    data[3..7].copy_from_slice(&offset.to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_firmware_version(device: &HidDevice) -> Result<String, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetBuildInfo as u8;
    data[2] = BuildInfoElements::FirmwareVersion as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(String::from_utf8(buf[3..3 + len].to_vec()).unwrap())
    }
}

pub fn get_build_date(device: &HidDevice) -> Result<String, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetBuildInfo as u8;
    data[2] = BuildInfoElements::BuildDate as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(String::from_utf8(buf[3..3 + len].to_vec()).unwrap())
    }
}

pub fn get_build_timestamp(device: &HidDevice) -> Result<String, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetBuildInfo as u8;
    data[2] = BuildInfoElements::BuildTimestamp as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(String::from_utf8(buf[3..3 + len].to_vec()).unwrap())
    }
}

pub fn get_build_profile(device: &HidDevice) -> Result<String, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetBuildInfo as u8;
    data[2] = BuildInfoElements::BuildProfile as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(String::from_utf8(buf[3..3 + len].to_vec()).unwrap())
    }
}

pub fn get_git_hash(device: &HidDevice) -> Result<String, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetBuildInfo as u8;
    data[2] = BuildInfoElements::GitHash as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(String::from_utf8(buf[3..3 + len].to_vec()).unwrap())
    }
}

pub fn get_git_branch(device: &HidDevice) -> Result<String, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetBuildInfo as u8;
    data[2] = BuildInfoElements::GitBranch as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(String::from_utf8(buf[3..3 + len].to_vec()).unwrap())
    }
}

pub fn get_git_semver(device: &HidDevice) -> Result<String, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetBuildInfo as u8;
    data[2] = BuildInfoElements::GitSemver as u8;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(String::from_utf8(buf[3..3 + len].to_vec()).unwrap())
    }
}
