#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 11. 盛最多水的容器 (Container With Most Water)
// 难度：中等 | 分类：双指针
// https://leetcode.cn/problems/container-with-most-water/
//
// 本地验证：cargo t p0011_container_with_most_water
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,8,6,2,5,4,8,3,7]
        // [1,1]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::max_area(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0011_container_with_most_water
}
