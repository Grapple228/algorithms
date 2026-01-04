//! Полезные макросы для тестирования

// utils/macros.rs

/// Проверка всех решений для одного тест-кейса
#[macro_export]
macro_rules! test_all_solutions {
    ($test_case:expr, $($solution:expr),+) => {
        {
            let mut results = Vec::new();
            $(results.push($test_case.execute($solution));)+

            if let Some(first) = results.first() {
                for (i, result) in results.iter().enumerate() {
                    assert_eq!(result, first, "Solution {} differs", i + 1);
                }
            }
        }
    };
}

/// Проверка всех решений для одного тест-кейса с сортировкой результатов
#[macro_export]
macro_rules! test_all_solutions_sorted {
    ($test_case:expr, $($solution:expr),+) => {
        {
            let mut sorted_results = Vec::new();

            $({
                let mut result = $test_case.execute($solution);
                result.sort();
                sorted_results.push(result);
            })+

            if let Some(first) = sorted_results.first() {
                for (i, result) in sorted_results.iter().enumerate() {
                    assert_eq!(result, first, "Solution {} differs after sorting", i + 1);
                }
            }
        }
    };
}

/// Проверка всех решений для одного тест-кейса через HashSet
#[macro_export]
macro_rules! test_all_solutions_set {
    ($test_case:expr, $($solution:expr),+) => {
        {
            use std::collections::HashSet;

            let mut set_results = Vec::new();

            $({
                let result = $test_case.execute($solution);
                let set: HashSet<_> = result.into_iter().collect();
                set_results.push(set);
            })+

            if let Some(first) = set_results.first() {
                for (i, result) in set_results.iter().enumerate() {
                    assert_eq!(result, first, "Solution {} differs (set comparison)", i + 1);
                }
            }
        }
    };
}

/// Проверка всех решений для вектора тест-кейсов
#[macro_export]
macro_rules! test_all_solutions_for_cases {
    ($test_cases:expr, $($solution:expr),+ $(,)?) => {
        {
            // Сохраняем решения как замыкания, принимающие I
            let solutions: Vec<(&str, Box<dyn Fn(_) -> _>)> = vec![
                $(
                    (stringify!($solution), Box::new(|input| {
                        // Преобразуем в нужный тип внутри
                        $solution(input)
                    })),
                )+
            ];

            let mut total_passed = 0;

            for test_case in $test_cases {
                let mut results = Vec::new();

                for (name, sol) in &solutions {
                    let result = test_case.execute(|input| sol(input));
                    results.push(result);
                }

                // Проверяем, что все результаты одинаковы
                let first_result = &results[0];
                for (i, result) in results.iter().enumerate() {
                    assert_eq!(
                        result, first_result,
                        "Solution {} differs from {} for input {:?}",
                        solutions[i].0, solutions[0].0, test_case.input
                    );
                }

                // Проверяем, что результат правильный
                assert_eq!(
                    first_result, &test_case.expected,
                    "All solutions agree but wrong for input {:?}",
                    test_case.input
                );

                total_passed += 1;
            }

            println!("✅ All {} solutions passed {} test cases",
                     solutions.len(), total_passed);
        }
    };
}

/// Проверка всех решений для вектора тест-кейсов с сортировкой результатов
#[macro_export]
macro_rules! test_all_solutions_for_cases_sorted {
    ($test_cases:expr, $($solution:expr),+ $(,)?) => {
        {
            // Сохраняем решения как замыкания, принимающие I
            let solutions: Vec<(&str, Box<dyn Fn(_) -> _>)> = vec![
                $(
                    (stringify!($solution), Box::new(|input| {
                        $solution(input)
                    })),
                )+
            ];

            let mut total_passed = 0;

            for test_case in $test_cases {
                let mut results = Vec::new();

                // Получаем результаты от всех решений
                for (name, sol) in &solutions {
                    let result = test_case.execute(|input| sol(input));
                    results.push((name, result));
                }

                // Сортируем результаты для сравнения
                let mut sorted_results = Vec::new();
                for (name, mut result) in results {
                    result.sort();
                    sorted_results.push((name, result));
                }

                // Проверяем, что все отсортированные результаты одинаковы
                let first_result = &sorted_results[0].1;
                for (i, (name, result)) in sorted_results.iter().enumerate() {
                    assert_eq!(
                        result, first_result,
                        "Solution {} differs from {} for input {:?} (after sorting)",
                        name, sorted_results[0].0, test_case.input
                    );
                }

                // Проверяем, что результат правильный (сравниваем с отсортированным expected)
                let mut sorted_expected = test_case.expected.clone();
                sorted_expected.sort();
                assert_eq!(
                    first_result, &sorted_expected,
                    "All solutions agree but wrong for input {:?}",
                    test_case.input
                );

                total_passed += 1;
            }

            println!("✅ All {} solutions passed {} test cases (sorted comparison)",
                     solutions.len(), total_passed);
        }
    };
}

/// Проверка всех решений для вектора тест-кейсов через HashSet
#[macro_export]
macro_rules! test_all_solutions_for_cases_set {
    ($test_cases:expr, $($solution:expr),+ $(,)?) => {
        {
            use std::collections::HashSet;

            // Сохраняем решения как замыкания, принимающие I
            let solutions: Vec<(&str, Box<dyn Fn(_) -> _>)> = vec![
                $(
                    (stringify!($solution), Box::new(|input| {
                        $solution(input)
                    })),
                )+
            ];

            let mut total_passed = 0;

            for test_case in $test_cases {
                let mut set_results = Vec::new();

                // Получаем результаты от всех решений и преобразуем в HashSet
                for (name, sol) in &solutions {
                    let result = test_case.execute(|input| sol(input));
                    let set: HashSet<_> = result.into_iter().collect();
                    set_results.push((name, set));
                }

                // Проверяем, что все наборы одинаковы
                let first_set = &set_results[0].1;
                for (i, (name, set)) in set_results.iter().enumerate() {
                    assert_eq!(
                        set, first_set,
                        "Solution {} differs from {} for input {:?} (set comparison)",
                        name, set_results[0].0, test_case.input
                    );
                }

                // Проверяем, что результат правильный (сравниваем с expected как HashSet)
                let expected_set: HashSet<_> = test_case.expected.iter().cloned().collect();
                assert_eq!(
                    first_set, &expected_set,
                    "All solutions agree but wrong for input {:?}",
                    test_case.input
                );

                total_passed += 1;
            }

            println!("✅ All {} solutions passed {} test cases (set comparison)",
                     solutions.len(), total_passed);
        }
    };
}
