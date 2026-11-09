#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 104. 二叉树的最大深度 (Maximum Depth of Binary Tree)
// 难度：简单 | 分类：二叉树
// https://leetcode.cn/problems/maximum-depth-of-binary-tree/
//
// 本地验证：cargo t p0104_maximum_depth_of_binary_tree
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [3,9,20,null,null,15,7]
        // [1,null,2]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::max_depth(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0104_maximum_depth_of_binary_tree
}
