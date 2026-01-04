# [🟢 Easy] 217. Contains Duplicate

**LeetCode:** [217. Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)  
**Topics:** `Array`, `Hash Table`, `Sorting`

## 📋 Description

Given an integer array `nums`, return `true` if any value appears at least twice in the array, and return `false` if every element is distinct.

## ⚖️ Constraints

| Parameter  | Range                     | Notes                           |
| ---------- | ------------------------- | ------------------------------- |
| **Length** | `1 <= nums.length <= 10⁵` | 1 to 100,000 elements           |
| **Values** | `-10⁹ <= nums[i] <= 10⁹`  | Large positive/negative numbers |

## 💡 Notes

- Заметка

## 🏷️ Tags

`#contains-duplicate` `#hashset` `#array` `#leetcode-easy`

## 🎯 Recommended Solution

### **Approach: HashSet with Early Return**

```rust
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut seen = HashSet::with_capacity(nums.len());

        for &num in &nums {
            if !seen.insert(num) {
                return true;  // Early return on first duplicate
            }
        }

        false
    }
}
```
