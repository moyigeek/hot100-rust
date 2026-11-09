#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 98. 验证二叉搜索树 (Validate Binary Search Tree)
// 难度：中等 | 分类：二叉树
// https://leetcode.cn/problems/validate-binary-search-tree/
//
// 本地验证：cargo t p0098_validate_binary_search_tree
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [2,1,3]
        // [5,1,4,null,null,3,6]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::is_valid_bst(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0098_validate_binary_search_tree
}
