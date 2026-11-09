#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 160. 相交链表 (Intersection of Two Linked Lists)
// 难度：简单 | 分类：链表
// https://leetcode.cn/problems/intersection-of-two-linked-lists/
//
// ⚠️ 力扣的 Rust 判题暂不支持本题，这里用 Rc<RefCell<...>> 做本地版本。
// 用 Rc::ptr_eq 判断两个指针是否指向同一个节点。
//
// 本地验证：cargo t p0160_intersection_of_two_linked_lists

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
    pub fn get_intersection_node(
        head_a: Option<Rc<RefCell<ListNode>>>,
        head_b: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 输入：intersectVal = 8, listA = [4,1,8,4,5], listB = [5,6,1,8,4,5]
        // skipA = 2, skipB = 3；输出：返回值为 8 的节点
        // let shared = ListNode::new(8);
        // let a1 = ListNode::new(4);
        // let a2 = ListNode::new(1);
        // a1.borrow_mut().next = Some(a2.clone());
        // a2.borrow_mut().next = Some(shared.clone());
        //
        // let b1 = ListNode::new(5);
        // let b2 = ListNode::new(6);
        // let b3 = ListNode::new(1);
        // b1.borrow_mut().next = Some(b2.clone());
        // b2.borrow_mut().next = Some(b3.clone());
        // b3.borrow_mut().next = Some(shared.clone());
        //
        // let node = Solution::get_intersection_node(Some(a1), Some(b1)).unwrap();
        // assert!(Rc::ptr_eq(&node, &shared));
    }
}

fn main() {}
