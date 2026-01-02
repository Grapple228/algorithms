//! 1. Two Sum
//!
//! Given an array of integers nums and an integer target,
//! return indices of the two numbers such that they add up to target.

/// Входные данные для задачи
type Input = (Vec<i32>, i32);
/// Выходные данные (результат)
type Output = Vec<i32>;

use std::collections::HashMap;

/// Solution 1: brute force
/// Time: O(n²), Space: O(1)
/// Best for: n ≤ 20
fn solution1(nums: Vec<i32>, target: i32) -> Output {
    let n = nums.len();

    for i in 0..n {
        for j in i + 1..n {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![]
}

/// Solution 2: HashMap  
/// Time: O(n) average, Space: O(n)
/// Best for: n ≥ 100
fn solution2(nums: Vec<i32>, target: i32) -> Output {
    let mut hashmap = HashMap::with_capacity(nums.len());

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&j) = hashmap.get(&complement) {
            return vec![j as i32, i as i32];
        }

        hashmap.insert(num, i);
    }

    vec![]
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
            // Основные
            TestCase::new((vec![2, 7, 11, 15], 9), vec![0, 1]),
            TestCase::new((vec![3, 2, 4], 6), vec![1, 2]),
            TestCase::new((vec![3, 3], 6), vec![0, 1]),
            // Edge cases
            TestCase::new((vec![0, 4, 3, 0], 0), vec![0, 3]),
            TestCase::new((vec![-3, 4, 3, 90], 0), vec![0, 2]),
            TestCase::new((vec![1, 1, 1, 1, 1], 2), vec![0, 1]),
        ]
    }

    // endregion: --- Test Cases

    // region:    --- Basic Tests

    #[test]
    fn test_solution_1() {
        for tester in &test_cases() {
            tester.assert_solution(|(nums, target): Input| solution1(nums, target), "sol1");
        }
    }

    #[test]
    fn test_solution_2() {
        for tester in &test_cases() {
            tester.assert_solution(|(nums, target): Input| solution2(nums, target), "sol2");
        }
    }

    // endregion: --- Basic Tests

    // region:    --- Equality Tests

    #[test]
    fn test_solutions_equal() {
        test_all_solutions_for_cases!(
            test_cases(),
            |(nums, target): Input| solution1(nums, target),
            |(nums, target): Input| solution2(nums, target)
        );
    }
    // endregion: --- Equality Tests

    // region:    --- Benchmarks

    #[test]
    #[cfg(feature = "bench")]
    fn bench() {
        let solutions: Vec<(&str, Box<dyn Fn(Input) -> Output>)> = vec![
            ("sol1", Box::new(|(input, target)| solution1(input, target))),
            ("sol2", Box::new(|(input, target)| solution2(input, target))),
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
                ("sol1", Box::new(|(input, target)| solution1(input, target))),
                ("sol2", Box::new(|(input, target)| solution2(input, target))),
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
            let target = (size as i32 - 2) + (size as i32 - 1);

            let solutions: Vec<(&str, Box<dyn Fn(Input) -> Output>)> = vec![
                ("sol1", Box::new(|(input, target)| solution1(input, target))),
                ("sol2", Box::new(|(input, target)| solution2(input, target))),
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
            let results = compare_solutions(solutions, &(nums, target), iterations);
            print_comparison_table(&results);
        }
    }

    // endregion: --- Benchmarks
}

// endregion: --- Tests
