#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 25. K 个一组翻转链表 (Reverse Nodes in k-Group)
// 难度：困难 | 分类：链表
// https://leetcode.cn/problems/reverse-nodes-in-k-group/
//
// 本地验证：cargo t p0025_reverse_nodes_in_k_group
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,2,3,4,5]
        // 2
        // [1,2,3,4,5]
        // 3
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::reverse_k_group(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0025_reverse_nodes_in_k_group
}
