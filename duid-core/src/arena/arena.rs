use web_sys::{Node, Document};
use crate::arena::{ArenaNodeState, Pairs};
use crate::{
    core::{
        v_node::VirtualNode,
        duid_events::Dispatch
    },
    dom::HtmlNodeBuilder
};
use std::collections::{HashMap, HashSet};
use super::{NodeId, ArenaNode};

#[derive(Debug, Clone)]
pub struct Arena<T> {
    pub(crate) nodes: Vec<T>,
    pub(crate) first_node_id: NodeId,
    pub(crate) last_node_id: NodeId,
    pub(crate) nodes_ids: HashSet<Pairs>
    
}

impl<MSG> Arena<ArenaNode<MSG>> 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    pub(crate) fn new() -> Arena<ArenaNode<MSG>> {
        Arena {
            nodes: Vec::with_capacity(0),
            first_node_id: NodeId::default(),
            last_node_id: NodeId::default(),
            nodes_ids: HashSet::with_capacity(0)
        }
    }

    pub(crate) fn remove(&mut self, node_id: &NodeId) {
        self.nodes.retain(|node| node.id != *node_id);
        self.nodes_ids.retain(|node| node.child != *node_id);
    }

    pub(crate) fn add(&mut self, nodes_ids: HashSet<Pairs>, nodes: &[ArenaNode<MSG>]) {
        self.nodes.extend_from_slice(&nodes);
        self.nodes_ids.extend(nodes_ids);
    }

    pub(crate) fn get_first_id(&self) -> NodeId {
        self.first_node_id.clone()
    }

    pub(crate) fn get_last_id(&self) -> NodeId {
        self.last_node_id.clone()
    }

    pub(crate) fn new_from_virtual_node(virtual_node: &VirtualNode<MSG>) -> Arena<ArenaNode<MSG>> {
        let mut arena = Arena::new();
        let index_pairs_vec = Arena::to_indexes_pair(&virtual_node);
        let mut nodes_ids = HashSet::with_capacity(0);
        let mut first_node_id = virtual_node.get_node_id();
        first_node_id.level = 1;
        first_node_id.level_index = 0;
        let root = Pairs::new(NodeId::default(), first_node_id.clone());

        NodeId::set_node_id(2, &[root], &mut nodes_ids, &index_pairs_vec,);
        Arena::to_nodes(&virtual_node, &mut arena, &nodes_ids);
        
        arena.last_node_id = Arena::<ArenaNode<MSG>>::last_id(&nodes_ids);
        arena.nodes_ids = nodes_ids;
        arena.first_node_id = first_node_id.clone();

        arena
    }

    fn last_id(nodes: &HashSet<Pairs>) -> NodeId {
        let levels: Vec<_> = nodes.iter().map(|pairs| pairs.child.level.clone()).collect();
        let Some(max_level) = levels.iter().max() else {
            return NodeId::default();
        };
        let mut last_nodes: Vec<_> = nodes.iter()
            .filter(|pairs| pairs.child.level == *max_level)
            .map(|pairs| pairs.child.clone())
            .collect();
        last_nodes.sort_by(|a, b| a.global_index.cmp(&b.global_index));

        match last_nodes.last() {
            Some(node) => node.clone(),
            None => NodeId::default()
        }
        
    }

    fn to_nodes(node: &VirtualNode<MSG>, arena: &mut Arena<ArenaNode<MSG>>, nodes_ids: &HashSet<Pairs>) {
        let mut local_node = node.into_arena_node();
        if let Some(ids) = nodes_ids.iter().find(|id| id.child.global_index == local_node.id.global_index) {
            local_node.id.level = ids.child.level;
            local_node.id.level_index = ids.child.level_index;
        }
        arena.nodes.push(local_node);

        let mut children = node.children.iter();
        
        while let Some(child) = children.next() {
            Arena::to_nodes(child, arena, &nodes_ids);
        }
    }

    fn to_indexes_pair(node: &VirtualNode<MSG>) -> Vec<[NodeId; 2]> {
        let result: Vec<_> = node.children.iter().map(|child| {
            let mut indexes: Vec<[NodeId; 2]> = vec![[node.get_node_id(), child.get_node_id()]];
            indexes.extend_from_slice(&Arena::to_indexes_pair(child));
            indexes
        })
        .collect();

        let res = result.into_iter()
            .fold(vec![], |mut old_vec: Vec<[NodeId; 2]>, new_vec: Vec<[NodeId; 2]>| {
                old_vec.extend_from_slice(&new_vec);
                old_vec
            });

        res
    }

    pub(crate) fn build_html_node<DSP>(
        &mut self, 
        node_id: NodeId,
        program: &DSP,
        doc: &Document, 
        styles_map: &mut HashMap<String, String>, 
        selectors_set: &mut HashMap<usize, HashSet<String>>
    ) -> Node
    where
        DSP: Dispatch<MSG> + Clone + 'static
    {
        // get ArenaNode by nodeId
        match node_id.get_node_by_id_mut(self) {
            Some(node) => {
                let html_node = HtmlNodeBuilder::build(
                    program, 
                    &doc, 
                    node,
                    styles_map,
                    selectors_set
                );
                // get children
                let children_ids = node_id.get_children(&self.nodes_ids);
                let children_nodes: Vec<_> = children_ids.iter().map(|child| self.build_html_node(
                    child.clone(),
                    program,
                    &doc, 
                    styles_map,
                    selectors_set
                ))
                .collect();

                children_nodes.iter().for_each(|child| {
                    let _ = html_node.append_child(&child);
                });
                html_node
            },
            None => todo!()
        }
    }

    pub(crate) fn clean_patches(&mut self) {
        self.nodes.iter_mut().for_each(|node| {
            node.node_state = ArenaNodeState::default();
        });
    }
}