
use std::{cell::RefCell, rc::Rc};
use crate::core::router::RouterHistory;


thread_local!{
    pub(crate) static ROUTERHISTORY: Rc<RefCell<RouterHistory>> = Rc::new(RefCell::new(RouterHistory::new()));
}
