#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 19. 删除链表的倒数第 N 个结点 (Remove Nth Node From End of List)
// 难度：中等 | 分类：链表
// https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
//
// 本地验证：cargo t p0019_remove_nth_node_from_end_of_list
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
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
        // [1]
        // 1
        // [1,2]
        // 1
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::remove_nth_from_end(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0019_remove_nth_node_from_end_of_list
}
