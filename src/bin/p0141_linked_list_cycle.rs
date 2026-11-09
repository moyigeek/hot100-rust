#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 141. 环形链表 (Linked List Cycle)
// 难度：简单 | 分类：链表
// https://leetcode.cn/problems/linked-list-cycle/
//
// ⚠️ 力扣的 Rust 判题暂不支持本题，这里用 Rc<RefCell<...>> 做本地版本。
// Box 链表无法形成环，Rc 允许节点被共享，因此可以构造环来练习快慢指针。
//
// 本地验证：cargo t p0141_linked_list_cycle

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { val, next: None }))
    }
}

impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 输入：head = [3, 2, 0, -4]，pos = 1；输出：true
        // let a = ListNode::new(3);
        // let b = ListNode::new(2);
        // a.borrow_mut().next = Some(b.clone());
        // b.borrow_mut().next = Some(a.clone()); // 尾节点指回下标 1，形成环
        // assert!(Solution::has_cycle(Some(a)));
        //
        // 无环：head = [1]，pos = -1；输出：false
        // assert!(!Solution::has_cycle(Some(ListNode::new(1))));
    }
}

fn main() {}
