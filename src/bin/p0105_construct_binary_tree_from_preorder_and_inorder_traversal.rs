#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 105. 从前序与中序遍历序列构造二叉树 (Construct Binary Tree from Preorder and Inorder Traversal)
// 难度：中等 | 分类：二叉树
// https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
//
// 本地验证：cargo t p0105_construct_binary_tree_from_preorder_and_inorder_traversal
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [3,9,20,15,7]
        // [9,3,15,20,7]
        // [-1]
        // [-1]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::build_tree(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0105_construct_binary_tree_from_preorder_and_inorder_traversal
}
