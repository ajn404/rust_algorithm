// Leetcode #100: Same Tree
// https://leetcode.com/problems/same-tree/
// Time Complexity: O(n)
// Space Complexity: O(h)
// Rust implementation by Ajn404

use std::rc::Rc;
use std::cell::RefCell;

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
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
}

pub fn is_same_tree(p: Option<TreeNodeRef>, q: Option<TreeNodeRef>) -> bool {
    match (p, q) {
        (None, None) => true,
        (None, _) | (_, None) => false,
        (Some(p_node), Some(q_node)) => {
            let p_node = p_node.borrow();
            let q_node = q_node.borrow();
            p_node.val == q_node.val &&
            is_same_tree(p_node.left.clone(), q_node.left.clone()) &&
            is_same_tree(p_node.right.clone(), q_node.right.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_tree() {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert_eq!(is_same_tree(p, q), true);

        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
                     val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        })));
        assert_eq!(is_same_tree(p, q), false);
    }
}
