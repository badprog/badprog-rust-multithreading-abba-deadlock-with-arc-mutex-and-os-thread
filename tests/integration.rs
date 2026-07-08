// badprog.com
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

// ------------------------------------
// const
// ------------------------------------
const MESSAGE_EXPECT: &str = "Failed to run binary.";
const DEADLOCK_TIMEOUT: Duration = Duration::from_secs(1);

// ------------------------------------
// macro_rules - binary_path
// Centralize the binary name.
// ------------------------------------
macro_rules! binary_path {
    () => {
        env!("CARGO_BIN_EXE_deadlock-abba")
    };
}

// ------------------------------------
// Test - it - it for integration
// ------------------------------------
mod it {
    use super::*;
    use std::process::Stdio;

    // ------------------------------------
    // test - deadlock_hangs_within_timeout
    // ------------------------------------
    #[test]
    fn ok_deadlock_activated() {
        // 1. Running program in deadlock mode and get the Child.
        let mut child = Command::new(binary_path!())
            .arg("deadlock")
            .stdout(Stdio::null())
            .spawn()
            .expect(MESSAGE_EXPECT);

        // 2. Keep the started time.
        let started_time = Instant::now();

        // 3. Loop to check activity with try_wait() every 10 ms
        loop {
            let status = child.try_wait().expect("Result failed.");
            match status {
                // 4. The Child is dead before the timeout
                Some(exit_status) => {
                    let message = format!("Child is dead (so it has exited): {exit_status}");
                    println!("{message}");
                    panic!("If the child dies, there is thus no deadlock: Not what we expected.");
                }
                // 5. The child is still alive but timeout is reached -> break.
                None if (Instant::now() - started_time) > DEADLOCK_TIMEOUT => {
                    break;
                }
                // 6. The Child is still alive and timeout not being reached yet -> continue.
                None => {
                    println!("Child still alive.");
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }

        // The child is still alive, we have to kill and wait (reap) it.
        child.kill().expect("Child kill not OK.");
        child.wait().expect("Child wait not OK.");
    }
}
