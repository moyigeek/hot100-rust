//! LeetCode 热题 100 的本地练习库。
//!
//! 这里提供力扣题目里常用的链表 / 二叉树节点定义，以及构造、打印用的辅助函数。
//! 每个题目位于 `src/bin/` 下，互相独立，可以单独编译和测试。

pub mod prelude;

/// 用一组整数快速构造单链表：`list![1, 2, 3]`。
#[macro_export]
macro_rules! list {
    () => { None };
    ($($value:expr),+ $(,)?) => {{
        let mut head: Option<Box<$crate::prelude::ListNode>> = None;
        let mut tail = &mut head;
        $(
            *tail = Some(Box::new($crate::prelude::ListNode::new($value)));
            tail = &mut tail.as_mut().unwrap().next;
        )+
        head
    }};
}

/// 用层序遍历快速构造二叉树，`null` 表示空节点：
/// `tree![3, 9, 20, null, null, 15, 7]`。
#[macro_export]
macro_rules! tree {
    () => { None };
    ($($token:tt)*) => {{
        $crate::prelude::build_tree(&$crate::tree_values![$($token)*])
    }};
}

/// `tree!` 的内部实现：把 `3, null, -1, ...` 转成 `Vec<Option<i32>>`。
#[doc(hidden)]
#[macro_export]
macro_rules! tree_values {
    () => { vec![] };
    (null $(,)?) => { vec![None] };
    (- $value:literal $(,)?) => { vec![Some(-$value)] };
    ($value:literal $(,)?) => { vec![Some($value)] };
    (null, $($rest:tt)*) => {{
        let mut values = vec![None];
        values.extend($crate::tree_values![$($rest)*]);
        values
    }};
    (- $value:literal, $($rest:tt)*) => {{
        let mut values = vec![Some(-$value)];
        values.extend($crate::tree_values![$($rest)*]);
        values
    }};
    ($value:literal, $($rest:tt)*) => {{
        let mut values = vec![Some($value)];
        values.extend($crate::tree_values![$($rest)*]);
        values
    }};
}
