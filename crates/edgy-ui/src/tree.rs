use slotmap::SlotMap;

use crate::{Node, NodeId};

pub fn children_count(tree: &SlotMap<NodeId, Node>, parent: NodeId) -> usize {
    let mut count = 0;
    let mut child = tree.get(parent).and_then(|n| n.first_child);
    while let Some(key) = child {
        count += 1;
        child = tree.get(key).and_then(|n| n.next_sibling);
    }
    count
}

pub fn children(tree: &SlotMap<NodeId, Node>, parent: NodeId) -> alloc::vec::Vec<NodeId> {
    let mut result = alloc::vec::Vec::with_capacity(children_count(tree, parent));
    let mut child = tree.get(parent).and_then(|n| n.first_child);
    while let Some(key) = child {
        result.push(key);
        child = tree.get(key).and_then(|n| n.next_sibling);
    }
    result
}

pub fn attach_child(tree: &mut SlotMap<NodeId, Node>, parent_id: NodeId, child_id: NodeId) {
    {
        let child = tree.get_mut(child_id).expect("child not in tree");
        child.parent = Some(parent_id);
    }

    let parent = tree.get_mut(parent_id).expect("parent not in tree");
    match parent.first_child {
        None => {
            parent.first_child = Some(child_id);
        }
        Some(first) => {
            let mut sibling = first;
            loop {
                let next = tree[sibling].next_sibling;
                match next {
                    None => {
                        tree.get_mut(sibling).unwrap().next_sibling = Some(child_id);
                        break;
                    }
                    Some(n) => sibling = n,
                }
            }
        }
    }
}

pub fn detach(tree: &mut SlotMap<NodeId, Node>, id: NodeId) {
    let parent_key = match tree.get(id).and_then(|n| n.parent) {
        Some(p) => p,
        None => return,
    };

    let prev_sibling = {
        let parent = tree.get(parent_key).unwrap();
        let first = parent.first_child.unwrap_or(id);
        if first == id {
            None
        } else {
            let mut current = first;
            loop {
                let next = tree[current].next_sibling;
                match next {
                    Some(n) if n == id => break Some(current),
                    Some(n) => current = n,
                    None => break None,
                }
            }
        }
    };

    let next = tree[id].next_sibling;

    match prev_sibling {
        Some(prev) => {
            tree.get_mut(prev).unwrap().next_sibling = next;
        }
        None => {
            tree.get_mut(parent_key).unwrap().first_child = next;
        }
    }

    tree.get_mut(id).unwrap().parent = None;
    tree.get_mut(id).unwrap().next_sibling = None;
}
