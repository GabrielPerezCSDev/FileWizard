
use async_trait::async_trait;
use futures::future::join_all;

#[async_trait]
pub trait Router: Send + Sync {
    async fn initialize(&self); // Called when the router starts
    async fn start(&self);      // Logic to handle incoming requests
}

pub struct RouterRegistry {
    routers: Vec<Box<dyn Router + Send + Sync>>,
}

impl RouterRegistry {
    pub fn new() -> Self {
        println!("[RouterRegistry] created");
        Self { routers: Vec::new() }
    }

    pub fn register(&mut self, router: Box<dyn Router + Send + Sync>) {
        println!("[RouterRegistry] registered a router");
        self.routers.push(router);
    }

    pub async fn start(&self) {
        println!("[RouterRegistry] starting the routers");
        // Initialize all routers concurrently
        let init_futures = self.routers.iter().map(|router| router.initialize());
        join_all(init_futures).await;

        // Start all routers concurrently
        let start_futures = self.routers.iter().map(|router| router.start());
        join_all(start_futures).await;
    }
}