use file_wizard_db::init_db;
mod search;
use search::thread_manager::spawn_thread as search_thread;
//mod general;

pub fn initialize_backend() {
    // Initialize the database
    if let Err(e) = init_db() {
        eprintln!("Failed to initialize the database: {}", e);
    } else {
        println!("Database initialized successfully.");
    }

    
    // Spawn threads for search and general
    search_thread();
    //general::thread_manager::spawn_thread();

}