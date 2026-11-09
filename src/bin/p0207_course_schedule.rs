#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 207. 课程表 (Course Schedule)
// 难度：中等 | 分类：图论
// https://leetcode.cn/problems/course-schedule/
//
// 本地验证：cargo t p0207_course_schedule
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // 2
        // [[1,0]]
        // 2
        // [[1,0],[0,1]]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::can_finish(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0207_course_schedule
}
