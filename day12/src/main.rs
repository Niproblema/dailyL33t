use std::cell::RefCell;
use std::rc::Rc;

fn main() {
        // //let left_child = Rc::new(RefCell::new(TreeNode::new(2)));
        // let right_child = Rc::new(RefCell::new(TreeNode::new(2)));
        // let root = Rc::new(RefCell::new(TreeNode {
        //     val: 1,
        //     left: None,
        //     right: Some(right_child.clone()),
        // }));
    
        // let res = Solution::average_of_subtree(Some(root));
    
        println!("{:?}", res);
}

// Definition for a binary tree node.
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
}

// https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/description/?envType=daily-question&envId=2023-11-02
pub struct Solution {}
impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nice_subtree_count = 0;

        Solution::get_subtree_sum(root.as_ref(), &mut nice_subtree_count);

        nice_subtree_count
    }

    fn get_subtree_sum(root: Option<&Rc<RefCell<TreeNode>>>, count: &mut i32) -> (i32, i32) {
        if let Some(node) = root {
            let unwrapped_node = node.borrow();
            let left_sub = Solution::get_subtree_sum(unwrapped_node.left.as_ref(), count);
            let right_sub = Solution::get_subtree_sum(unwrapped_node.right.as_ref(), count);
            let node_val = unwrapped_node.val;
            let subtree_sum = left_sub.0 + right_sub.0 + node_val;
            let subtree_size = left_sub.1 + right_sub.1 + 1;

            if subtree_sum / subtree_size == node_val {
                *count += 1;
            }

            return (subtree_sum, subtree_size);
        }

        (0, 0)
    }
}
