//packages to be used
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait;
use std::sync::{Mutex, Arc};
use serde::Deserialize;
//crates and file imports
use file_wizard_controllers::search::search_controller::SearchController;
use crate::routers::router::Router;

#[derive(Deserialize)]
struct SetDirectoryRequest {
    new_directory: String,
}

pub struct SearchRouter {
    controller: Arc<Mutex<SearchController>>,
}

impl SearchRouter {
    // Constructor to create a new SearchRouter
    pub fn new() -> Self {
        println!("[SearchRouter] created");
        let controller = Arc::new(Mutex::new(SearchController::new())); // Initialize the controller
        Self { controller }
    }

    // Define your endpoint handlers here
    async fn search_endpoint() -> impl Responder {
        HttpResponse::Ok().body("[SearchRouter] Search endpoint reached!")
    }

    // Define your endpoint handlers here
    async fn search_start(controller: web::Data<Arc<Mutex<SearchController>>>) -> impl Responder {
        // Clone the Arc to move into the closure
        let controller_clone = Arc::clone(&controller);

        // Offload the blocking operation to a thread pool
        let result = web::block(move || {
            // Attempt to lock the Mutex
            let controller = controller_clone.lock().unwrap();
            controller.start_search();
        })
        .await;

        match result {
            Ok(_) => HttpResponse::Ok().body("[SearchRouter] Search operation initiated!"),
            Err(e) => {
                eprintln!("[SearchRouter] Error initiating search: {:?}", e);
                HttpResponse::InternalServerError().body("Failed to initiate search.")
            }
        }
    }


     // Define your endpoint handlers here
     async fn search_stop(controller: web::Data<Arc<Mutex<SearchController>>>) -> impl Responder {
        // Clone the Arc to move into the closure
        let controller_clone = Arc::clone(&controller);

        // Offload the blocking operation to a thread pool
        let result = web::block(move || {
            // Attempt to lock the Mutex
            let controller = controller_clone.lock().unwrap();
            controller.stop_search();
        })
        .await;

        match result {
            Ok(_) => HttpResponse::Ok().body("[SearchRouter] Stopped search operation"),
            Err(e) => {
                eprintln!("[SearchRouter] Error stopping search: {:?}", e);
                HttpResponse::InternalServerError().body("Failed to stop search.")
            }
        }
    }

    // Define your endpoint handlers here
    async fn search_resume(controller: web::Data<Arc<Mutex<SearchController>>>) -> impl Responder {
        // Clone the Arc to move into the closure
        let controller_clone = Arc::clone(&controller);

        // Offload the blocking operation to a thread pool
        let result = web::block(move || {
            // Attempt to lock the Mutex
            let controller = controller_clone.lock().unwrap();
            controller.resume_search();
        })
        .await;

        match result {
            Ok(_) => HttpResponse::Ok().body("[SearchRouter] Resumed search operation"),
            Err(e) => {
                eprintln!("[SearchRouter] Error resuming search: {:?}", e);
                HttpResponse::InternalServerError().body("Failed to resume search.")
            }
        }
    }

    // Define your endpoint handlers here
    async fn search_pause(controller: web::Data<Arc<Mutex<SearchController>>>) -> impl Responder {
        // Clone the Arc to move into the closure
        let controller_clone = Arc::clone(&controller);

        // Offload the blocking operation to a thread pool
        let result = web::block(move || {
            // Attempt to lock the Mutex
            let controller = controller_clone.lock().unwrap();
            controller.pause_search();
        })
        .await;

        match result {
            Ok(_) => HttpResponse::Ok().body("[SearchRouter] Resumed search operation"),
            Err(e) => {
                eprintln!("[SearchRouter] Error resuming search: {:?}", e);
                HttpResponse::InternalServerError().body("Failed to resume search.")
            }
        }
    }

    async fn set_root_search_directory(
        controller: web::Data<Arc<Mutex<SearchController>>>,
        item: web::Json<SetDirectoryRequest>,
    ) -> impl Responder {
        let new_dir = item.into_inner().new_directory;

        // Clone the Arc to move into the closure
        let controller_clone = Arc::clone(&controller);

        // Offload the blocking operation to a thread pool
        let result = web::block(move || {
            let controller = controller_clone.lock().unwrap();
            controller.set_root_search_directory(new_dir);
        })
        .await;

        match result {
            Ok(_) => HttpResponse::Ok().body("[SearchRouter] Current directory set successfully!"),
            Err(e) => {
                eprintln!("[SearchRouter] Error setting directory: {:?}", e);
                HttpResponse::InternalServerError().body("Failed to set current directory.")
            }
        }
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

         // Clone the Arc to pass into the HTTP server closure
         let controller_data = web::Data::new(Arc::clone(&self.controller));

        // Start the HTTP server
        HttpServer::new(move || {
            App::new()
                .app_data(controller_data.clone()) // Share the controller with all handlers
                .route("/search", web::get().to(Self::search_endpoint))
                .route("/search/start", web::get().to(Self::search_start))
                .route("/search/stop", web::get().to(Self::search_stop))
                .route("/search/resume", web::get().to(Self::search_resume))
                .route("/search/pause", web::get().to(Self::search_pause))
                .route("/search/set_directory", web::post().to(Self::set_root_search_directory)) 
        })
        .bind("127.0.0.1:8080")
        .expect("[SearchRouter] Failed to bind to address")
        .run()
        .await
        .expect("[SearchRouter] Failed to run HTTP server");
    }
}


