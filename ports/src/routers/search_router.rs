//packages to be used
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait;

//crates and file imports
use file_wizard_controllers::search::search_controller::SearchController;
use crate::routers::router::Router;

pub struct SearchRouter {
    controller: SearchController,
}

impl SearchRouter {
    // Constructor to create a new SearchRouter
    pub fn new() -> Self {
        println!("[SearchRouter] created");
        let controller = SearchController::new(); // Initialize the controller
        Self { controller }
    }

    // Define your endpoint handlers here
    async fn search_endpoint() -> impl Responder {
        HttpResponse::Ok().body("[SearchRouter] Search endpoint reached!")
    }
}


#[async_trait]
impl Router for SearchRouter {
    async fn initialize(&self) {
        println!("[SearchRouter] Initializing SearchRouter");
        // Start the HTTP server
        
    }

    async fn start(&self) {
        println!("[SearchRouter] SearchRouter is running");

        // Clone the controller if needed
        // let controller = self.controller.clone();

        // Start the HTTP server
        HttpServer::new(|| {
            App::new()
                .route("/search", web::get().to(Self::search_endpoint))
                // Add more routes here
        })
        .bind("127.0.0.1:8080")
        .expect("Failed to bind to address")
        .run()
        .await
        .expect("Failed to run HTTP server");
    }
}


