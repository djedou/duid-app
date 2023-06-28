mod stack;


pub(crate) use stack::*;
use std::rc::Rc;



#[derive(Debug)]
pub(crate) struct RouterHistory {
    pub(crate) history: Stack<Rc<&'static str>>,
    pub(crate) future: Stack<Rc<&'static str>>
}


impl RouterHistory {

    pub(crate) fn new() -> RouterHistory {
        let mut history = Stack::<Rc<&'static str>>::new();
        history.push(Rc::new("/".into()));

        RouterHistory {
            history,
            future: Stack::<Rc<&str>>::new()
        }
    }

    pub(crate) fn visit(&mut self, path: &'static str) {
        self.history.push(Rc::new(path.into()));
        self.future = Stack::<Rc<&str>>::new();
    }

    pub(crate) fn top(&self) -> Option<Rc<&'static str>> {
        self.history.top().cloned()
    }

    pub(crate) fn back(&mut self) -> Option<Rc<&'static str>> {
        match self.history.top() {
            Some(h) => {
                self.future.push(Rc::clone(h));
                let _ = self.history.pop();
                self.history.top().cloned()
            },
            None => None
        }
    }

    pub(crate) fn forward(&mut self) -> Option<Rc<&'static str>> {
        match self.future.top() {
            Some(h) => {
                self.history.push(Rc::clone(h));
                let _ = self.future.pop();
                self.history.top().cloned()
            },
            None => None
        }
    }
}

#[cfg(test)]
mod browser_history_test {
    use super::*;

    #[test]
    fn test_history_new() {

        let _history = RouterHistory::new();
        //println!("history: {:#?}", history);
    }

    #[test]
    fn test_history_visit() {

        let mut browser_history = RouterHistory::new();
        browser_history.visit("/buy");

        //println!("browser_history: {:#?}", browser_history);
    }

    #[test]
    fn test_history_back() {
        let mut browser_history = RouterHistory::new();
        browser_history.visit("/buy");
        //println!("browser_history: {:#?}", browser_history);
        let _back = browser_history.back();
        //println!("back_browser_history: {:#?}", back);
        //println!("browser_history: {:#?}", browser_history);
    }

    #[test]
    fn test_history_forward() {
        let mut browser_history = RouterHistory::new();
        browser_history.visit("/buy");
        browser_history.visit("/flight");
        browser_history.visit("/hotel");
        //println!("browser_history: {:#?}", browser_history);
        let _back1 = browser_history.back();
        //println!("back_browser_history1: {:#?}", back1);
        let _back2 = browser_history.back();
        //println!("back_browser_history2: {:#?}", back2);
        //println!("browser_history: {:#?}", browser_history);
        browser_history.visit("/visa");
        //println!("browser_history: {:#?}", browser_history);
        let _forward = browser_history.forward();
        //println!("forward_browser_history: {:#?}", forward);
        //println!("browser_history: {:#?}", browser_history);
    }
}