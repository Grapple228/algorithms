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
