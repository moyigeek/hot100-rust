# LeetCode 热题 100 · Rust 练习工程

每题一个独立的 Cargo binary，自带力扣官方 Rust 初始代码、示例输入注释和公共数据结构。

## 环境要求

- Rust stable（`rust-toolchain.toml` 已声明，当前 `cargo 1.99.1`）
- `rustfmt`、`clippy` 已安装

## 快速开始

```bash
cargo t p0001_two_sum     # 跑单题测试（= cargo test --bin p0001_two_sum）
cargo r p0001_two_sum     # 跑单题 main（= cargo run --bin p0001_two_sum）
cargo build               # 编译全部 100 题
cargo clippy --all-targets -- -D warnings
cargo fmt                 # 格式化
```

## 目录结构

```text
.
├── Cargo.toml
├── src/
│   ├── lib.rs            # list! / tree! 宏
│   ├── prelude.rs        # ListNode / TreeNode、构造与序列化辅助
│   └── bin/              # 100 个题目，一题一个文件
│       ├── p0001_two_sum.rs
│       └── ...
└── scripts/progress.sh   # 查看完成进度
```

## 刷题流程

1. 在下方索引里打开题目文件。
2. 把 `tests` 中注释掉的输入改写成断言。
3. 实现 `impl Solution`（设计类实现 `struct` + `impl`）。
4. 运行 `cargo t pXXXX_name`，通过后复制到力扣提交。
5. 运行 `./scripts/progress.sh` 查看总体进度。

## 公共库用法

```rust
use leetcode::prelude::*;

// 单链表：list![1, 2, 3]
let head = list![1, 2, 3];
assert_eq!(list_to_vec(head), vec![1, 2, 3]);

// 二叉树：null 表示空节点，可直接粘贴力扣的层序数组
let root = tree![3, 9, 20, null, null, 15, 7];
assert_eq!(
    tree_to_vec(root),
    vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
);
```

可用的辅助函数：`to_list`、`list_to_vec`、`build_tree`、`tree_to_vec`。
`prelude` 还会导入 `HashMap`、`HashSet`、`BinaryHeap`、`VecDeque`、`Rc`、`RefCell` 等常用类型。

## 提交到力扣

- 只复制 `impl Solution { ... }`；设计类复制 `struct Xxx` 和 `impl Xxx`。
- **不要**复制 `use leetcode::prelude::*;`、`pub struct Solution;`、`#[cfg(test)] mod tests`、`fn main`。
- 力扣会自动提供 `ListNode` / `TreeNode` 的定义。
- 设计类官方模板方法接收 `&self`，需要修改字段时改成 `&mut self`。

## 特别说明

力扣的 Rust 判题暂不支持 4 道链表题，工程里提供了基于 `Rc<RefCell<...>>` 的本地版本：
[141. 环形链表](https://leetcode.cn/problems/linked-list-cycle/)、
[142. 环形链表 II](https://leetcode.cn/problems/linked-list-cycle-ii/)、
[160. 相交链表](https://leetcode.cn/problems/intersection-of-two-linked-lists/)、
[138. 随机链表的复制](https://leetcode.cn/problems/copy-list-with-random-pointer/)。

进度脚本按文件中是否还存在 `todo!()` 统计。

## 题目索引

### 哈希（3 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 1 | [两数之和](https://leetcode.cn/problems/two-sum/) | 简单 | [p0001_two_sum](src/bin/p0001_two_sum.rs) |
| 49 | [字母异位词分组](https://leetcode.cn/problems/group-anagrams/) | 中等 | [p0049_group_anagrams](src/bin/p0049_group_anagrams.rs) |
| 128 | [最长连续序列](https://leetcode.cn/problems/longest-consecutive-sequence/) | 中等 | [p0128_longest_consecutive_sequence](src/bin/p0128_longest_consecutive_sequence.rs) |

### 双指针（4 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 283 | [移动零](https://leetcode.cn/problems/move-zeroes/) | 简单 | [p0283_move_zeroes](src/bin/p0283_move_zeroes.rs) |
| 11 | [盛最多水的容器](https://leetcode.cn/problems/container-with-most-water/) | 中等 | [p0011_container_with_most_water](src/bin/p0011_container_with_most_water.rs) |
| 15 | [三数之和](https://leetcode.cn/problems/3sum/) | 中等 | [p0015_3sum](src/bin/p0015_3sum.rs) |
| 42 | [接雨水](https://leetcode.cn/problems/trapping-rain-water/) | 困难 | [p0042_trapping_rain_water](src/bin/p0042_trapping_rain_water.rs) |

### 滑动窗口（2 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 3 | [无重复字符的最长子串](https://leetcode.cn/problems/longest-substring-without-repeating-characters/) | 中等 | [p0003_longest_substring_without_repeating_characters](src/bin/p0003_longest_substring_without_repeating_characters.rs) |
| 438 | [找到字符串中所有字母异位词](https://leetcode.cn/problems/find-all-anagrams-in-a-string/) | 中等 | [p0438_find_all_anagrams_in_a_string](src/bin/p0438_find_all_anagrams_in_a_string.rs) |

### 子串（3 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 560 | [和为 K 的子数组](https://leetcode.cn/problems/subarray-sum-equals-k/) | 中等 | [p0560_subarray_sum_equals_k](src/bin/p0560_subarray_sum_equals_k.rs) |
| 239 | [滑动窗口最大值](https://leetcode.cn/problems/sliding-window-maximum/) | 困难 | [p0239_sliding_window_maximum](src/bin/p0239_sliding_window_maximum.rs) |
| 76 | [最小覆盖子串](https://leetcode.cn/problems/minimum-window-substring/) | 困难 | [p0076_minimum_window_substring](src/bin/p0076_minimum_window_substring.rs) |

### 普通数组（5 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 53 | [最大子数组和](https://leetcode.cn/problems/maximum-subarray/) | 中等 | [p0053_maximum_subarray](src/bin/p0053_maximum_subarray.rs) |
| 56 | [合并区间](https://leetcode.cn/problems/merge-intervals/) | 中等 | [p0056_merge_intervals](src/bin/p0056_merge_intervals.rs) |
| 189 | [轮转数组](https://leetcode.cn/problems/rotate-array/) | 中等 | [p0189_rotate_array](src/bin/p0189_rotate_array.rs) |
| 238 | [除了自身以外数组的乘积](https://leetcode.cn/problems/product-of-array-except-self/) | 中等 | [p0238_product_of_array_except_self](src/bin/p0238_product_of_array_except_self.rs) |
| 41 | [缺失的第一个正数](https://leetcode.cn/problems/first-missing-positive/) | 困难 | [p0041_first_missing_positive](src/bin/p0041_first_missing_positive.rs) |

### 矩阵（4 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 73 | [矩阵置零](https://leetcode.cn/problems/set-matrix-zeroes/) | 中等 | [p0073_set_matrix_zeroes](src/bin/p0073_set_matrix_zeroes.rs) |
| 54 | [螺旋矩阵](https://leetcode.cn/problems/spiral-matrix/) | 中等 | [p0054_spiral_matrix](src/bin/p0054_spiral_matrix.rs) |
| 48 | [旋转图像](https://leetcode.cn/problems/rotate-image/) | 中等 | [p0048_rotate_image](src/bin/p0048_rotate_image.rs) |
| 240 | [搜索二维矩阵 II](https://leetcode.cn/problems/search-a-2d-matrix-ii/) | 中等 | [p0240_search_a_2d_matrix_ii](src/bin/p0240_search_a_2d_matrix_ii.rs) |

### 链表（14 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 160 | [相交链表](https://leetcode.cn/problems/intersection-of-two-linked-lists/) ⚠️ | 简单 | [p0160_intersection_of_two_linked_lists](src/bin/p0160_intersection_of_two_linked_lists.rs) |
| 206 | [反转链表](https://leetcode.cn/problems/reverse-linked-list/) | 简单 | [p0206_reverse_linked_list](src/bin/p0206_reverse_linked_list.rs) |
| 234 | [回文链表](https://leetcode.cn/problems/palindrome-linked-list/) | 简单 | [p0234_palindrome_linked_list](src/bin/p0234_palindrome_linked_list.rs) |
| 141 | [环形链表](https://leetcode.cn/problems/linked-list-cycle/) ⚠️ | 简单 | [p0141_linked_list_cycle](src/bin/p0141_linked_list_cycle.rs) |
| 142 | [环形链表 II](https://leetcode.cn/problems/linked-list-cycle-ii/) ⚠️ | 中等 | [p0142_linked_list_cycle_ii](src/bin/p0142_linked_list_cycle_ii.rs) |
| 21 | [合并两个有序链表](https://leetcode.cn/problems/merge-two-sorted-lists/) | 简单 | [p0021_merge_two_sorted_lists](src/bin/p0021_merge_two_sorted_lists.rs) |
| 2 | [两数相加](https://leetcode.cn/problems/add-two-numbers/) | 中等 | [p0002_add_two_numbers](src/bin/p0002_add_two_numbers.rs) |
| 19 | [删除链表的倒数第 N 个结点](https://leetcode.cn/problems/remove-nth-node-from-end-of-list/) | 中等 | [p0019_remove_nth_node_from_end_of_list](src/bin/p0019_remove_nth_node_from_end_of_list.rs) |
| 24 | [两两交换链表中的节点](https://leetcode.cn/problems/swap-nodes-in-pairs/) | 中等 | [p0024_swap_nodes_in_pairs](src/bin/p0024_swap_nodes_in_pairs.rs) |
| 25 | [K 个一组翻转链表](https://leetcode.cn/problems/reverse-nodes-in-k-group/) | 困难 | [p0025_reverse_nodes_in_k_group](src/bin/p0025_reverse_nodes_in_k_group.rs) |
| 138 | [随机链表的复制](https://leetcode.cn/problems/copy-list-with-random-pointer/) ⚠️ | 中等 | [p0138_copy_list_with_random_pointer](src/bin/p0138_copy_list_with_random_pointer.rs) |
| 148 | [排序链表](https://leetcode.cn/problems/sort-list/) | 中等 | [p0148_sort_list](src/bin/p0148_sort_list.rs) |
| 23 | [合并 K 个升序链表](https://leetcode.cn/problems/merge-k-sorted-lists/) | 困难 | [p0023_merge_k_sorted_lists](src/bin/p0023_merge_k_sorted_lists.rs) |
| 146 | [LRU 缓存](https://leetcode.cn/problems/lru-cache/) | 中等 | [p0146_lru_cache](src/bin/p0146_lru_cache.rs) |

### 二叉树（15 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 94 | [二叉树的中序遍历](https://leetcode.cn/problems/binary-tree-inorder-traversal/) | 简单 | [p0094_binary_tree_inorder_traversal](src/bin/p0094_binary_tree_inorder_traversal.rs) |
| 104 | [二叉树的最大深度](https://leetcode.cn/problems/maximum-depth-of-binary-tree/) | 简单 | [p0104_maximum_depth_of_binary_tree](src/bin/p0104_maximum_depth_of_binary_tree.rs) |
| 226 | [翻转二叉树](https://leetcode.cn/problems/invert-binary-tree/) | 简单 | [p0226_invert_binary_tree](src/bin/p0226_invert_binary_tree.rs) |
| 101 | [对称二叉树](https://leetcode.cn/problems/symmetric-tree/) | 简单 | [p0101_symmetric_tree](src/bin/p0101_symmetric_tree.rs) |
| 543 | [二叉树的直径](https://leetcode.cn/problems/diameter-of-binary-tree/) | 简单 | [p0543_diameter_of_binary_tree](src/bin/p0543_diameter_of_binary_tree.rs) |
| 102 | [二叉树的层序遍历](https://leetcode.cn/problems/binary-tree-level-order-traversal/) | 中等 | [p0102_binary_tree_level_order_traversal](src/bin/p0102_binary_tree_level_order_traversal.rs) |
| 108 | [将有序数组转换为二叉搜索树](https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/) | 简单 | [p0108_convert_sorted_array_to_binary_search_tree](src/bin/p0108_convert_sorted_array_to_binary_search_tree.rs) |
| 98 | [验证二叉搜索树](https://leetcode.cn/problems/validate-binary-search-tree/) | 中等 | [p0098_validate_binary_search_tree](src/bin/p0098_validate_binary_search_tree.rs) |
| 230 | [二叉搜索树中第 K 小的元素](https://leetcode.cn/problems/kth-smallest-element-in-a-bst/) | 中等 | [p0230_kth_smallest_element_in_a_bst](src/bin/p0230_kth_smallest_element_in_a_bst.rs) |
| 199 | [二叉树的右视图](https://leetcode.cn/problems/binary-tree-right-side-view/) | 中等 | [p0199_binary_tree_right_side_view](src/bin/p0199_binary_tree_right_side_view.rs) |
| 114 | [二叉树展开为链表](https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/) | 中等 | [p0114_flatten_binary_tree_to_linked_list](src/bin/p0114_flatten_binary_tree_to_linked_list.rs) |
| 105 | [从前序与中序遍历序列构造二叉树](https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/) | 中等 | [p0105_construct_binary_tree_from_preorder_and_inorder_traversal](src/bin/p0105_construct_binary_tree_from_preorder_and_inorder_traversal.rs) |
| 437 | [路径总和 III](https://leetcode.cn/problems/path-sum-iii/) | 中等 | [p0437_path_sum_iii](src/bin/p0437_path_sum_iii.rs) |
| 236 | [二叉树的最近公共祖先](https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/) | 中等 | [p0236_lowest_common_ancestor_of_a_binary_tree](src/bin/p0236_lowest_common_ancestor_of_a_binary_tree.rs) |
| 124 | [二叉树中的最大路径和](https://leetcode.cn/problems/binary-tree-maximum-path-sum/) | 困难 | [p0124_binary_tree_maximum_path_sum](src/bin/p0124_binary_tree_maximum_path_sum.rs) |

### 图论（4 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 200 | [岛屿数量](https://leetcode.cn/problems/number-of-islands/) | 中等 | [p0200_number_of_islands](src/bin/p0200_number_of_islands.rs) |
| 994 | [腐烂的橘子](https://leetcode.cn/problems/rotting-oranges/) | 中等 | [p0994_rotting_oranges](src/bin/p0994_rotting_oranges.rs) |
| 207 | [课程表](https://leetcode.cn/problems/course-schedule/) | 中等 | [p0207_course_schedule](src/bin/p0207_course_schedule.rs) |
| 208 | [实现 Trie (前缀树)](https://leetcode.cn/problems/implement-trie-prefix-tree/) | 中等 | [p0208_implement_trie_prefix_tree](src/bin/p0208_implement_trie_prefix_tree.rs) |

### 回溯（8 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 46 | [全排列](https://leetcode.cn/problems/permutations/) | 中等 | [p0046_permutations](src/bin/p0046_permutations.rs) |
| 78 | [子集](https://leetcode.cn/problems/subsets/) | 中等 | [p0078_subsets](src/bin/p0078_subsets.rs) |
| 17 | [电话号码的字母组合](https://leetcode.cn/problems/letter-combinations-of-a-phone-number/) | 中等 | [p0017_letter_combinations_of_a_phone_number](src/bin/p0017_letter_combinations_of_a_phone_number.rs) |
| 39 | [组合总和](https://leetcode.cn/problems/combination-sum/) | 中等 | [p0039_combination_sum](src/bin/p0039_combination_sum.rs) |
| 22 | [括号生成](https://leetcode.cn/problems/generate-parentheses/) | 中等 | [p0022_generate_parentheses](src/bin/p0022_generate_parentheses.rs) |
| 79 | [单词搜索](https://leetcode.cn/problems/word-search/) | 中等 | [p0079_word_search](src/bin/p0079_word_search.rs) |
| 131 | [分割回文串](https://leetcode.cn/problems/palindrome-partitioning/) | 中等 | [p0131_palindrome_partitioning](src/bin/p0131_palindrome_partitioning.rs) |
| 51 | [N 皇后](https://leetcode.cn/problems/n-queens/) | 困难 | [p0051_n_queens](src/bin/p0051_n_queens.rs) |

### 二分查找（6 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 35 | [搜索插入位置](https://leetcode.cn/problems/search-insert-position/) | 简单 | [p0035_search_insert_position](src/bin/p0035_search_insert_position.rs) |
| 74 | [搜索二维矩阵](https://leetcode.cn/problems/search-a-2d-matrix/) | 中等 | [p0074_search_a_2d_matrix](src/bin/p0074_search_a_2d_matrix.rs) |
| 34 | [在排序数组中查找元素的第一个和最后一个位置](https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/) | 中等 | [p0034_find_first_and_last_position_of_element_in_sorted_array](src/bin/p0034_find_first_and_last_position_of_element_in_sorted_array.rs) |
| 33 | [搜索旋转排序数组](https://leetcode.cn/problems/search-in-rotated-sorted-array/) | 中等 | [p0033_search_in_rotated_sorted_array](src/bin/p0033_search_in_rotated_sorted_array.rs) |
| 153 | [寻找旋转排序数组中的最小值](https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array/) | 中等 | [p0153_find_minimum_in_rotated_sorted_array](src/bin/p0153_find_minimum_in_rotated_sorted_array.rs) |
| 4 | [寻找两个正序数组的中位数](https://leetcode.cn/problems/median-of-two-sorted-arrays/) | 困难 | [p0004_median_of_two_sorted_arrays](src/bin/p0004_median_of_two_sorted_arrays.rs) |

### 栈（5 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 20 | [有效的括号](https://leetcode.cn/problems/valid-parentheses/) | 简单 | [p0020_valid_parentheses](src/bin/p0020_valid_parentheses.rs) |
| 155 | [最小栈](https://leetcode.cn/problems/min-stack/) | 中等 | [p0155_min_stack](src/bin/p0155_min_stack.rs) |
| 394 | [字符串解码](https://leetcode.cn/problems/decode-string/) | 中等 | [p0394_decode_string](src/bin/p0394_decode_string.rs) |
| 739 | [每日温度](https://leetcode.cn/problems/daily-temperatures/) | 中等 | [p0739_daily_temperatures](src/bin/p0739_daily_temperatures.rs) |
| 84 | [柱状图中最大的矩形](https://leetcode.cn/problems/largest-rectangle-in-histogram/) | 困难 | [p0084_largest_rectangle_in_histogram](src/bin/p0084_largest_rectangle_in_histogram.rs) |

### 堆（3 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 215 | [数组中的第K个最大元素](https://leetcode.cn/problems/kth-largest-element-in-an-array/) | 中等 | [p0215_kth_largest_element_in_an_array](src/bin/p0215_kth_largest_element_in_an_array.rs) |
| 347 | [前 K 个高频元素](https://leetcode.cn/problems/top-k-frequent-elements/) | 中等 | [p0347_top_k_frequent_elements](src/bin/p0347_top_k_frequent_elements.rs) |
| 295 | [数据流的中位数](https://leetcode.cn/problems/find-median-from-data-stream/) | 困难 | [p0295_find_median_from_data_stream](src/bin/p0295_find_median_from_data_stream.rs) |

### 贪心算法（4 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 121 | [买卖股票的最佳时机](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/) | 简单 | [p0121_best_time_to_buy_and_sell_stock](src/bin/p0121_best_time_to_buy_and_sell_stock.rs) |
| 55 | [跳跃游戏](https://leetcode.cn/problems/jump-game/) | 中等 | [p0055_jump_game](src/bin/p0055_jump_game.rs) |
| 45 | [跳跃游戏 II](https://leetcode.cn/problems/jump-game-ii/) | 中等 | [p0045_jump_game_ii](src/bin/p0045_jump_game_ii.rs) |
| 763 | [划分字母区间](https://leetcode.cn/problems/partition-labels/) | 中等 | [p0763_partition_labels](src/bin/p0763_partition_labels.rs) |

### 动态规划（10 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 70 | [爬楼梯](https://leetcode.cn/problems/climbing-stairs/) | 简单 | [p0070_climbing_stairs](src/bin/p0070_climbing_stairs.rs) |
| 118 | [杨辉三角](https://leetcode.cn/problems/pascals-triangle/) | 简单 | [p0118_pascals_triangle](src/bin/p0118_pascals_triangle.rs) |
| 198 | [打家劫舍](https://leetcode.cn/problems/house-robber/) | 中等 | [p0198_house_robber](src/bin/p0198_house_robber.rs) |
| 279 | [完全平方数](https://leetcode.cn/problems/perfect-squares/) | 中等 | [p0279_perfect_squares](src/bin/p0279_perfect_squares.rs) |
| 322 | [零钱兑换](https://leetcode.cn/problems/coin-change/) | 中等 | [p0322_coin_change](src/bin/p0322_coin_change.rs) |
| 139 | [单词拆分](https://leetcode.cn/problems/word-break/) | 中等 | [p0139_word_break](src/bin/p0139_word_break.rs) |
| 300 | [最长递增子序列](https://leetcode.cn/problems/longest-increasing-subsequence/) | 中等 | [p0300_longest_increasing_subsequence](src/bin/p0300_longest_increasing_subsequence.rs) |
| 152 | [乘积最大子数组](https://leetcode.cn/problems/maximum-product-subarray/) | 中等 | [p0152_maximum_product_subarray](src/bin/p0152_maximum_product_subarray.rs) |
| 416 | [分割等和子集](https://leetcode.cn/problems/partition-equal-subset-sum/) | 中等 | [p0416_partition_equal_subset_sum](src/bin/p0416_partition_equal_subset_sum.rs) |
| 32 | [最长有效括号](https://leetcode.cn/problems/longest-valid-parentheses/) | 困难 | [p0032_longest_valid_parentheses](src/bin/p0032_longest_valid_parentheses.rs) |

### 多维动态规划（5 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 62 | [不同路径](https://leetcode.cn/problems/unique-paths/) | 中等 | [p0062_unique_paths](src/bin/p0062_unique_paths.rs) |
| 64 | [最小路径和](https://leetcode.cn/problems/minimum-path-sum/) | 中等 | [p0064_minimum_path_sum](src/bin/p0064_minimum_path_sum.rs) |
| 5 | [最长回文子串](https://leetcode.cn/problems/longest-palindromic-substring/) | 中等 | [p0005_longest_palindromic_substring](src/bin/p0005_longest_palindromic_substring.rs) |
| 1143 | [最长公共子序列](https://leetcode.cn/problems/longest-common-subsequence/) | 中等 | [p1143_longest_common_subsequence](src/bin/p1143_longest_common_subsequence.rs) |
| 72 | [编辑距离](https://leetcode.cn/problems/edit-distance/) | 中等 | [p0072_edit_distance](src/bin/p0072_edit_distance.rs) |

### 技巧（5 题）

| # | 题目 | 难度 | 代码 |
|---|------|------|------|
| 136 | [只出现一次的数字](https://leetcode.cn/problems/single-number/) | 简单 | [p0136_single_number](src/bin/p0136_single_number.rs) |
| 169 | [多数元素](https://leetcode.cn/problems/majority-element/) | 简单 | [p0169_majority_element](src/bin/p0169_majority_element.rs) |
| 75 | [颜色分类](https://leetcode.cn/problems/sort-colors/) | 中等 | [p0075_sort_colors](src/bin/p0075_sort_colors.rs) |
| 31 | [下一个排列](https://leetcode.cn/problems/next-permutation/) | 中等 | [p0031_next_permutation](src/bin/p0031_next_permutation.rs) |
| 287 | [寻找重复数](https://leetcode.cn/problems/find-the-duplicate-number/) | 中等 | [p0287_find_the_duplicate_number](src/bin/p0287_find_the_duplicate_number.rs) |
