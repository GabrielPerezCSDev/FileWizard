use file_wizard_db::init_db;

pub fn initialize_backend() {
    // Initialize the database
    if let Err(e) = init_db() {
        eprintln!("Failed to initialize the database: {}", e);
    } else {
        println!("Database initialized successfully.");
    }


}