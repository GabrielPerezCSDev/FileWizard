use crate::search::utils::Search;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Running,
    Paused,
    Stopped,
}

pub fn spawn_thread(search: Arc<Mutex<Search>>, state: Arc<Mutex<State>>) -> thread::JoinHandle<()> {
    // Create a new thread
    thread::spawn(move || {
        // Thread logic here
        println!("[search thread] started");
        loop {
            // Check the thread state
            let current_state = *state.lock().unwrap();
            match current_state {
                State::Running => {
                    search.lock().unwrap().execute_search();
                    thread::sleep(Duration::from_secs(1));
                }
                State::Paused => {
                    println!("[search thread] paused");
                    thread::sleep(Duration::from_millis(500)); // Sleep while paused
                }
                State::Stopped => {
                    println!("[search thread] stopped");
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
    *thread_state = State::Running;
    println!("Thread resumed.");
}

pub fn stop_thread(state: Arc<Mutex<State>>) {
    let mut thread_state = state.lock().unwrap();
    *thread_state = State::Stopped;
    println!("Thread stopped.");
}