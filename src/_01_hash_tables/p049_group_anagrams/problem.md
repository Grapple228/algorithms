# [🟡 Medium] 49. Group Anagrams

**LeetCode:** [49. Group Anagrams](https://leetcode.com/problems/group-anagrams/)  
**Topics:** `String`, `Hash Table`, `Sorting`,
`Array`

## 📋 Description

Given an array of strings `strs`, group the anagrams\* together. You can return the answer in **any order**.

\*anagram - An anagram is a word or phrase formed by rearranging the letters of a different word or phrase, using all the original letters exactly once.

## ⚖️ Constraints

| Parameter       | Range                                  | Notes                     |
| --------------- | -------------------------------------- | ------------------------- |
| **Length**      | `1 <= strs.length <= 10⁴`              | 1 to 10,000 elements      |
| **Elem Length** | `0 <= strs[i].length, t.length <= 100` | 1 to 100 chars            |
| **Values**      | `'a' <= s[i] <= 'z'`                   | English lowercase letters |

## 💡 Notes

- Заметка

## 🏷️ Tags

`#group-anagrams` `#hashmap` `#string` `#leetcode-medium`

## 🎯 Recommended Solution

### **Approach: Frequency Counter Array as Hash Key**

```rust
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut key = [0u8; 26];

            // Count character frequencies
            for &byte in s.as_bytes() {
                key[(byte - b'a') as usize] += 1;
            }

            // Group by frequency array
            groups.entry(key)
                .or_insert_with(Vec::new)
                .push(s);
        }

        groups.into_values().collect()
    }
}
```
