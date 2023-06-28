mod history;

pub(crate) use history::*;
use std::rc::Rc;
use crate::memory::ROUTERHISTORY;



/// Duid Core Router
pub struct Router;

impl Router {
    pub fn new() -> Router {
        Router
    }

    /// Router route.
    pub fn route(&self, url: &'static str) {
        ROUTERHISTORY.with(|history| {
            history.borrow_mut().visit(url);
        })
    }
    
    /// use to console log Duid Router.
    pub fn debug(&self) {
        ROUTERHISTORY.with(|history| {
            println!("ROUTER: {:#?}", history);
        })
    }
    
    /// returns the current route of the application.
    pub fn active_route(&self) -> Option<Rc<&'static str>> {
        ROUTERHISTORY.with(|history| {
            history.borrow().top()
        })
    }
    
    /// returns last visited route just before current route.
    pub fn back(&self) -> Option<Rc<&'static str>> {
        ROUTERHISTORY.with(|history| {
            history.borrow_mut().back()
        })
    }
    
    /// returns next route that we want to revisit.
    pub fn forward(&self) -> Option<Rc<&'static str>> {
        ROUTERHISTORY.with(|history| {
            history.borrow_mut().forward()
        })
    }
}

/*
#[cfg(test)]
mod router_test {
    use super::*;

    #[test]
    fn test_router_route() {

        Router::route("/djed");
        Router::debug();
    }

    #[test]
    fn test_router_active_route() {

        Router::route("/djed");
        let top = Router::active_route();
        Router::debug();

        println!("router top: {:#?}", top);
    }

    #[test]
    fn test_router_back() {
        Router::route("/djed");
        Router::route("/hotel");
        let back = Router::back();
        Router::debug();

        println!("router back: {:#?}", back);
    }

    #[test]
    fn test_router_forward() {
        Router::route("/djed");
        Router::route("/hotel");
        let back = Router::back();
        Router::debug();
        println!("router back: {:#?}", back);

        let forward = Router::forward();
        Router::debug();
        println!("router forward: {:#?}", forward);
    }
}*/