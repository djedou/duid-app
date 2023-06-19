/*
NB: The idea is to build TempArena at the moment the developer is calling tag_node and component_node functions

*/

use std::{cell::RefCell, rc::Rc};


#[derive(Default, Debug, Clone)]
pub struct NodeId {
    pub level: usize,
    pub level_index: usize,
    pub global_index: usize
}


#[derive(Default, Debug, Clone)]
pub struct ArenaNode//<MSG> 
//where 
//    MSG: Clone
{
    pub id: NodeId,
    //pub(crate) tag: &'static str,
    //pub(crate) node_type: VirtualNodeType,
    //pub(crate) namespace: Option<&'static str>,
    //pub(crate) props: Vec<Attribute<MSG>>,
    //pub(crate) value: Option<String>,
    //pub(crate) active_closures: Rc<RefCell<ActiveClosure>>,
    //pub(crate) node_state: ArenaNodeState
    //pub(crate) view
    //pub(crate) update
    //pub(crate) subscribe
}

#[derive(Default, Debug, Clone)]
pub struct Model/*<MDL: Clone>*/ {
    pub id: NodeId,
    //pub data: MDL
}

/// Temporary Arena
#[derive(Debug, Clone, Default)]
pub struct TempArena {
    pub data: Vec<ArenaNode>
}

/// Permanent Arena
#[derive(Debug, Clone, Default)]
pub struct PermArena {
    pub data: Option<Rc<[ArenaNode]>>
}

/// Temporary Models
#[derive(Debug, Clone, Default)]
pub struct TempModels {
    pub data: Vec<Model>
}


/// Permanent Models
#[derive(Debug, Clone, Default)]
pub struct PermModels {
    pub data: Option<Rc<[Model]>>
}



thread_local!{
    pub static TEMPMEMORY: Rc<RefCell<TempArena>> = Rc::new(RefCell::new(TempArena::default()));
    pub static PERMMEMORY: Rc<RefCell<PermArena>> = Rc::new(RefCell::new(PermArena::default()));
    pub static TEMPMODELS: Rc<RefCell<TempModels>> = Rc::new(RefCell::new(TempModels::default()));
    pub static PERMMODELS: Rc<RefCell<PermModels>> = Rc::new(RefCell::new(PermModels::default()));
}


pub fn get_temp_memory() -> Rc<RefCell<TempArena>> {
    TEMPMEMORY.with(|d| {
        Rc::clone(&d)
    })
}

pub fn set_temp_memory(data: &[ArenaNode]) {
    TEMPMEMORY.with(|d| {
        d.borrow_mut().data.extend_from_slice(&data);
    })
}

pub fn get_perm_memory() -> Rc<RefCell<PermArena>> {
    PERMMEMORY.with(|d| {
        Rc::clone(&d)
    })
}

pub fn set_perm_memory(data: &[ArenaNode]) {
    PERMMEMORY.with(|d| {
        *d.borrow_mut() = PermArena{data: Some(data.into())};
    })
}

#[cfg(test)]
mod test_memory {
    #[test] 
    fn test_default() {
        use crate::core::memory::{set_temp_memory, get_temp_memory, get_perm_memory, ArenaNode};

        set_temp_memory(&[ArenaNode::default(), ArenaNode::default()]);

        let temp = get_temp_memory();
        println!("djed temp_memory: {:#?}", temp);
        let perm = get_perm_memory();
        println!("perm_memory: {:#?}", perm);
    }
}