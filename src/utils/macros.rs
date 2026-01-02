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

/// Проверка всех решений для вектора тест-кейсов
#[macro_export]
macro_rules! test_all_solutions_for_cases {
    ($test_cases:expr, $($solution:expr),+) => {
        {
            let solutions = vec![$( $solution ),+];
            let solution_names = vec![$( stringify!($solution) ),+];

            let mut total_passed = 0;

            for test_case in $test_cases {
                let mut results = Vec::new();

                for solution in &solutions {
                    results.push(test_case.execute(solution));
                }

                // Проверяем, что все результаты одинаковы
                let first_result = &results[0];
                for (i, result) in results.iter().enumerate() {
                    assert_eq!(
                        result, first_result,
                        "Solution {} differs from {} for input {:?}",
                        solution_names[i], solution_names[0], test_case.input
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
/// Макрос для быстрого создания бенчмарка
#[macro_export]
macro_rules! create_benchmark {
    ($solutions:expr, $input:expr, $iterations:expr) => {{
        use crate::utils::bench;

        let boxed_solutions: Vec<(&str, Box<dyn Fn(_) -> _>)> = $solutions
            .into_iter()
            .map(|(name, sol)| (name, Box::new(sol) as Box<dyn Fn(_) -> _>))
            .collect();

        bench::compare_solutions_boxed(boxed_solutions, $input, $iterations)
    }};

    ($solutions:expr, $input:expr) => {{
        create_benchmark!($solutions, $input, 100_000)
    }};
}

/// Макрос для быстрого создания тест-кейса
#[macro_export]
macro_rules! test_case {
    ($input:expr, $expected:expr) => {
        crate::utils::bench::TestCase::new($input, $expected)
    };
}
