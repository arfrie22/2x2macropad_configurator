use crc::{Crc, CRC_32_CKSUM};
use hidapi::HidDevice;
use macropad_protocol::data_protocol::{
    BuildInfoElements, ConfigElements, DataCommand, KeyConfigElements, KeyMode, LedCommand,
    LedEffect,
};
use semver::Version;
use usbd_human_interface_device::page::{Consumer, Keyboard};

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

pub fn get_build_version(device: &HidDevice) -> Result<String, ()> {
    let mut data = [0u8; 65];
    data[1] = DataCommand::GetBuildVersion as u8;
    let buf = send_command(device, data)?;

    if data[1] != buf[0] {
        Err(())
    } else {
        let len = buf[1] as usize;
        Ok(String::from_utf8(buf[2..2 + len].to_vec()).unwrap())
    }
}

pub fn enter_bootloader(device: &HidDevice, version: &Version) -> Result<(), ()> {
    // Exists in all versions
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::EnterBootloader.value_for_version(version) {
        data[1] = value;
    } else {
        return Err(());
    }
    send_command(device, data)?;

    Ok(())
}

pub fn get_key_mode(device: &HidDevice, version: &Version, index: u8) -> Result<Option<KeyMode>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::ReadKeyConfig.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = KeyConfigElements::KeyMode.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }

    data[3] = index;
    let buf = send_command(device, data)?;

    if data[1..4] != buf[0..3] {
        Err(())
    } else {
        Ok(Some(KeyMode::from(buf[3])))
    }
}

pub fn set_key_mode(device: &HidDevice, version: &Version, index: u8, mode: KeyMode) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::WriteKeyConfig.value_for_version(&version) {
        data[1] = value;
    } else {
        return Err(());
    }

    if let Some(value) = KeyConfigElements::KeyMode.value_for_version(&version) {
        data[2] = value;
    } else {
        return Err(());
    }

    data[3] = index;
    data[4] = mode as u8;
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_keyboard_data(device: &HidDevice, version: &Version, index: u8) -> Result<Option<Keyboard>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::ReadKeyConfig.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = KeyConfigElements::KeyboardData.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }

    data[3] = index;
    let buf = send_command(device, data)?;

    if data[1..4] != buf[0..3] {
        Err(())
    } else {
        Ok(Some(Keyboard::from(buf[3])))
    }
}

pub fn set_keyboard_data(device: &HidDevice, version: &Version, index: u8, keyboard: Keyboard) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::WriteKeyConfig.value_for_version(&version) {
        data[1] = value;
    } else {
        return Err(());
    }

    if let Some(value) = KeyConfigElements::KeyboardData.value_for_version(&version) {
        data[2] = value;
    } else {
        return Err(());
    }

    data[3] = index;
    data[4] = keyboard as u8;
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_consumer_data(device: &HidDevice, version: &Version, index: u8) -> Result<Option<Consumer>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::ReadKeyConfig.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = KeyConfigElements::ConsumerData.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }

    data[3] = index;
    let buf = send_command(device, data)?;

    if data[1..4] != buf[0..3] {
        Err(())
    } else {
        Ok(Some(Consumer::from(u16::from_le_bytes([buf[3], buf[4]]))))
    }
}

pub fn set_consumer_data(device: &HidDevice, version: &Version, index: u8, consumer: Consumer) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::WriteKeyConfig.value_for_version(&version) {
        data[1] = value;
    } else {
        return Err(());
    }
    
    if let Some(value) = KeyConfigElements::ConsumerData.value_for_version(&version) {
        data[2] = value;
    } else {
        return Err(());
    }

    data[3] = index;
    data[4..6].copy_from_slice(&(consumer as u16).to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_key_color(device: &HidDevice, version: &Version, index: u8) -> Result<Option<(u8, u8, u8)>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::ReadKeyConfig.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = KeyConfigElements::KeyColor.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }

    data[3] = index;
    let buf = send_command(device, data)?;

    if data[1..4] != buf[0..3] {
        Err(())
    } else {
        Ok(Some((buf[3], buf[4], buf[5])))
    }
}

pub fn set_key_color(device: &HidDevice, version: &Version, index: u8, color: (u8, u8, u8)) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::WriteKeyConfig.value_for_version(&version) {
        data[1] = value;
    } else {
        return Err(());
    }

    if let Some(value) = KeyConfigElements::KeyColor.value_for_version(&version) {
        data[2] = value;
    } else {
        return Err(());
    }

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

pub fn get_macro(device: &HidDevice, version: &Version, index: u8) -> Result<Option<[u8; 4092]>, ()> {
    let mut output = [0u8; 4092];
    for i in 0..(MACRO_SIZE / 59) {
        let mut data = [0u8; 65];
        let offset: u16 = i as u16 * 59;
        let size = if offset as usize + 59 > MACRO_SIZE {
            MACRO_SIZE % 59
        } else {
            59
        };

        if let Some(value) = DataCommand::ReadMacro.value_for_version(version) {
            data[1] = value;
        } else {
            return Ok(None);
        }

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

    Ok(Some(output))
}

pub fn set_macro(device: &HidDevice, version: &Version, index: u8, macro_data: &[u8; 4092]) -> Result<(), ()> {
    for i in 0..(MACRO_SIZE / 59) {
        let mut data = [0u8; 65];
        let offset: u16 = i as u16 * 59;
        let size = if offset as usize + 59 > MACRO_SIZE {
            MACRO_SIZE % 59
        } else {
            59
        };
        if let Some(value) = DataCommand::WriteMacro.value_for_version(version) {
            data[1] = value;
        } else {
            return Err(());
        }

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

pub fn validate_macro(device: &HidDevice, version: &Version, index: u8, macro_data: &[u8; 4092]) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::ValidateMacro.value_for_version(version) {
        data[1] = value;
    } else {
        return Err(());
    }
    data[2] = index;
    data[3..7].copy_from_slice(&CKSUM.checksum(macro_data).to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] || buf[2..6] != buf[6..10] {
        Err(())
    } else {
        Ok(())
    }
}

pub fn clear_macro(device: &HidDevice, version: &Version, index: u8) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::ClearMacro.value_for_version(version) {
        data[1] = value;
    } else {
        return Err(());
    }
    data[2] = index;
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_tap_speed(device: &HidDevice, version: &Version) -> Result<Option<u32>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::ReadConfig.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = ConfigElements::TapSpeed.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(Some(u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]])))
    }
}

pub fn set_tap_speed(device: &HidDevice, version: &Version, speed: u32) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::WriteConfig.value_for_version(version) {
        data[1] = value;
    } else {
        return Err(());
    }

    if let Some(value) = ConfigElements::TapSpeed.value_for_version(version) {
        data[2] = value;
    } else {
        return Err(());
    }
    data[3..7].copy_from_slice(&speed.to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_hold_speed(device: &HidDevice, version: &Version) -> Result<Option<u32>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::ReadConfig.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = ConfigElements::HoldSpeed.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(Some(u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]])))
    }
}

pub fn set_hold_speed(device: &HidDevice, version: &Version, speed: u32) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::WriteConfig.value_for_version(version) {
        data[1] = value;
    } else {
        return Err(());
    }

    if let Some(value) = ConfigElements::HoldSpeed.value_for_version(version) {
        data[2] = value;
    } else {
        return Err(());
    }
    data[3..7].copy_from_slice(&speed.to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_led_base_color(device: &HidDevice, version: &Version) -> Result<Option<(u8, u8, u8)>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetLed.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = LedCommand::BaseColor.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(Some((buf[2], buf[3], buf[4])))
    }
}

pub fn set_led_base_color(device: &HidDevice, version: &Version, color: (u8, u8, u8)) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::SetLed.value_for_version(version) {
        data[1] = value;
    } else {
        return Err(());
    }
    
    if let Some(value) = LedCommand::BaseColor.value_for_version(version) {
        data[2] = value;
    } else {
        return Err(());
    }
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

pub fn get_led_effect(device: &HidDevice, version: &Version) -> Result<Option<LedEffect>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetLed.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = LedCommand::Effect.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(Some(LedEffect::from(buf[2])))
    }
}

pub fn set_led_effect(device: &HidDevice, version: &Version, effect: LedEffect) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::SetLed.value_for_version(version) {
        data[1] = value;
    } else {
        return Err(());
    }

    if let Some(value) = LedCommand::Effect.value_for_version(version) {
        data[2] = value;
    } else {
        return Err(());
    }
    data[3] = effect as u8;
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_led_brightness(device: &HidDevice, version: &Version) -> Result<Option<u8>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetLed.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = LedCommand::Brightness.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(Some(buf[2]))
    }
}

pub fn set_led_brightness(device: &HidDevice, version: &Version, brightness: u8) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::SetLed.value_for_version(version) {
        data[1] = value;
    } else {
        return Err(());
    }

    if let Some(value) = LedCommand::Brightness.value_for_version(version) {
        data[2] = value;
    } else {
        return Err(());
    }
    data[3] = brightness;
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_led_effect_period(device: &HidDevice, version: &Version) -> Result<Option<f32>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetLed.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = LedCommand::EffectPeriod.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(Some(f32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]])))
    }
}

pub fn set_led_effect_period(device: &HidDevice, version: &Version, period: f32) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::SetLed.value_for_version(version) {
        data[1] = value;
    } else {
        return Err(());
    }

    if let Some(value) = LedCommand::EffectPeriod.value_for_version(version) {
        data[2] = value;
    } else {
        return Err(());
    }
    data[3..7].copy_from_slice(&period.to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_led_effect_offset(device: &HidDevice, version: &Version) -> Result<Option<f32>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetLed.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = LedCommand::EffectOffset.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        Ok(Some(f32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]])))
    }
}

pub fn set_led_effect_offset(device: &HidDevice, version: &Version, offset: f32) -> Result<(), ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::SetLed.value_for_version(version) {
        data[1] = value;
    } else {
        return Err(());
    }

    if let Some(value) = LedCommand::EffectOffset.value_for_version(version) {
        data[2] = value;
    } else {
        return Err(());
    }
    data[3..7].copy_from_slice(&offset.to_le_bytes());
    let buf = send_command(device, data)?;

    if data[1..65] != buf {
        Err(())
    } else {
        Ok(())
    }
}

pub fn get_firmware_version(device: &HidDevice, version: &Version) -> Result<Option<String>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetBuildInfo.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = BuildInfoElements::FirmwareVersion.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(Some(String::from_utf8(buf[3..3 + len].to_vec()).unwrap()))
    }
}

pub fn get_build_date(device: &HidDevice, version: &Version) -> Result<Option<String>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetBuildInfo.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = BuildInfoElements::BuildDate.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(Some(String::from_utf8(buf[3..3 + len].to_vec()).unwrap()))
    }
}

pub fn get_build_timestamp(device: &HidDevice, version: &Version) -> Result<Option<String>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetBuildInfo.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = BuildInfoElements::BuildTimestamp.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(Some(String::from_utf8(buf[3..3 + len].to_vec()).unwrap()))
    }
}

pub fn get_build_profile(device: &HidDevice, version: &Version) -> Result<Option<String>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetBuildInfo.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = BuildInfoElements::BuildProfile.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(Some(String::from_utf8(buf[3..3 + len].to_vec()).unwrap()))
    }
}

pub fn get_git_hash(device: &HidDevice, version: &Version) -> Result<Option<String>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetBuildInfo.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = BuildInfoElements::GitHash.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(Some(String::from_utf8(buf[3..3 + len].to_vec()).unwrap()))
    }
}

pub fn get_git_branch(device: &HidDevice, version: &Version) -> Result<Option<String>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetBuildInfo.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = BuildInfoElements::GitBranch.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(Some(String::from_utf8(buf[3..3 + len].to_vec()).unwrap()))
    }
}

pub fn get_git_semver(device: &HidDevice, version: &Version) -> Result<Option<String>, ()> {
    let mut data = [0u8; 65];
    if let Some(value) = DataCommand::GetBuildInfo.value_for_version(version) {
        data[1] = value;
    } else {
        return Ok(None);
    }

    if let Some(value) = BuildInfoElements::GitSemver.value_for_version(version) {
        data[2] = value;
    } else {
        return Ok(None);
    }
    let buf = send_command(device, data)?;

    if data[1..3] != buf[0..2] {
        Err(())
    } else {
        let len = buf[2] as usize;
        Ok(Some(String::from_utf8(buf[3..3 + len].to_vec()).unwrap()))
    }
}
