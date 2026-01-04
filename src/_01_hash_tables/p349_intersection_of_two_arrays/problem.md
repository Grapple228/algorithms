# [🟢 Easy] 349. Intersection of Two Arrays

**LeetCode:** [349. Intersection of Two Arrays](https://leetcode.com/problems/intersection-of-two-arrays/)  
**Topics:** `Array`, `Hash Table`, `Two Pointers`, `Binary Search`, `Sorting`

## 📋 Description

Given two integer arrays `nums1` and `nums2`, return an array of their **intersection**. Each element in the result must be **unique** and you may return the result in any order.

The intersection of two arrays is the set of elements that are present in both arrays.

## 📝 Examples

### Example 1

**Input:** nums1 = [1,2,2,1], nums2 = [2,2]  
**Output:** [2]  
**Explanation:** 2 is the only element present in both arrays.

### Example 2

**Input:** nums1 = [4,9,5], nums2 = [9,4,9,8,4]  
**Output:** [9,4] or [4,9]  
**Explanation:** 4 and 9 are the elements present in both arrays.

## ⚖️ Constraints

| Parameter        | Range                                            | Notes                                           |
| ---------------- | ------------------------------------------------ | ----------------------------------------------- |
| **Array Length** | `1 <= nums1.length, nums2.length <= 1000`        | Small to medium sized arrays                    |
| **Values**       | `0 <= nums1[i] <= 1000`, `0 <= nums2[i] <= 1000` | Values are bounded (important for optimization) |
| **Uniqueness**   | Result must have no duplicates                   | Each element appears only once in output        |
| **Order**        | Any order is acceptable                          | Simplifies the problem                          |

## 💡 Key Insights

### 1. **Value Range Constraint is Critical**

- Values are limited to `0 ≤ nums[i] ≤ 1000`
- Enables **O(1) space** counting array solution with fixed size 1001
- Without this constraint, need O(n) space for hash tables

### 2. **Multiple Valid Approaches**

- **Hash Set**: O(n+m) time, O(n+m) space - simplest
- **Sorting + Two Pointers**: O(n log n + m log m) time, O(min(n,m)) space - memory efficient
- **Counting Array**: O(n+m) time, O(1) space - fastest for this specific constraint
- **Binary Search**: O(n log m) or O(m log n) time - good if one array is much smaller

### 3. **Duplicate Handling**

- Input arrays can contain duplicates
- Output must contain only unique elements
- Need deduplication mechanism in all solutions

## 🔍 Complexity Analysis

| Approach                   | Time Complexity      | Space Complexity     | Best Use Case                       |
| -------------------------- | -------------------- | -------------------- | ----------------------------------- |
| **Brute Force**            | O(n × m)             | O(min(n,m))          | Never - educational only            |
| **Hash Set**               | O(n + m)             | O(n + m)             | General case, simple implementation |
| **Sorting + Two Pointers** | O(n log n + m log m) | O(min(n,m))          | Memory constrained environments     |
| **Counting Array**         | O(n + m)             | O(1) [1001 elements] | **Optimal for this problem**        |
| **Binary Search**          | O(n log m)           | O(min(n,m))          | One array much smaller than other   |

## 🏷️ Tags

`#intersection-of-arrays` `#hashset` `#two-pointers` `#counting-sort` `#leetcode-easy` `#array-intersection`

## 🎯 Recommended Solution

For LeetCode submission: **Counting Array** approach (Solution 5)

```rust
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut seen = [false; 1001];
        let mut result = Vec::new();

        // Mark elements from first array
        for &num in &nums1 {
            seen[num as usize] = true;
        }

        // Check elements from second array
        for &num in &nums2 {
            if seen[num as usize] {
                result.push(num);
                seen[num as usize] = false; // Prevent duplicates
            }
        }

        result
    }
}
```
