# [🟢 Easy] 242. Valid Anagram

**LeetCode:** [242. Valid Anagram](https://leetcode.com/problems/valid-anagram/)  
**Topics:** `String`, `Hash Table`, `Sorting`

## 📋 Description

Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`, and `false` otherwise.

\*anagram - An anagram is a word or phrase formed by rearranging the letters of a different word or phrase, using all the original letters exactly once.

## ⚖️ Constraints

| Parameter  | Range                                | Notes                     |
| ---------- | ------------------------------------ | ------------------------- |
| **Length** | `1 <= s.length, t.length <= 5 * 10⁴` | 1 to 50,000 elements      |
| **Values** | `'a' <= s[i],t[i] <= 'z'`            | English lowercase letters |

## 💡 Notes

- Заметка

## 🏷️ Tags

`#valid-anagram` `#hashmap` `#string` `#leetcode-easy`

## 🎯 Recommended Solution

### **Approach: Frequency Array Counter**

```rust
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut char_count = [0i32; 26];
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        // Increment for s, decrement for t in single combined loop
        for i in 0..s.len() {
            char_count[(s_bytes[i] - b'a') as usize] += 1;
            char_count[(t_bytes[i] - b'a') as usize] -= 1;
        }

        // Check if all counts are zero
        char_count.iter().all(|&count| count == 0)
    }
}
```
