#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 23. 合并 K 个升序链表 (Merge k Sorted Lists)
// 难度：困难 | 分类：链表
// https://leetcode.cn/problems/merge-k-sorted-lists/
//
// 本地验证：cargo t p0023_merge_k_sorted_lists
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [[1,4,5],[1,3,4],[2,6]]
        // []
        // [[]]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::merge_k_lists(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0023_merge_k_sorted_lists
}
