#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 155. 最小栈 (Min Stack)
// 难度：中等 | 分类：栈
// https://leetcode.cn/problems/min-stack/
//
// 本地验证：cargo t p0155_min_stack
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

struct MinStack {}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        todo!()
    }

    fn push(&self, value: i32) {
        todo!()
    }

    fn pop(&self) {
        todo!()
    }

    fn top(&self) -> i32 {
        todo!()
    }

    fn get_min(&self) -> i32 {
        todo!()
    }
}

/*
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(value);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // ["MinStack","push","push","push","getMin","pop","top","getMin"]
        // [[],[-2],[0],[-3],[],[],[],[]]
        //
        // 注意：官方模板方法接收 &self；需要修改字段时改成 &mut self。
        // let mut obj = MinStack::new(/* 参数 */);
    }
}

fn main() {
    // 调试入口：cargo r p0155_min_stack
}
