//! 217. Contains Duplicate
//!
//! Given an integer array `nums`, return `true`
//! if any value appears at least twice in the array,
//! and return `false` if every element is distinct.

/// Входные данные для задачи
type Input = Vec<i32>;
/// Выходные данные (результат)
type Output = bool;

use std::collections::HashSet;

/// Solution 1: brute force
/// Time: O(n²)
/// Space: O(1)
/// Key insight: Перебор всех возможных вариантов
fn solution1(nums: Vec<i32>) -> Output {
    let len = nums.len();

    for i in 0..len {
        for j in i + 1..len {
            if nums[i] == nums[j] {
                return true;
            }
        }
    }

    false
}

/// Solution 2: HashSet
/// Time: O(n)
/// Space: O(n)
/// Key insight: Если Set уже содержит значение, значит повтор
fn solution2(nums: Vec<i32>) -> Output {
    let mut set = HashSet::with_capacity(nums.len());

    for value in nums {
        if !set.insert(value) {
            return true;
        }
    }

    return false;
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_all_solutions_for_cases, utils::*};

    // region:    --- Test Cases

    /// Создание набора тестов
    fn test_cases() -> Vec<TestCase<Input, Output>> {
        vec![
            // Пример из LeetCode
            TestCase::new(vec![1, 2, 3, 1], true),
            TestCase::new(vec![1, 2, 3, 4], false),
            TestCase::new(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
            // Edge cases
            // TestCase::new((vec![], 0), false), // пустой массив
            // TestCase::new((vec![1], 1), false), // один элемент
        ]
    }

    // endregion: --- Test Cases

    // region:    --- Basic Tests

    #[test]
    fn test_solution_1() {
        for tester in &test_cases() {
            tester.assert_solution(|nums: Input| solution1(nums), "sol1");
        }
    }

    #[test]
    fn test_solution_2() {
        for tester in &test_cases() {
            tester.assert_solution(|nums: Input| solution2(nums), "sol2");
        }
    }

    // endregion: --- Basic Tests

    // region:    --- Equality Tests

    #[test]
    fn test_solutions_equal() {
        test_all_solutions_for_cases!(
            test_cases(),
            |nums: Input| solution1(nums),
            |nums: Input| solution2(nums)
        );
    }
    // endregion: --- Equality Tests

    // region:    --- Benchmarks

    #[test]
    #[cfg(feature = "bench")]
    fn bench() {
        let solutions: Vec<(&str, Box<dyn Fn(Input) -> Output>)> = vec![
            ("sol1", Box::new(|input| solution1(input))),
            ("sol2", Box::new(|input| solution2(input))),
        ];

        let input = &test_cases()[0].input;
        let results = compare_solutions(solutions, input, bench_config::SMALL_ITERATIONS);

        print_comparison_table(&results);
        analyze_benchmark_results(&results);
    }

    #[test]
    #[cfg(feature = "bench")]
    fn bench_cases() {
        for case in test_cases() {
            let solutions: Vec<(&str, Box<dyn Fn(Input) -> Output>)> = vec![
                ("sol1", Box::new(|input| solution1(input))),
                ("sol2", Box::new(|input| solution2(input))),
            ];

            let results = compare_solutions(solutions, &case.input, bench_config::SMALL_ITERATIONS);
            print_comparison_table(&results);
            analyze_benchmark_results(&results);
        }
    }

    #[test]
    #[cfg(feature = "bench")]
    fn bench_scaling() {
        let sizes = [4, 10, 100, 500, 1000];

        for &size in &sizes {
            // Генерируем большой массив
            let nums: Vec<i32> = (0..size as i32).collect();

            let solutions: Vec<(&str, Box<dyn Fn(Input) -> Output>)> = vec![
                ("sol1", Box::new(|input| solution1(input))),
                ("sol2", Box::new(|input| solution2(input))),
            ];

            let iterations = match size {
                4 => 100_000, // 30 ns × 100k = 3 ms
                10 => 50_000, // 200 ns × 50k = 10 ms
                100 => 1_000, // 20 µs × 1k = 20 ms
                500 => 100,
                1000 => 10, // 2 ms × 10 = 20 ms
                10000 => 1, // 140 ms × 1 = 140 ms
                _ => 1,
            };

            println!("\n=== n = {} ===", size);
            let results = compare_solutions(solutions, &nums, iterations);
            print_comparison_table(&results);
        }
    }

    // endregion: --- Benchmarks
}

// endregion: --- Tests
