use file_wizard_backend::search::thread_manager::{spawn_thread, pause_thread, resume_thread, stop_thread};
use file_wizard_backend::search::thread_manager::State;
use file_wizard_backend::search::utils::Search;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct SearchController {
    thread_state: Arc<Mutex<State>>,
    search: Arc<Mutex<Search>>, // The search instance
}

impl SearchController {
    // Constructor for SearchController
    pub fn new() -> Self {
        println!("[SearchController] created");
        let thread_state = Arc::new(Mutex::new(State::Stopped));
        let search = Arc::new(Mutex::new(Search::new()));
        Self {
            thread_state,
            search,
        }
    }

    // Start the search thread
    pub fn start_search(&self) {
        let mut state = match self.thread_state.lock() {
            Ok(lock) => lock,
            Err(poisoned) => {
                eprintln!("[SearchController] Mutex poisoned: {:?}", poisoned);
                return;
            }
        };

        if *state == State::Stopped {
            println!("[SearchController] Starting search thread...");
            let search_clone = Arc::clone(&self.search);
            let state_clone = Arc::clone(&self.thread_state);
            spawn_thread(search_clone, state_clone);
            *state = State::RunningNonInit;
        } else {
            println!("[SearchController] Search thread is already running or paused.");
        }
    }

    // Pause the search thread
    pub fn pause_search(&self) {
        if *self.thread_state.lock().unwrap() == State::RunningInit {
            println!("[SearchController] Pausing search thread...");
            pause_thread(Arc::clone(&self.thread_state));
        } else {
            println!("[SearchController] Cannot pause; thread not running.");
        }
    }

    // Resume the search thread
    pub fn resume_search(&self) {
        if *self.thread_state.lock().unwrap() == State::Paused {
            println!("[SearchController] Resuming search thread...");
            resume_thread(Arc::clone(&self.thread_state));
        } else {
            println!("[SearchController] Cannot resume; thread not paused.");
        }
    }

    // Stop the search thread
    pub fn stop_search(&self) {
        let current_state = self.thread_state.lock().unwrap();
        if *current_state != State::Stopped {
            drop(current_state); // Drop the lock before proceeding
            println!("[SearchController] Stopping search thread...");
            stop_thread(Arc::clone(&self.thread_state));
        } else {
            println!("[SearchController] Search thread is already stopped.");
        }
    }

    // Get the current state of the search thread
    pub fn get_thread_state(&self) -> State {
        *self.thread_state.lock().unwrap()
    }

    pub fn get_curr_directory(&self) -> Option<String> {
        self.search.lock().unwrap().get_curr_directory()
    }

    pub fn set_curr_directory(&self, new_directory: String) {
        self.search.lock().unwrap().set_curr_directory(new_directory);
    }

}