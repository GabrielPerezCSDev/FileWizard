use backend::search::search_controller::SearchController;
use crate::routers::router::Router;

pub struct SearchRouter {
    controller: SearchController,
}

impl Router for SearchRouter {

    fn new(controller: SearchController) -> Self {
        Self { controller }
    }

    fn initialize(&self) {
        println!("Initializing SearchRouter...");
    }

    fn start(&self) {
        println!("SearchRouter is running!");
        // Actual router logic here
    }
}

