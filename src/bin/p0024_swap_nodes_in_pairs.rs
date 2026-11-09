#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 24. 两两交换链表中的节点 (Swap Nodes in Pairs)
// 难度：中等 | 分类：链表
// https://leetcode.cn/problems/swap-nodes-in-pairs/
//
// 本地验证：cargo t p0024_swap_nodes_in_pairs
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,2,3,4]
        // []
        // [1]
        // [1,2,3]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::swap_pairs(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0024_swap_nodes_in_pairs
}
