fn main() {
    //let left_child = Rc::new(RefCell::new(TreeNode::new(2)));
    let right_child = Rc::new(RefCell::new(TreeNode::new(2)));
    let root = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(right_child.clone()),
    }));

    let res = Solution::find_mode(Some(root));

    println!("{:?}", res);
}

use std::cell::RefCell;
use std::rc::Rc;

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

// https://leetcode.com/problems/find-mode-in-binary-search-tree/?envType=daily-question&envId=2023-11-01
pub struct Solution {}
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut counter = TreeModeCounter::new();

        Solution::find_sub_mode(root.as_ref(), &mut counter);

        counter.complete();
        counter.mode_values
    }

    fn find_sub_mode(root: Option<&Rc<RefCell<TreeNode>>>, counter: &mut TreeModeCounter) {
        if let Some(node) = root {
            let unwrapped_node: std::cell::Ref<'_, TreeNode> = node.borrow();
            Solution::find_sub_mode(unwrapped_node.left.as_ref(), counter);
            counter.add_value(unwrapped_node.val);
            Solution::find_sub_mode(unwrapped_node.right.as_ref(), counter);
        }
    }
}

pub struct TreeModeCounter {
    pub mode_values: Vec<i32>,
    mode_count: i32,
    live_value: i32,
    live_count: i32,
}

impl TreeModeCounter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mode_values: vec![],
            mode_count: 0,
            live_value: 0,
            live_count: 0,
        }
    }

    pub fn add_value(&mut self, value: i32) {
        if value != self.live_value {
            if self.live_count > self.mode_count {
                self.mode_values.clear();
                self.mode_values.push(self.live_value);
                self.mode_count = self.live_count;
            } else if self.live_count == self.mode_count && self.live_count != 0 {
                self.mode_values.push(self.live_value);
            }
            self.live_value = value;
            self.live_count = 1;
            return;
        }

        self.live_count += 1;
    }

    pub fn complete(&mut self) {
        if self.live_count > self.mode_count {
            self.mode_values.clear();
            self.mode_values.push(self.live_value);
        } else if self.live_count == self.mode_count && self.live_count != 0 {
            self.mode_values.push(self.live_value);
        }
    }
}
