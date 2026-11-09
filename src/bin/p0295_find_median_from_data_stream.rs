#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 295. 数据流的中位数 (Find Median from Data Stream)
// 难度：困难 | 分类：堆
// https://leetcode.cn/problems/find-median-from-data-stream/
//
// 本地验证：cargo t p0295_find_median_from_data_stream
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

struct MedianFinder {}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        todo!()
    }

    fn add_num(&self, num: i32) {
        todo!()
    }

    fn find_median(&self) -> f64 {
        todo!()
    }
}

/*
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // ["MedianFinder","addNum","addNum","findMedian","addNum","findMedian"]
        // [[],[1],[2],[],[3],[]]
        //
        // 注意：官方模板方法接收 &self；需要修改字段时改成 &mut self。
        // let mut obj = MedianFinder::new(/* 参数 */);
    }
}

fn main() {
    // 调试入口：cargo r p0295_find_median_from_data_stream
}
