use std::{cell::RefCell, rc::Rc};


#[derive(Debug, Clone, PartialEq)]
pub struct Leaf<MSG> {
    pub tag: Rc<str>,
    pub namespace: Option<Rc<str>>,
    //pub props: Vec<Attribute<MSG>>,
    pub value: Rc<RefCell<str>>,
}

