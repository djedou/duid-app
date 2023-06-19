use std::{cell::RefCell, rc::Rc};



#[derive(Debug, Clone, PartialEq)]
pub struct Element<MSG>
{
    pub id: Rc<str>,
    pub tag: Rc<str>,
    pub namespace: Option<Rc<str>>,
    //pub props: Vec<Attribute<MSG>>,
    pub children: Rc<RefCell<[Tag]>>
}