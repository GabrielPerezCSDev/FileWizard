use file_wizard_backend::initialize_backend;
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize the backend (and database)
    initialize_backend();

    // Keep the backend running
    println!("Rust handler running. Press Ctrl+C to exit.");
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}