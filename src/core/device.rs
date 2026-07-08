// badprog.com

// lib
use std::fmt::{self};

// const
const NUMBER_OF_REGISTERS: usize = 3; // array length can only be usize

// enum
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeviceKind {
    I2c,
    Spi,
}

// struct
#[derive(Debug)]
pub struct Device {
    name: String,
    registers: [u8; NUMBER_OF_REGISTERS],
    usage_count: i32,
}

// --------------------------------
// --------------------------------
// --------------------------------
// --------------------------------
// impl
// --------------------------------
// --------------------------------
// --------------------------------
// --------------------------------

// --------------------------------
// Display for Device
// Format the output like this:
// device [I²C] [0x00, 0xAA, 0xEE] [1000]
// --------------------------------
impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let registers_hex = self
            .registers
            .iter()
            .map(|r| format!("0x{:02X}", r))
            .collect::<Vec<String>>()
            .join(", ");
        write!(
            f,
            "device [{}] [{}] [{}]",
            self.name, registers_hex, self.usage_count,
        )
    }
}

// --------------------------------
// Device
// --------------------------------
impl Device {
    // pub const REG_STATUS: usize = 0; // not used in this example
    pub const REG_TO_READ: usize = 1; // data read from this device
    pub const REG_TO_WRITE: usize = 2; // data received from other devices

    // --------------------------------
    // new
    // --------------------------------
    pub fn new(name: &str, registers: [u8; NUMBER_OF_REGISTERS]) -> Self {
        Device {
            name: name.to_string(),
            registers,
            usage_count: 0,
        }
    }

    // --------------------------------
    // read_register
    // --------------------------------
    pub fn read_register(&self, index: usize) -> u8 {
        self.registers[index]
    }

    // --------------------------------
    // write_register
    // --------------------------------
    pub fn write_register(&mut self, index: usize, value: u8) {
        self.registers[index] = value;
        self.usage_count += 1;
    }
}

// --------------------------------
// --------------------------------
// --------------------------------
// --------------------------------
// --------------------------------
// test
// --------------------------------
// --------------------------------
// --------------------------------
// --------------------------------
// --------------------------------
#[cfg(test)]
// --------------------------------
// mod ut
// --------------------------------
mod ut {
    use super::*;

    // --------------------------------
    // mod impl_device
    // --------------------------------
    mod impl_display_for_device {
        use super::*;

        #[test]
        fn ok_format_is_stable() {
            let device = Device::new("SPI", [0x00, 0xAA, 0xBB]);
            assert_eq!(device.to_string(), "device [SPI] [0x00, 0xAA, 0xBB] [0]");
        }

        #[test]
        fn ok_count_usage_is_displayed() {
            let mut device = Device::new("SPI", [0x00, 0xAA, 0xBB]);
            let max = 1000;
            for i in 0..max {
                assert!(device.to_string().ends_with(&format!("[{}]", i)));
                device.write_register(Device::REG_TO_WRITE, 0xCC);
            }
            assert!(device.to_string().ends_with(&format!("[{}]", max)));
        }

        #[test]
        fn ok_name_is_displayed() {
            let name = "SPI";
            let device = Device::new(name, [0x00, 0xAA, 0xBB]);
            assert!(device.to_string().contains(name));
        }
    }

    // --------------------------------
    // mod impl_device
    // --------------------------------
    mod impl_device {
        use super::*;
        // --------------------------------
        // mod new
        // --------------------------------
        mod new {
            use super::*;

            #[test]
            fn ok_device_created() {
                let device1 = Device::new("SPI", [0x00, 0xAA, 0xBB]);
                let device2 = Device::new("I2C", [0x00, 0xCC, 0xDD]);
                //
                assert_eq!(device1.read_register(0), 0x00);
                assert_eq!(device1.read_register(Device::REG_TO_READ), 0xAA);
                assert_eq!(device1.read_register(Device::REG_TO_WRITE), 0xBB);
                assert_eq!(device2.read_register(0), 0x00);
                assert_eq!(device2.read_register(Device::REG_TO_READ), 0xCC);
                assert_eq!(device2.read_register(Device::REG_TO_WRITE), 0xDD);
            }
        }

        // --------------------------------
        // mod read_register
        // --------------------------------
        mod read_register {
            use crate::core::device::{Device, NUMBER_OF_REGISTERS};

            #[test]
            fn ok_with_all_same_values() {
                let value = 0xBB;
                let device = Device::new("SPI", [value; NUMBER_OF_REGISTERS]);
                let result = device.read_register(Device::REG_TO_READ);
                assert_eq!(result, value);
            }

            #[test]
            fn ok_with_different_values_on_each_register() {
                let device = Device::new("SPI", [0xAA, 0xBB, 0xCC]);
                //
                assert_eq!(device.read_register(Device::REG_TO_READ), 0xBB);
                assert_eq!(device.read_register(Device::REG_TO_WRITE), 0xCC);
            }

            #[test]
            fn ko_with_different_values_on_each_register() {
                let device = Device::new("SPI", [0xAA, 0xBB, 0xCC]);
                //
                assert_ne!(device.read_register(Device::REG_TO_READ), 0xFF);
                assert_ne!(device.read_register(Device::REG_TO_WRITE), 0xFF);
            }
        }

        // --------------------------------
        // mod write_register
        // --------------------------------
        mod write_register {
            use super::*;

            #[test]
            fn ok_register_updated() {
                let value = 0xCC;
                let mut device = Device::new("SPI", [0x00, 0xAA, 0xBB]);
                device.write_register(Device::REG_TO_WRITE, value);
                //
                assert_eq!(device.read_register(0), 0x00);
                assert_eq!(device.read_register(Device::REG_TO_READ), 0xAA);
                assert_eq!(device.read_register(Device::REG_TO_WRITE), value);
            }

            #[test]
            #[should_panic(expected = "index out of bounds")] // expected from the native panic
            fn ko_register_out_of_bounds() {
                let value = 0xCC;
                let wrong_index = NUMBER_OF_REGISTERS;
                let mut device = Device::new("SPI", [0x00, 0xAA, 0xBB]);
                //
                device.write_register(wrong_index, value);
            }
        }
    }
}
