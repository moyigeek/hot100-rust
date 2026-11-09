//! 常用类型的预导入模块。
//!
//! 题目文件里的 `use leetcode::prelude::*;` 会一次性引入：
//! - 链表 / 二叉树节点及构造辅助函数
//! - `HashMap`、`HashSet`、`BinaryHeap`、`VecDeque` 等刷题常用容器
//! - `Rc`、`RefCell`、`Cell`（二叉树题目会用到）

pub use std::cell::{Cell, RefCell};
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
pub use std::rc::Rc;

/// 单链表节点，字段与力扣一致。
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

/// 由切片构造单链表，等价于 `list![...]` 的非宏版本。
pub fn to_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    for &value in values {
        *tail = Some(Box::new(ListNode::new(value)));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

/// 把单链表转成 `Vec`，方便写断言。
pub fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut values = Vec::new();
    while let Some(node) = head {
        values.push(node.val);
        head = node.next;
    }
    values
}

/// 二叉树节点，字段与力扣一致。
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/// 由层序数组构造二叉树（`None` 表示空节点），等价于 `tree![...]` 的非宏版本。
pub fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let first = (*values.first()?)?;
    let root = Rc::new(RefCell::new(TreeNode::new(first)));
    let mut queue = VecDeque::from([root.clone()]);
    let mut index = 1;

    while index < values.len() {
        let Some(node) = queue.pop_front() else {
            break;
        };

        if let Some(&Some(value)) = values.get(index) {
            let child = Rc::new(RefCell::new(TreeNode::new(value)));
            node.borrow_mut().left = Some(child.clone());
            queue.push_back(child);
        }
        index += 1;

        if index >= values.len() {
            break;
        }
        if let Some(&Some(value)) = values.get(index) {
            let child = Rc::new(RefCell::new(TreeNode::new(value)));
            node.borrow_mut().right = Some(child.clone());
            queue.push_back(child);
        }
        index += 1;
    }

    Some(root)
}

/// 把二叉树按层序转成数组，末尾的 `None` 会被去掉。
pub fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let Some(root) = root else {
        return Vec::new();
    };

    let mut values = Vec::new();
    let mut queue = VecDeque::from([Some(root)]);

    while let Some(item) = queue.pop_front() {
        match item {
            Some(node) => {
                let node = node.borrow();
                values.push(Some(node.val));
                queue.push_back(node.left.clone());
                queue.push_back(node.right.clone());
            }
            None => values.push(None),
        }
    }

    while values.last() == Some(&None) {
        values.pop();
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_macro_round_trip() {
        assert_eq!(list_to_vec(crate::list![1, 2, 3,]), vec![1, 2, 3]);
        assert_eq!(list_to_vec(to_list(&[4, 5])), vec![4, 5]);
    }

    #[test]
    fn tree_macro_round_trip() {
        let root = crate::tree![3, 9, 20, null, null, 15, 7];
        let expected = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(tree_to_vec(root), expected);
    }

    #[test]
    fn tree_negative_and_empty() {
        let root = crate::tree![-10, 9, 20, null, null, 15, 7];
        assert_eq!(
            tree_to_vec(root),
            vec![Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)]
        );
        assert_eq!(tree_to_vec(crate::tree![]), Vec::<Option<i32>>::new());
    }
}
