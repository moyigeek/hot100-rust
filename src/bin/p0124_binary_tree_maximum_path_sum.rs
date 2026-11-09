#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 124. 二叉树中的最大路径和 (Binary Tree Maximum Path Sum)
// 难度：困难 | 分类：二叉树
// https://leetcode.cn/problems/binary-tree-maximum-path-sum/
//
// 本地验证：cargo t p0124_binary_tree_maximum_path_sum
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,2,3]
        // [-10,9,20,null,null,15,7]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::max_path_sum(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0124_binary_tree_maximum_path_sum
}
