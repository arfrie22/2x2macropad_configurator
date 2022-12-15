use std::{thread::sleep, time::Duration};

use crc::{Crc, CRC_32_CKSUM};
use hidapi::HidApi;

use macropad_protocol::macro_protocol::MacroCommand;

pub const CKSUM: Crc<u32> = Crc::<u32>::new(&CRC_32_CKSUM);
pub const MACRO_SIZE: usize = 4092;

fn main () {
    let mut api = HidApi::new().unwrap();
    api.refresh_devices().unwrap();

    for device in api.device_list() {
        if device.vendor_id() == 4617 && device.product_id() == 1 && device.usage_page() == 0xff00 && device.usage() == 1 {
            println!("Device: {:?}", device);
            let d = device.open_device(&api).unwrap();
            let mut data = [0u8; 65];
            let mut buf = [0u8; 64];
            d.write(&data).unwrap();
            d.read_timeout(&mut buf, 1000).unwrap();

            data[1] = 0x09;
            data[2] = 0x02;
            data[3] = 0x05;
            d.write(&data).unwrap();
            d.read_timeout(&mut buf, 1000).unwrap();
            println!("Wrote: {:?}", data);
            println!("Read: {:?}", buf);


            data[1] = 0x04;
            data[2] = 0x00;
            d.write(&data).unwrap();
            d.read_timeout(&mut buf, 1000).unwrap();
            println!("Wrote: {:?}", data);
            println!("Read: {:?}", buf);
            assert_eq!(data[1], buf[0]);
            

            let mut mac = [0u8; MACRO_SIZE];
            mac[0] = MacroCommand::CommandSetLed as u8;
            mac[1] = 0xFF;
            mac[2] = 0xA7;
            mac[3] = 0x3B;
            mac[4] = MacroCommand::CommandPressKey as u8;
            mac[5] = 0x04;
            mac[6] = MacroCommand::CommandDelay as u8;
            mac[7] = 0xFF;
            mac[8] = 0x0F;
            mac[9] = 0x00;
            mac[10] = 0x00;
            mac[11] = MacroCommand::CommandTerminator as u8;
            mac[12] = MacroCommand::CommandReleaseKey as u8;
            mac[13] = 0x04;
            mac[14] = MacroCommand::CommandTerminator as u8;
            


            println!("Writing macro");
            for i in 0..(MACRO_SIZE/59) {
                let offset: u16 = i as u16 * 59;
                let size = if offset as usize + 59 > MACRO_SIZE { MACRO_SIZE % 59 } else { 59 };
                data[1] = 0x03;
                data[2] = 0x00;
                data[3] = (offset >> 8) as u8;
                data[4] = (offset & 0xFF) as u8;
                data[5] = size as u8;
                data[6..(6 + size)].copy_from_slice(&mac[(i*59)..((i*59) + size)]);
                if data[6..(6+size)] == [0u8; 65][6..(6+size)] {
                    break;
                }
                d.write(&data).unwrap();
                d.read_timeout(&mut buf, 1000).unwrap();
                println!("Wrote: {:?}", data);
                println!("Read: {:?}", buf);
                assert_eq!(data[1..65], buf);
            }

            data = [0u8; 65];
            println!("Reading macro");
            data[1] = 0x02;
            data[2] = 0x00;
            data[3] = 0x00;
            data[4] = 0x00;
            data[5] = 59;
            d.write(&data).unwrap();
            d.read_timeout(&mut buf, 1000).unwrap();
            println!("Wrote: {:?}", data);
            println!("Read: {:?}", buf);


            data = [0u8; 65];

            data[1] = 0x05;
            data[2] = 0x00;
            data[3..7].copy_from_slice(&CKSUM.checksum(&mac).to_be_bytes());

            d.write(&data).unwrap();
            d.read_timeout(&mut buf, 1000).unwrap();
            println!("Wrote: {:?}", data);
            println!("Read: {:?}", buf);
            assert_eq!(data[1], buf[0]);

            data = [0u8; 65];
            println!("Reading macro");
            data[1] = 0x02;
            data[2] = 0x00;
            data[3] = 0x00;
            data[4] = 0x00;
            data[5] = 59;
            d.write(&data).unwrap();
            d.read_timeout(&mut buf, 1000).unwrap();
            println!("Wrote: {:?}", data);
            println!("Read: {:?}", buf);


            for i in 0..4 {
                data = [0u8; 65];
                println!("Change settings");
                data[1] = 0x0B;
                data[2] = 0x00;
                data[3] = i;
                data[4] = 0x02;
                d.write(&data).unwrap();
                d.read_timeout(&mut buf, 1000).unwrap();
                println!("Wrote: {:?}", data);
                println!("Read: {:?}", buf);

                data = [0u8; 65];
                println!("Change settings");
                data[1] = 0x0B;
                data[2] = 0x01;
                data[3] = i;
                data[4] = 0x04;
                d.write(&data).unwrap();
                d.read_timeout(&mut buf, 1000).unwrap();
                println!("Wrote: {:?}", data);
                println!("Read: {:?}", buf);     
                
                data = [0u8; 65];
                println!("Change settings");
                data[1] = 0x0B;
                data[2] = 0x03;
                data[3] = i;
                data[4] = 0xFF;
                data[5] = 0x00;
                data[6] = 0x00;
                d.write(&data).unwrap();
                d.read_timeout(&mut buf, 1000).unwrap();
                println!("Wrote: {:?}", data);
                println!("Read: {:?}", buf);  
            }
        }
    }
}