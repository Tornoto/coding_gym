use super::GraphNode;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

/// https://leetcode.com/problems/clone-graph/description/
/// suppose value of node is unique
pub fn clone_graph(node: Option<Rc<RefCell<GraphNode>>>) -> Option<Rc<RefCell<GraphNode>>> {
    if node.is_none() {
        return None;
    }

    let mut queue = VecDeque::new();
    let mut visited: HashMap<i32, Rc<RefCell<GraphNode>>> = HashMap::new();
    let root = node.unwrap();
    let cloned_root = Rc::new(RefCell::new(GraphNode::new(root.borrow().val)));

    // 将 root 入队，并维护映射关系
    queue.push_back(root.clone());
    visited.insert(root.borrow().val, cloned_root.clone());

    while let Some(cur_node) = queue.pop_front() {
        // 获取当前节点的克隆节点
        let cloned_cur = visited.get(&cur_node.borrow().val).unwrap().clone();
        // 维护邻节点
        for neighbor in &cur_node.borrow().neighbors {
            // 如果邻节点没有克隆，创建克隆并加入队列
            if !visited.contains_key(&neighbor.borrow().val) {
                let cloned_neighbor = Rc::new(RefCell::new(GraphNode::new(neighbor.borrow().val)));
                visited.insert(neighbor.borrow().val, cloned_neighbor.clone());
                queue.push_back(neighbor.clone());
            }
            // 将克隆的邻节点天假到当前节点的邻节点列表中
            cloned_cur
                .borrow_mut()
                .neighbors
                .push(visited[&neighbor.borrow().val].clone());
        }
    }

    Some(cloned_root)
}

#[cfg(test)]
mod test {
    use crate::problems::graph::graph_node::display;

    use super::*;

    #[test]
    fn test() {
        // Create nodes
        let node1 = Rc::new(RefCell::new(GraphNode::new(1)));
        let node2 = Rc::new(RefCell::new(GraphNode::new(2)));
        let node3 = Rc::new(RefCell::new(GraphNode::new(3)));
        let node4 = Rc::new(RefCell::new(GraphNode::new(4)));

        // Build the graph
        node1.borrow_mut().neighbors = vec![node2.clone(), node4.clone()];
        node2.borrow_mut().neighbors = vec![node1.clone(), node3.clone()];
        node3.borrow_mut().neighbors = vec![node2.clone(), node4.clone()];
        node4.borrow_mut().neighbors = vec![node1.clone(), node3.clone()];

        // Clone the graph
        let cloned_graph = clone_graph(Some(node1.clone()));
        display(Some(node1));
        display(cloned_graph);
    }
}
