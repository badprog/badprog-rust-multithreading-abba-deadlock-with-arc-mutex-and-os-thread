// badprog.com

// lib
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

// crate
use crate::core::device::{Device, DeviceKind};

// const
pub const ERR_SAME_DEVICE: &str = "'from' and 'to' must be different devices";

// ------------------------------------
// transfer_data
// 'from' is the source device (where to read the register).
// 'to' is the destination device (where to write the register).
// ------------------------------------
pub fn transfer_data(
    devices: &[Mutex<Device>],
    from: DeviceKind,
    to: DeviceKind,
    deadlock_mode: bool,
) {
    //
    assert_ne!(from, to, "{}", ERR_SAME_DEVICE);

    //
    let device_to_read = from as usize;
    let device_to_write = to as usize;
    const MILLIS_1: u64 = 1;

    //
    match from {
        DeviceKind::I2c => {
            let device_i2c = devices[device_to_read].lock().unwrap();
            let mut device_spi = devices[device_to_write].lock().unwrap();
            device_spi.write_register(
                Device::REG_TO_WRITE,
                device_i2c.read_register(Device::REG_TO_READ),
            );
        }
        DeviceKind::Spi => {
            let (device_spi, mut device_i2c) = if deadlock_mode {
                let spi = devices[device_to_read].lock().unwrap();
                thread::sleep(Duration::from_millis(MILLIS_1));
                println!("The program is in deadlock_mode (CTRL + C to leave).");
                let i2c = devices[device_to_write].lock().unwrap();
                println!("The line can't be reached due to the deadlock.");
                (spi, i2c)
            } else {
                let i2c = devices[device_to_write].lock().unwrap();
                let spi = devices[device_to_read].lock().unwrap();
                (spi, i2c)
            };
            device_i2c.write_register(
                Device::REG_TO_WRITE,
                device_spi.read_register(Device::REG_TO_READ),
            );
        }
    }
}

// ------------------------------------
// is_deadlock_allowed
// Check if args[1] is "deadlock".
// ------------------------------------
pub fn is_deadlock_allowed(args: &[String]) -> bool {
    args.get(1).map(|s| s.as_str()) == Some("deadlock")
}

// ------------------------------------
// create_thread
// Create a thread then call 1000 times the transfer_data.
// ------------------------------------
pub fn create_thread(
    arc_to_share: &Arc<Vec<Mutex<Device>>>,
    from: DeviceKind,
    to: DeviceKind,
    deadlock_mode: bool,
) -> JoinHandle<()> {
    let arc_cloned = Arc::clone(arc_to_share);
    thread::spawn(move || {
        for _ in 0..1000 {
            transfer_data(&arc_cloned, from, to, deadlock_mode);
        }
    })
}

// ------------------------------------
// run
// ------------------------------------
pub fn run(args: &[String]) {
    //
    let deadlock_mode = is_deadlock_allowed(args);
    let device_i2c = Device::new("I²C", [0x00, 0xAA, 0x00]);
    let device_spi = Device::new("SPI", [0x01, 0xEE, 0x00]);
    let vec_devices = vec![Mutex::new(device_i2c), Mutex::new(device_spi)];
    let arc_to_share = Arc::new(vec_devices);
    let mut handles = Vec::new();

    //
    println!("At the beginning:");

    // Display elements in the container with raw data
    for mutexed_device in arc_to_share.iter() {
        let guard = mutexed_device.lock().unwrap();
        println!("guard = {guard}");
    }

    // jh = JoinHandle<T> where T is the type returned by the closure in the thread.
    // In our case, the thread only loop, so returns the unit type "()" meaning void
    // Thread 1.
    let jh1 = create_thread(
        &arc_to_share,
        DeviceKind::I2c,
        DeviceKind::Spi,
        deadlock_mode,
    );
    handles.push(jh1); // jh is immediately available (not necessary the thread itself)

    // Thread 2.
    let jh2 = create_thread(
        &arc_to_share,
        DeviceKind::Spi,
        DeviceKind::I2c,
        deadlock_mode,
    );
    handles.push(jh2);

    // Wait for all threads to finish.
    for jh in handles {
        jh.join().unwrap(); // .join() blocks until the current jh (thread) has finished
    }
    // This line is reached only if all threads have finished.

    //
    println!("At the end:");

    // Display elements in the container with modified data
    for mutexed_device in arc_to_share.iter() {
        let guard = mutexed_device.lock().unwrap();
        println!("guard = {guard}");
    }
}

// ------------------------------------
// ------------------------------------
// ------------------------------------
// ------------------------------------
// ------------------------------------
// tests - Only launched with cargo test
// ------------------------------------
// ------------------------------------
// ------------------------------------
// ------------------------------------
// ------------------------------------
#[cfg(test)]
// --------------------------------
// mod ut
// --------------------------------
mod ut {
    use super::*;

    // --------------------------------
    // mod is_deadlock_allowed
    // --------------------------------
    mod is_deadlock_allowed {
        use super::*;

        #[test]
        fn deadlock_flag_present() {
            let args = vec!["program".to_string(), "deadlock".to_string()];
            let result = is_deadlock_allowed(&args);
            assert!(result);
        }

        #[test]
        fn deadlock_flag_empty() {
            let args = vec!["program".to_string(), "".to_string()];
            let result = !is_deadlock_allowed(&args);
            assert!(result);
        }

        #[test]
        fn deadlock_flag_absent() {
            let args = vec!["program".to_string()];
            let result = !is_deadlock_allowed(&args);
            assert!(result);
        }

        #[test]
        fn deadlock_flag_wrong_word() {
            let args = vec!["program".to_string(), "helloWorld".to_string()];
            let result = !is_deadlock_allowed(&args);
            assert!(result);
        }
    }

    // --------------------------------
    // mod transfer_data
    // --------------------------------
    mod transfer_data {
        use super::*;

        #[test]
        #[should_panic(expected = "must be different")] // needs string literal (no const allowed)
        fn from_and_to_must_be_different() {
            let devices = vec![
                Mutex::new(Device::new("I2C", [0; 3])),
                Mutex::new(Device::new("SPI", [0; 3])),
            ];
            let deadlock_mode = false;
            // the following function is going to trigger the assert within
            transfer_data(&devices, DeviceKind::I2c, DeviceKind::I2c, deadlock_mode);
        }
    }

    // --------------------------------
    // mod create_thread
    // --------------------------------
    mod create_thread {
        use super::*;

        #[test]
        fn ok_from_i2c_to_spi() {
            let devices = vec![
                Mutex::new(Device::new("I2C", [0x00, 0xAA, 0xBB])),
                Mutex::new(Device::new("SPI", [0x01, 0xCC, 0xDD])),
            ];
            let deadlock_mode = false;
            let arc_to_share = Arc::new(devices);
            let from = DeviceKind::I2c;
            let to = DeviceKind::Spi;

            // create thread
            let jh = create_thread(&arc_to_share, from, to, deadlock_mode);
            jh.join().unwrap(); // wait the thread to finish

            // lock spi device (destination)
            let guard_for_spi = arc_to_share[1].lock().unwrap();
            //
            assert_eq!(guard_for_spi.read_register(Device::REG_TO_READ), 0xCC);
            assert_eq!(guard_for_spi.read_register(Device::REG_TO_WRITE), 0xAA);
            assert!(guard_for_spi.to_string().ends_with("[1000]"));
            assert!(guard_for_spi.to_string().contains("SPI"));
        }

        #[test]
        fn ok_from_spi_to_i2c() {
            let devices = vec![
                Mutex::new(Device::new("I2C", [0x00, 0xAA, 0xBB])),
                Mutex::new(Device::new("SPI", [0x01, 0xCC, 0xDD])),
            ];
            let deadlock_mode = false;
            let arc_to_share = Arc::new(devices);
            let from = DeviceKind::Spi;
            let to = DeviceKind::I2c;

            // create thread
            let jh = create_thread(&arc_to_share, from, to, deadlock_mode);
            jh.join().unwrap(); // wait the thread to finish

            // lock i2c device (destination)
            let guard_for_i2c = arc_to_share[0].lock().unwrap();
            //
            assert_eq!(guard_for_i2c.read_register(Device::REG_TO_READ), 0xAA);
            assert_eq!(guard_for_i2c.read_register(Device::REG_TO_WRITE), 0xCC);
            assert!(guard_for_i2c.to_string().ends_with("[1000]"));
            assert!(guard_for_i2c.to_string().contains("I2C"));
        }

        #[test]
        fn ok_two_successive_transfers_back_and_forth() {
            let devices = vec![
                Mutex::new(Device::new("I2C", [0x00, 0xAA, 0xBB])),
                Mutex::new(Device::new("SPI", [0x01, 0xCC, 0xDD])),
            ];
            let deadlock_mode = false;
            let arc_to_share = Arc::new(devices);
            let from = DeviceKind::Spi;
            let to = DeviceKind::I2c;

            // create thread
            let jh = create_thread(&arc_to_share, from, to, deadlock_mode);
            jh.join().unwrap();

            // lock
            let guard_for_i2c = arc_to_share[0].lock().unwrap();
            //
            assert_eq!(guard_for_i2c.read_register(Device::REG_TO_READ), 0xAA);
            assert_eq!(guard_for_i2c.read_register(Device::REG_TO_WRITE), 0xCC);
            assert!(guard_for_i2c.to_string().ends_with("[1000]"));
            assert!(guard_for_i2c.to_string().contains("I2C"));

            // drop in order to prevent the deadlock
            drop(guard_for_i2c);

            //
            let from = DeviceKind::I2c;
            let to = DeviceKind::Spi;

            // create thread
            let jh = create_thread(&arc_to_share, from, to, deadlock_mode);
            jh.join().unwrap();

            // lock
            let guard_for_spi = arc_to_share[1].lock().unwrap();
            //
            assert_eq!(guard_for_spi.read_register(Device::REG_TO_READ), 0xCC);
            assert_eq!(guard_for_spi.read_register(Device::REG_TO_WRITE), 0xAA);
            assert!(guard_for_spi.to_string().ends_with("[1000]"));
            assert!(guard_for_spi.to_string().contains("SPI"));
        }
    }

    // --------------------------------
    // mod create_thread
    // --------------------------------
}
