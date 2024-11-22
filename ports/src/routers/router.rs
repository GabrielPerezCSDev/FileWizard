use crate::routers::search_router::SearchRouter;

//this file will control spawning and controlling all routers 
pub trait Router {
    fn initialize(&self); // Called when the router starts
    fn start(&self);      // Logic to handle incoming requests
}

pub struct RouterRegistry {
    routers: Vec<Box<dyn Router>>,
}

impl RouterRegistry {
    pub fn new() -> Self {
        Self { routers: Vec::new() }
    }

    pub fn register(&mut self, router: Box<dyn Router>) {
        self.routers.push(router);
    }

    pub fn start(&self) {
        for router in &self.routers {
            router.start();
        }
    }
}