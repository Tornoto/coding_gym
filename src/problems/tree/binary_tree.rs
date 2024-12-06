use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn set(
        &mut self,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) {
        self.left = left;
        self.right = right;
    }
}

pub fn get_test_case() -> Option<Rc<RefCell<TreeNode>>> {
    let tn1: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(1)));
    let tn2: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(2)));
    let tn3: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(3)));
    let tn4: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(4)));
    let tn5: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(5)));
    let tn6: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(6)));
    let tn7: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(7)));
    let tn8: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(8)));
    let tn9: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(9)));
    tn1.borrow_mut().set(Some(tn2.clone()), Some(tn3.clone()));
    tn2.borrow_mut().set(Some(tn4), Some(tn5.clone()));
    tn3.borrow_mut().set(None, Some(tn8.clone()));
    tn5.borrow_mut().set(Some(tn6), Some(tn7));
    tn8.borrow_mut().set(Some(tn9), None);
    Some(tn1)
}
