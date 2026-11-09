#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 138. 随机链表的复制 (Copy List with Random Pointer)
// 难度：中等 | 分类：链表
// https://leetcode.cn/problems/copy-list-with-random-pointer/
//
// ⚠️ 力扣的 Rust 判题暂不支持本题，这里用 Rc<RefCell<...>> 做本地版本。
// random 指向链表中的任意节点（或空），复制时要求深拷贝：新链表节点与原链表互不共享。
//
// 本地验证：cargo t p0138_copy_list_with_random_pointer

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

#[derive(Debug)]
pub struct RandomNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<RandomNode>>>,
    pub random: Option<Rc<RefCell<RandomNode>>>,
}

impl RandomNode {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            next: None,
            random: None,
        }))
    }
}

impl Solution {
    pub fn copy_random_list(
        head: Option<Rc<RefCell<RandomNode>>>,
    ) -> Option<Rc<RefCell<RandomNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 输入：head = [[7, null], [13, 0], [11, 4], [10, 2], [1, 0]]
        // 每项为 [val, random 下标]；输出：深拷贝后的链表
        // let n0 = RandomNode::new(7);
        // let n1 = RandomNode::new(13);
        // n0.borrow_mut().next = Some(n1.clone());
        // n1.borrow_mut().random = Some(n0.clone());
        //
        // let copy = Solution::copy_random_list(Some(n0)).unwrap();
        // assert!(!Rc::ptr_eq(&copy, &n1));
        // assert_eq!(copy.borrow().val, 7);
    }
}

fn main() {}
