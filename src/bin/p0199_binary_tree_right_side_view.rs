#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 199. 二叉树的右视图 (Binary Tree Right Side View)
// 难度：中等 | 分类：二叉树
// https://leetcode.cn/problems/binary-tree-right-side-view/
//
// 本地验证：cargo t p0199_binary_tree_right_side_view
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,2,3,null,5,null,4]
        // [1,2,3,4,null,null,null,5]
        // [1,null,3]
        // []
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::right_side_view(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0199_binary_tree_right_side_view
}
