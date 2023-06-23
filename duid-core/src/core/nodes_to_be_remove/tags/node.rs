use std::{cell::RefCell, rc::Rc};



#[derive(Debug, Clone, PartialEq)]
pub enum Node<MSG> {
    Element(Element<MSG>),
    Fragment(Vec<Node<MSG>>),
    Text(Leaf<MSG>),
    Comment(Leaf<MSG>),
    DocType(Leaf<MSG>),
}