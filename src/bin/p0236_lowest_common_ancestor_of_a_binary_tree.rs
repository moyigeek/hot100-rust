#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 236. 二叉树的最近公共祖先 (Lowest Common Ancestor of a Binary Tree)
// 难度：中等 | 分类：二叉树
// https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/
//
// 本地验证：cargo t p0236_lowest_common_ancestor_of_a_binary_tree
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [3,5,1,6,2,0,8,null,null,7,4]
        // 5
        // 1
        // [3,5,1,6,2,0,8,null,null,7,4]
        // 5
        // 4
        // [1,2]
        // 1
        // ...
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::lowest_common_ancestor(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0236_lowest_common_ancestor_of_a_binary_tree
}
