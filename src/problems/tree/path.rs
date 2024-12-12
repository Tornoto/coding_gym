use std::{cell::RefCell, rc::Rc};

use super::binary_tree::TreeNode;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut paths: Vec<String> = vec![];
    let mut stack: Vec<(Rc<RefCell<TreeNode>>, Vec<i32>)> = vec![];
    if let Some(node) = root {
        stack.push((node, vec![]));
    }

    while let Some((node, mut path)) = stack.pop() {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        path.push(node.borrow().val);

        if left.is_none() && right.is_none() {
            paths.push(
                path.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("->"),
            );
        }

        if let Some(r) = right {
            stack.push((r, path.clone()));
        }

        if let Some(l) = left {
            stack.push((l, path));
        }
    }

    paths
}

pub fn binary_tree_paths_rec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    btp_rec_sub(root, &mut paths, &mut vec![]);
    paths
}

pub fn btp_rec_sub(
    root: Option<Rc<RefCell<TreeNode>>>,
    paths: &mut Vec<String>,
    path: &mut Vec<i32>,
) {
    if let Some(node) = root {
        path.push(node.borrow().val);
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        if left.is_none() && right.is_none() {
            return paths.push(
                path.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("->"),
            );
        }

        if let Some(l) = left {
            btp_rec_sub(Some(l), paths, &mut path.clone());
        }

        if let Some(r) = right {
            btp_rec_sub(Some(r), paths, path);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::problems::tree::binary_tree::{get_test_case, get_test_case2};

    use super::*;

    #[test]
    fn test_binary_tree_paths() {
        let root = get_test_case();
        let result1 = binary_tree_paths_rec(root.clone());
        let result2 = binary_tree_paths(root.clone());
        println!("result1\n{:?}", result1);
        println!("result1\n{:?}", result2);
        let root = get_test_case2();
        let result1 = binary_tree_paths_rec(root.clone());
        let result2 = binary_tree_paths(root.clone());
        println!("result1\n{:?}", result1);
        println!("result1\n{:?}", result2);
    }
}
