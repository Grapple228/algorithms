//! 349. Intersection of Two Arrays
//!
//! Given two integer arrays nums1 and nums2, return an array of
//! their intersection. Each element in the result must be unique
//! and you may return the result in any order.

/// Входные данные для задачи
type Input = (Vec<i32>, Vec<i32>);
/// Выходные данные (результат)
type Output = Vec<i32>;

use core::num;
use std::collections::{HashMap, HashSet};

/// Solution 1: Brute Force - двойной цикл
/// Time: O(n*m) где n = nums1.len(), m = nums2.len()
/// Space: O(min(n,m)) для хранения результата
/// Key insight: Простейший подход, катастрофически медленный на больших данных
fn solution1(nums1: Vec<i32>, nums2: Vec<i32>) -> Output {
    let mut result = HashSet::new();

    for &num1 in &nums1 {
        for &num2 in &nums2 {
            if num1 == num2 {
                result.insert(num1);
            }
        }
    }

    result.into_iter().collect()
}

/// Solution 2: Два HashSet'а - первый в HashSet, второй проверяем
/// Time: O(n + m) - один проход по каждому массиву
/// Space: O(n + m) - храним оба HashSet'а
/// Key insight: Используем HashSet для O(1) проверки наличия элемента
fn solution2(nums1: Vec<i32>, nums2: Vec<i32>) -> Output {
    let set1: HashSet<_> = nums1.into_iter().collect();
    let mut set2 = HashSet::new();

    for num in nums2 {
        if set1.contains(&num) {
            set2.insert(num);
        }
    }

    set2.into_iter().collect()
}

/// Solution 3: Сортировка + два указателя
/// Time: O(n log n + m log m) для сортировки + O(n + m) для сравнения
/// Space: O(min(n,m)) для результата (сортировка in-place)
/// Key insight: Сортировка позволяет использовать два указателя для линейного прохода
fn solution3(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Output {
    nums1.sort();
    nums2.sort();

    let mut result = vec![];
    let mut i1 = 0;
    let mut i2 = 0;
    let mut added = 0;

    while i1 < nums1.len() && i2 < nums2.len() {
        if nums1[i1] < nums2[i2] {
            i1 += 1;
        } else if nums1[i1] > nums2[i2] {
            i2 += 1;
        } else if added > 0 && nums1[i1] == result[added - 1] {
            i1 += 1;
            i2 += 1;
        } else {
            result.push(nums1[i1]);
            added += 1;
        }
    }

    result
}

/// Solution 4: Оптимизированная сортировка + два указателя
/// Time: O(n log n + m log m) но быстрее за счет sort_unstable
/// Space: O(min(n,m)) для результата
/// Key insight: sort_unstable быстрее sort, result.last() безопаснее индексации
fn solution4(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Output {
    nums1.sort_unstable();
    nums2.sort_unstable();

    let mut result = vec![];
    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < nums1.len() && i2 < nums2.len() {
        use std::cmp::Ordering;
        match nums1[i1].cmp(&nums2[i2]) {
            Ordering::Less => i1 += 1,
            Ordering::Greater => i2 += 1,
            Ordering::Equal => {
                if result.last() != Some(&nums1[i1]) {
                    result.push(nums1[i1]);
                }

                i1 += 1;
                i2 += 1;
            }
        }
    }

    result
}

/// Solution 5: Counting array (массив-фильтр)
/// Time: O(n + m) - линейное время
/// Space: O(1) - фиксированный массив на 1001 элемент
/// Key insight: Используем тот факт, что значения ограничены 0-1000
/// Рекомендации:
/// - Использовать если известно, что значения в диапазоне 0-1000
/// - Самый быстрый вариант для больших массивов
/// - Не использовать если значения могут быть любыми
fn solution5(nums1: Vec<i32>, nums2: Vec<i32>) -> Output {
    // Так как элементы всего от 0 до 1000 можно использовать массив
    // число = индекс в массиве
    // O(n+m) time, O(1) space (fixed 1001 elements)
    let mut seen = [false; 1001]; // 0-1000 включительно

    // Отмечаем элементы из первого массива
    for &num in &nums1 {
        seen[num as usize] = true;
    }

    let mut result = Vec::new();

    // Ищем элементы которые есть в обоих массивах
    for &num in &nums2 {
        if seen[num as usize] {
            result.push(num);
            seen[num as usize] = false; // Помечаем как уже добавленное
        }
    }

    result
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_all_solutions_for_cases, test_all_solutions_for_cases_set, utils::*};

    // region:    --- Test Cases

    /// Создание набора тестов
    fn test_cases() -> Vec<TestCase<Input, Output>> {
        vec![
            // Пример из LeetCode
            TestCase::new_sorted((vec![1, 2, 2, 1], vec![2, 2]), vec![2]),
            TestCase::new_sorted((vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![9, 4]),
            TestCase::new_sorted(
                (
                    (0..300).into_iter().collect(),
                    (100..200).into_iter().collect(),
                ),
                (100..200).into_iter().collect(),
            ),
            // Edge cases
            // TestCase::new((vec![], 0), vec![]), // пустой массив
            // TestCase::new((vec![1], 1), vec![]), // один элемент
        ]
    }

    // endregion: --- Test Cases

    // region:    --- Basic Tests

    #[test]
    fn test_solution_1() {
        for tester in &test_cases() {
            tester.assert_solution_set(|(nums, target): Input| solution1(nums, target), "sol1");
        }
    }

    #[test]
    fn test_solution_2() {
        for tester in &test_cases() {
            tester.assert_solution_set(|(nums, target): Input| solution2(nums, target), "sol2");
        }
    }

    #[test]
    fn test_solution_3() {
        for tester in &test_cases() {
            tester.assert_solution_set(|(nums, target): Input| solution3(nums, target), "sol3");
        }
    }

    #[test]
    fn test_solution_4() {
        for tester in &test_cases() {
            tester.assert_solution_set(|(nums, target): Input| solution4(nums, target), "sol4");
        }
    }

    #[test]
    fn test_solution_5() {
        for tester in &test_cases() {
            tester.assert_solution_set(|(nums, target): Input| solution5(nums, target), "sol5");
        }
    }

    // endregion: --- Basic Tests

    // region:    --- Equality Tests

    #[test]
    fn test_solutions_equal() {
        test_all_solutions_for_cases_set!(
            test_cases(),
            |(nums, target): Input| solution1(nums, target),
            |(nums, target): Input| solution2(nums, target),
            |(nums, target): Input| solution3(nums, target),
            |(nums, target): Input| solution4(nums, target),
            |(nums, target): Input| solution5(nums, target),
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
            ("sol3", Box::new(|(input, target)| solution3(input, target))),
            ("sol4", Box::new(|(input, target)| solution4(input, target))),
            ("sol5", Box::new(|(input, target)| solution5(input, target))),
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
                ("sol3", Box::new(|(input, target)| solution3(input, target))),
                ("sol4", Box::new(|(input, target)| solution4(input, target))),
                ("sol5", Box::new(|(input, target)| solution5(input, target))),
            ];

            let results = compare_solutions(solutions, &case.input, bench_config::SMALL_ITERATIONS);
            print_comparison_table(&results);
            analyze_benchmark_results(&results);
        }
    }

    // #[test]
    // #[cfg(feature = "bench")]
    // fn bench_scaling() {
    //     let sizes = [4, 10, 100, 500, 1000];

    //     for &size in &sizes {
    //         // Генерируем большой массив
    //         let nums: Vec<i32> = (0..size as i32).collect();
    //         let target = (size as i32 - 2) + (size as i32 - 1);

    //         let solutions: Vec<(&str, Box<dyn Fn(Input) -> Output>)> = vec![
    //             ("sol1", Box::new(|(input, target)| solution1(input, target))),
    //             // ("sol2", Box::new(|(input, target)| solution2(input, target))),
    //         ];

    //         let iterations = match size {
    //             4 => 100_000, // 30 ns × 100k = 3 ms
    //             10 => 50_000, // 200 ns × 50k = 10 ms
    //             100 => 1_000, // 20 µs × 1k = 20 ms
    //             500 => 100,
    //             1000 => 10, // 2 ms × 10 = 20 ms
    //             10000 => 1, // 140 ms × 1 = 140 ms
    //             _ => 1,
    //         };

    //         println!("\n=== n = {} ===", size);
    //         let results = compare_solutions(solutions, &(nums, target), iterations);
    //         print_comparison_table(&results);
    //     }
    // }

    // endregion: --- Benchmarks
}

// endregion: --- Tests
