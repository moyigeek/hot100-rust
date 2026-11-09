#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 101. 对称二叉树 (Symmetric Tree)
// 难度：简单 | 分类：二叉树
// https://leetcode.cn/problems/symmetric-tree/
//
// 本地验证：cargo t p0101_symmetric_tree
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,2,2,3,4,4,3]
        // [1,2,2,null,3,null,3]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::is_symmetric(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0101_symmetric_tree
}
