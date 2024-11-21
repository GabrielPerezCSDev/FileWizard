use file_wizard_backend::initialize_backend;

fn main() {
    // Initialize the backend (and database)
    initialize_backend();

    println!("File Wizard application is running!");
}