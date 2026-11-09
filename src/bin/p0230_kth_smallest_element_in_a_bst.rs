#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 230. 二叉搜索树中第 K 小的元素 (Kth Smallest Element in a BST)
// 难度：中等 | 分类：二叉树
// https://leetcode.cn/problems/kth-smallest-element-in-a-bst/
//
// 本地验证：cargo t p0230_kth_smallest_element_in_a_bst
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [3,1,4,null,2]
        // 1
        // [5,3,6,2,4,null,null,1]
        // 3
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::kth_smallest(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0230_kth_smallest_element_in_a_bst
}
