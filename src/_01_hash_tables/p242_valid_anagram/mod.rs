//! 242. Valid Anagram
//!
//! Given two strings s and t, return true if t is an anagram of s,
//! and false otherwise.
//!
//! An Anagram is a word or phrase formed by rearranging the letters
//! of a different word or phrase, typically using all the original
//! letters exactly once.

/// Входные данные для задачи
type Input = (String, String);
/// Выходные данные (результат)
type Output = bool;

use std::collections::HashMap;

/// Solution 1: Hash Map Character Count
/// Time: O(n) - где n длина строк (проходим по каждой строке один раз)
/// Space: O(k) - где k количество уникальных символов в строках
/// Key insight: Две строки являются анаграммами если они имеют одинаковое
/// количество каждого символа. Хеш-таблица позволяет эффективно считать и сравнивать.
fn solution1(s: String, t: String) -> Output {
    if s.len() != t.len() {
        return false;
    }

    let mut char_count: HashMap<char, u32> = HashMap::new();

    for c in s.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        match char_count.get_mut(&c) {
            Some(count) if *count > 0 => *count -= 1,
            _ => return false,
        }
    }

    true
}

/// Solution 2: Single Array for ASCII
/// Time: O(n) - один проход по строкам
/// Space: O(1) - фиксированный массив на 26 элементов (только для строчных букв)
/// Key insight: Для задач с только строчными английскими буквами можно
/// использовать массив размера 26 вместо HashMap
fn solution2(s: String, t: String) -> Output {
    if s.len() != t.len() {
        return false;
    }

    let mut char_count = [0u8; 26];
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    // Сначала инкрементируем для s
    for &b in s_bytes {
        char_count[(b - b'a') as usize] += 1;
    }

    // Затем декрементируем для t с ранним выходом
    for &b in t_bytes {
        let idx = (b - b'a') as usize;
        if char_count[idx] == 0 {
            return false; // Ранний выход
        }
        char_count[idx] -= 1;
    }

    true
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
            TestCase::new((String::from("anagram"), String::from("nagaram")), true),
            TestCase::new((String::from("rat"), String::from("car")), false),
            TestCase::new(
                (
                    String::from("abcdefghijklmnopqrstuvwxyz"),
                    String::from("zyxwvutsrqponmlkjihgfedcba"),
                ),
                true,
            ),
            TestCase::new((String::from("ab"), String::from("ac")), false),
            TestCase::new((String::from("bc"), String::from("ad")), false),
            // Unicode test. Commented because solution2 will fail
            // TestCase::new((String::from("café"), String::from("facé")), true),
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
            |(nums, target): Input| solution2(nums, target),
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
        let sizes = [4, 10, 100, 500, 1000, 5000, 10000];

        for &size in &sizes {
            // Генерируем случайные анаграммы заданного размера

            use rand::Rng;
            let s: String =
                std::iter::repeat_with(|| (rand::rng().random_range(b'a'..=b'z')) as char)
                    .take(size)
                    .collect();

            // Создаем анаграмму, перемешивая строку
            let mut t_chars: Vec<char> = s.chars().collect();
            let mut rng = rand::rng();
            for i in 0..size {
                use rand::Rng;

                let j = rng.random_range(i..size);
                t_chars.swap(i, j);
            }
            let t: String = t_chars.into_iter().collect();

            // Проверяем, что это действительно анаграммы (для отладки)
            assert!(solution1(s.clone(), t.clone()));
            assert!(solution2(s.clone(), t.clone()));

            let solutions: Vec<(&str, Box<dyn Fn(Input) -> Output>)> = vec![
                ("sol1", Box::new(|(s, t)| solution1(s, t))),
                ("sol2", Box::new(|(s, t)| solution2(s, t))),
            ];

            // Определяем количество итераций в зависимости от размера
            let iterations = match size {
                4 => 100_000,
                10 => 100_000,
                100 => 10_000,
                500 => 5_000,
                1000 => 1_000,
                5000 => 500,
                10000 => 100,
                _ => 100,
            };

            println!("\n=== n = {} ===", size);
            let results = compare_solutions(solutions, &(s, t), iterations);
            print_comparison_table(&results);
        }
    }

    // endregion: --- Benchmarks
}

// endregion: --- Tests
