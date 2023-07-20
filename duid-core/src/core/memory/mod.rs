
use std::{cell::RefCell, rc::Rc};
use crate::core::router::RouterHistory;
use std::cell::Cell;
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;


thread_local!{
    pub(crate) static ROUTERHISTORY: Rc<RefCell<RouterHistory>> = Rc::new(RefCell::new(RouterHistory::new()));
    pub(crate) static NODE_ID_COUNTER: Cell<usize> = Cell::new(1);
}

pub(crate) fn create_unique_identifier() -> usize {
    let id = NODE_ID_COUNTER.with(|x| {
        let tmp = x.get();
        x.set(tmp + 1);
        tmp
    });
    id
}

pub(crate) type ActiveClosure = HashMap<usize, Vec<(&'static str, Closure<dyn FnMut(web_sys::Event)>)>>;
pub(crate) const DATA_VDOM_ID: &str = "dom-event-id";