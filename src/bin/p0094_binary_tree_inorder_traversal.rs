#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 94. 二叉树的中序遍历 (Binary Tree Inorder Traversal)
// 难度：简单 | 分类：二叉树
// https://leetcode.cn/problems/binary-tree-inorder-traversal/
//
// 本地验证：cargo t p0094_binary_tree_inorder_traversal
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,null,2,3]
        // [1,2,3,4,5,null,8,null,null,6,7,9]
        // []
        // [1]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::inorder_traversal(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0094_binary_tree_inorder_traversal
}
