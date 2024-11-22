use file_wizard_db::init_db;
use ports::routers::{RouterRegistry, SearchRouter};
use backend::search::search_controller::SearchController;
mod search;
//mod general;

pub fn initialize_backend() {
    // Initialize the database
    if let Err(e) = init_db() {
        eprintln!("Failed to initialize the database: {}", e);
    } else {
        println!("Database initialized successfully.");
    }

    // Create controllers
    let search_controller = SearchController::new();

    // Set up routers
    let search_router = SearchRouter::new(search_controller); // Pass controller to router
    let mut router_registry = RouterRegistry::new();
    router_registry.register(Box::new(search_router)); // Add router to registry
    router_registry.start(); // Start all registered routers

}