//! 49. Group anagrams
//!
//! Given an array of strings strs, group the together.
//! You can return the answer in any order.
//!
//! An Anagram is a word or phrase formed by rearranging the letters
//! of a different word or phrase, typically using all the original
//! letters exactly once.

/// Входные данные для задачи
type Input = Vec<String>;
/// Выходные данные (результат)
type Output = Vec<Vec<String>>;

use std::collections::HashMap;

/// Solution 1: Группировка по хэшу символов
/// Time: O(n*k) где n - количество строк, k - максимальная длина строки
/// Space: O(n*k) для хранения результатов
/// Key insight: использовать отсортированный массив символов как ключ
pub fn solution1(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[u32; 26], Vec<usize>> = HashMap::new();

    for i in 0..strs.len() {
        let cur = &strs[i];
        let mut char_count = [0_32; 26];

        for &byte in cur.as_bytes() {
            char_count[(byte - b'a') as usize] += 1;
        }

        map.entry(char_count).or_insert(vec![]).push(i);
    }

    let mut result = Vec::with_capacity(map.len());

    for counts in map.values() {
        let mut inner = Vec::with_capacity(counts.len());

        for &i in counts {
            inner.push(strs[i].clone());
        }

        result.push(inner);
    }

    result
}

/// Solution 2: Группировка по хэшу символов c одним проходом
/// Time: O(n*k) где n - количество строк, k - максимальная длина строки
/// Space: O(n*k) для хранения результатов
/// Key insight: Вместо индекса в HashMap можем сразу хранить строку
/// В конце просто выполняем преобразование в нужный формат данных
/// Так как символы только в диапазоне a..z, то можем хранить u8 (байты), что ускорит программу
fn solution2(strs: Vec<String>) -> Output {
    let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut key = [0u8; 26];
        s.as_bytes().iter().for_each(|&b| {
            key[(b - b'a') as usize] += 1;
        });

        map.entry(key).or_insert_with(Vec::new).push(s);
    }

    map.into_values().collect()
}

/// Solution 3: Дополнительные оптимизации
/// Time: O(n*k)
/// Space: O(n*k)
fn solution3(strs: Vec<String>) -> Output {
    // Преаллокация HashMap для уменьшения реаллокаций
    let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());

    for s in strs {
        let mut key = [0u8; 26];

        // Использование while для избегания bounds checking
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        while i < len {
            let b = bytes[i];
            key[(b - b'a') as usize] += 1;
            i += 1;
        }

        map.entry(key)
            .or_insert_with(|| Vec::with_capacity(4)) // Предполагаем маленькие группы
            .push(s);
    }

    map.into_values().collect()
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_all_solutions_for_cases, utils::*};

    /// Вспомогательная функция для сортировки результата
    fn normalize_result(mut result: Output) -> Output {
        // Сортируем каждую внутреннюю группу
        for group in result.iter_mut() {
            group.sort();
        }
        // Сортируем внешний вектор по первому элементу
        result.sort_by(|a, b| {
            a.first()
                .unwrap_or(&String::new())
                .cmp(b.first().unwrap_or(&String::new()))
        });
        result
    }

    /// Вспомогательная функция для создания ожидаемого результата
    fn expected_result(groups: Vec<Vec<&str>>) -> Output {
        let mut result: Output = groups
            .into_iter()
            .map(|group| group.into_iter().map(String::from).collect())
            .collect();
        // Нормализуем ожидаемый результат сразу
        normalize_result(result)
    }

    // region:    --- Test Cases

    /// Создание набора тестов
    fn test_cases() -> Vec<TestCase<Input, Output>> {
        vec![
            // Пример из LeetCode
            TestCase::new(
                vec![
                    "eat".into(),
                    "tea".into(),
                    "tan".into(),
                    "ate".into(),
                    "nat".into(),
                    "bat".into(),
                ],
                expected_result(vec![
                    vec!["bat"],
                    vec!["nat", "tan"],
                    vec!["ate", "eat", "tea"],
                ]),
            ),
            TestCase::new(vec!["".into()], expected_result(vec![vec![""]])),
            TestCase::new(vec!["a".into()], expected_result(vec![vec!["a"]])),
        ]
    }

    // endregion: --- Test Cases

    // region:    --- Basic Tests

    #[test]
    fn test_solution_1() {
        let cases = test_cases();
        for case in cases {
            let input = case.input;
            let expected = case.expected;

            let result = solution1(input);

            // Нормализуем оба результата перед сравнением
            let normalized_result = normalize_result(result);
            let normalized_expected = normalize_result(expected);

            assert_eq!(normalized_result, normalized_expected);
        }
    }

    #[test]
    fn test_solution_2() {
        let cases = test_cases();
        for case in cases {
            let input = case.input;
            let expected = case.expected;

            let result = solution2(input);

            // Нормализуем оба результата перед сравнением
            let normalized_result = normalize_result(result);
            let normalized_expected = normalize_result(expected);

            assert_eq!(normalized_result, normalized_expected);
        }
    }

    #[test]
    fn test_solution_3() {
        let cases = test_cases();
        for case in cases {
            let input = case.input;
            let expected = case.expected;

            let result = solution3(input);

            // Нормализуем оба результата перед сравнением
            let normalized_result = normalize_result(result);
            let normalized_expected = normalize_result(expected);

            assert_eq!(normalized_result, normalized_expected);
        }
    }

    // endregion: --- Basic Tests

    // region:    --- Benchmarks

    #[test]
    #[cfg(feature = "bench")]
    fn bench() {
        let solutions: Vec<(&str, Box<dyn Fn(Input) -> Output>)> = vec![
            ("sol1", Box::new(|strs| solution1(strs))),
            ("sol2", Box::new(|strs| solution2(strs))),
            ("sol3", Box::new(|strs| solution3(strs))),
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
                ("sol1", Box::new(|strs| solution1(strs))),
                ("sol2", Box::new(|strs| solution2(strs))),
                ("sol3", Box::new(|strs| solution3(strs))),
            ];

            let results = compare_solutions(solutions, &case.input, bench_config::SMALL_ITERATIONS);
            print_comparison_table(&results);
            analyze_benchmark_results(&results);
        }
    }

    // endregion: --- Benchmarks
}

// endregion: --- Tests
