#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 49. 字母异位词分组 (Group Anagrams)
// 难度：中等 | 分类：哈希
// https://leetcode.cn/problems/group-anagrams/
//
// 本地验证：cargo t p0049_group_anagrams
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // ["eat","tea","tan","ate","nat","bat"]
        // [""]
        // ["a"]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::group_anagrams(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0049_group_anagrams
}
