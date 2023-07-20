use std::rc::Rc;
use std::cell::RefCell;
use crate::core::{
    memory::ActiveClosure,
    html::attributes::Attribute
};
use crate::arena::{ArenaNode, NodeId, ArenaNodeState};



#[derive(Debug, Clone, PartialEq)]
pub(crate) enum VirtualNodeType {
    Element,
    Fragment,
    Text,
    Comment,
    DocType,
}



#[derive(Debug, Clone)]
pub(crate) struct VirtualNode<MSG>
{
    pub(crate) key: usize,
    pub(crate) tag: &'static str,
    pub(crate) node_type: VirtualNodeType,
    pub(crate) namespace: Option<&'static str>,
    pub(crate) props: Vec<Attribute<MSG>>,
    pub(crate) value: Option<String>,
    pub(crate) children: Vec<VirtualNode<MSG>>
}

impl<MSG> VirtualNode<MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static 
{
    pub(crate) fn set_key(&mut self, key: usize) -> usize {
        let mut local_key = key;
        self.key = key;

        let mut children = self.children.iter_mut();
        
        while let Some(child) = children.next() {
            local_key = child.set_key(local_key + 1);
        }
        
        local_key
    }

    pub(crate) fn into_arena_node(&self) -> ArenaNode<MSG> {
        ArenaNode {
            id: NodeId::from_global_index(self.key),
            tag: self.tag.clone(),
            node_type: self.node_type.clone(),
            namespace: self.namespace.clone(),
            props: self.props.clone(),
            value: self.value.clone(),
            active_closures: Rc::new(RefCell::new(ActiveClosure::with_capacity(0))),
            node_state: ArenaNodeState::default()
        }
    }

    pub(crate) fn get_node_id(&self) -> NodeId {
        NodeId::from_global_index(self.key)
    }
}