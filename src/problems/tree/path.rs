use std::{cell::RefCell, rc::Rc};

use super::binary_tree::TreeNode;

pub fn binary_tree_paths_rec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut paths = Vec::new();
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
        } else if left.is_some() && right.is_none() {
            btp_rec_sub(left, paths, path);
        } else if left.is_none() && right.is_some() {
            btp_rec_sub(right, paths, path);
        } else {
            let mut r_path = path.clone();
            btp_rec_sub(left, paths, path);
            btp_rec_sub(right, paths, &mut r_path);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::problems::tree::binary_tree::get_test_case;

    use super::*;

    #[test]
    fn test_binary_tree_paths() {
        let root = get_test_case();
        let result = binary_tree_paths_rec(root);
        println!("result\n{:?}", result);
    }
}
