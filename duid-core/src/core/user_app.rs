use crate::core::{
    html::nodes::Node,
    duid_events::{Cmd, Sub}
};
use std::collections::{BTreeMap};
use std::{cell::RefCell, rc::Rc};


#[derive(Clone)]
pub struct UserApp<MDL, MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static,
    MDL: std::fmt::Debug + Clone + 'static
{
    model: MDL,
    view: fn(&MDL) -> Node<MSG>,
    update: fn(&mut MDL, MSG) -> Cmd<MSG>,
    subscription: fn(&MDL) -> Sub<MSG>
}

pub type UserAppRegister<MDL, MSG> = Rc<RefCell<BTreeMap<String, UserApp<MDL, MSG>>>>;