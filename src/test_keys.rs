use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Key detector - press any key to see its Keycode:");
    println!("Press Ctrl+C to exit");

    let device_state = DeviceState::new();
    let mut prev_keys: Vec<Keycode> = Vec::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        // Check for newly pressed keys
        for key in &keys {
            if !prev_keys.contains(key) {
                println!("Key pressed: {:?}", key);
            }
        }

        prev_keys = keys;
        thread::sleep(Duration::from_millis(50));
    }
}
