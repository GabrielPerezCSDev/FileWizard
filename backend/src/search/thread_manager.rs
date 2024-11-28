use crate::search::utils::Search;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    RunningInit,
    RunningNonInit,
    Paused,
    Stopped,
}

pub fn spawn_thread(search: Arc<Mutex<Search>>, state: Arc<Mutex<State>>) -> thread::JoinHandle<()> {
    // Create a new thread
    thread::spawn(move || {
        // Thread logic here
        println!("\n\n[Search Thread] started");
        // Add a variable to track whether the pause message was logged
        let mut paused_logged = false;
        loop {
            // Check the thread state
            let current_state = *state.lock().unwrap();
            match current_state {
                State::RunningNonInit => {
                    println!("\n\n[Search Thread] Initializing the search");
                    search.lock().unwrap().initialize_search();
                    // Transition to RunningInit
                    let mut state_guard = state.lock().unwrap();
                    *state_guard = State::RunningInit;
                    paused_logged = false;
                    println!("\n\n[Search Thread] State changed to RunningInit");
                }State::RunningInit => {
                    search.lock().unwrap().execute_search();
                    thread::sleep(Duration::from_secs(1));
                    paused_logged = false;
                }
                State::Paused => {
                    if !paused_logged {
                        println!("\n\n[search thread] paused");
                        paused_logged = true; // Log only once per pause state
                    }
                    thread::sleep(Duration::from_millis(500)); // Sleep while paused
                }
                State::Stopped => {
                    println!("\n\n[search thread] stopped");
                    break; // Exit the loop to terminate the thread
                }
            }
            thread::yield_now(); // Ensure rescheduling
        }
    })
}

pub fn pause_thread(state: Arc<Mutex<State>>) {
    let mut thread_state = state.lock().unwrap();
    *thread_state = State::Paused;
    println!("Thread paused.");
}

pub fn resume_thread(state: Arc<Mutex<State>>) {
    let mut thread_state = state.lock().unwrap();
    *thread_state = State::RunningInit;
    println!("Thread resumed.");
}

pub fn stop_thread(state: Arc<Mutex<State>>) {
    let mut thread_state = state.lock().unwrap();
    *thread_state = State::Stopped;
    println!("Thread stopped.");
}