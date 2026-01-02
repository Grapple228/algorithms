//! 1. Reverse array
//!
//! Given an array of integers nums,
//! return reversed array of integers.

/// Solution 1: mutable reverse
fn solution1(mut nums: Vec<i32>) -> Vec<i32> {
    nums.reverse();
    nums
}

// /// Solution 2: Iterator collect
// fn solution2(nums: Vec<i32>) -> Vec<i32> {
//     nums.into_iter().rev().collect()
// }

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_all_solutions;
    use crate::test_all_solutions_for_cases;
    use crate::utils::*;

    // region:    --- Test Cases

    /// Создание набора тестов
    fn test_cases() -> Vec<TestCase<Vec<i32>, Vec<i32>>> {
        vec![
            TestCase::new(vec![1, 2, 3], vec![3, 2, 1]),
            TestCase::new(vec![2, 7, 11, 15], vec![15, 11, 7, 2]),
            TestCase::new(vec![], vec![]),
            TestCase::new(vec![5], vec![5]),
        ]
    }

    // endregion: --- Test Cases

    // region:    --- Basic Tests

    #[test]
    fn test_solution_1() {
        let tester = TestCase::new(vec![2, 7, 11, 15], vec![15, 11, 7, 2]);
        tester.run_and_verify(|nums| solution1(nums), "solution1");
    }

    // #[test]
    // fn test_solution_2() {
    //     let tester = TestCase::new(vec![2, 7, 11, 15], vec![15, 11, 7, 2]);
    //     tester.run_and_verify(|nums| solution2(nums), "solution2");
    // }

    // endregion: --- Basic Tests

    // region:    --- Equality Tests

    #[test]
    fn test_solutions_equal() {
        let cases = test_cases();
        test_all_solutions_for_cases!(cases, solution1);
    }

    // endregion: --- Equality Tests

    // region:    --- Benchmarks

    #[test]
    fn bench() {
        // Используем стандартную конфигурацию
        let solutions: Vec<(&str, Box<dyn Fn(Vec<i32>) -> Vec<i32>>)> = vec![
            ("reverse", Box::new(|input| solution1(input))),
            // ("iterator", Box::new(|input| solution2(input))),
        ];

        let results = compare_solutions(
            solutions,
            bench_config::standard_test_input(),
            bench_config::SMALL_ITERATIONS,
        );

        print_comparison_table(&results);
        analyze_benchmark_results(&results);
    }

    #[test]
    fn bench_cases() {
        let sizes = [10, 100, 1000, 10000];

        for &size in &sizes {
            let input: Vec<i32> = (0..size).collect();
            let iterations = match size {
                s if s <= 100 => 100_000,
                s if s <= 1000 => 10_000,
                _ => 1_000,
            };

            println!("\n=== n = {} ({} iterations) ===", size, iterations);

            let solutions: Vec<(&str, Box<dyn Fn(Vec<i32>) -> Vec<i32>>)> = vec![
                ("reverse", Box::new(|input| solution1(input))),
                // ("iterator", Box::new(|input| solution2(input))),
            ];

            let results = compare_solutions(solutions, input, iterations);
            print_comparison_table(&results);
            analyze_benchmark_results(&results);
        }
    }

    // endregion: --- Benchmarks
}

// endregion: --- Tests
