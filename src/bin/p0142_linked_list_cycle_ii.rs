#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 142. 环形链表 II (Linked List Cycle II)
// 难度：中等 | 分类：链表
// https://leetcode.cn/problems/linked-list-cycle-ii/
//
// ⚠️ 力扣的 Rust 判题暂不支持本题，这里用 Rc<RefCell<...>> 做本地版本。
//
// 本地验证：cargo t p0142_linked_list_cycle_ii

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
    pub fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 输入：head = [3, 2, 0, -4]，pos = 1；输出：返回下标 1 的节点
        // let a = ListNode::new(3);
        // let b = ListNode::new(2);
        // let c = ListNode::new(0);
        // let d = ListNode::new(-4);
        // a.borrow_mut().next = Some(b.clone());
        // b.borrow_mut().next = Some(c.clone());
        // c.borrow_mut().next = Some(d.clone());
        // d.borrow_mut().next = Some(b.clone());
        // let entry = Solution::detect_cycle(Some(a)).unwrap();
        // assert!(Rc::ptr_eq(&entry, &b));
    }
}

fn main() {}
