use file_wizard_ports::routers::router::RouterRegistry;
use file_wizard_ports::routers::search_router::SearchRouter;
use file_wizard_db::init_db;

#[actix_web::main]
async fn main() {

    print!("\n\nStarting File Wizard backend tech stack! \n\n");
    // Initialize the database
    if let Err(e) = init_db() {
        eprintln!("Failed to initialize the database: {}", e);
    } else {
        println!("Database initialized successfully.");
    }

    // Set up routers
    let mut router_registry = RouterRegistry::new();
    router_registry.register(Box::new(SearchRouter::new()));
    router_registry.start().await;
}