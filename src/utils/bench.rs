//! Утилиты для бенчмаркинга и тестирования

use std::time::{Duration, Instant};

// region:    --- Структуры данных

/// Результат бенчмарка
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub iterations: usize,
}

impl BenchmarkResult {
    pub fn new(name: &str, duration: Duration, iterations: usize) -> Self {
        Self {
            name: name.to_string(),
            duration,
            iterations,
        }
    }

    pub fn avg_ns(&self) -> f64 {
        self.duration.as_nanos() as f64 / self.iterations as f64
    }

    pub fn avg_ms(&self) -> f64 {
        self.duration.as_micros() as f64 / 1000.0 / self.iterations as f64
    }

    pub fn ops_per_sec(&self) -> f64 {
        self.iterations as f64 / self.duration.as_secs_f64()
    }
}

/// Универсальная структура для тест-кейсов
#[derive(Debug, Clone)]
pub struct TestCase<I, O>
where
    I: Clone + std::fmt::Debug,
    O: PartialEq + std::fmt::Debug,
{
    pub input: I,
    pub expected: O,
}

impl<I, O> TestCase<I, O>
where
    I: Clone + std::fmt::Debug,
    O: PartialEq + std::fmt::Debug,
{
    pub fn new(input: I, expected: O) -> Self {
        Self { input, expected }
    }

    /// Универсальный метод выполнения - работает с функциями любой арности
    pub fn execute<F, Args>(&self, solution: F) -> O
    where
        F: Fn(Args) -> O,
        I: Into<Args> + Clone,
    {
        solution(self.input.clone().into())
    }

    /// Быстрая проверка решения
    pub fn assert_solution<F, Args>(&self, solution: F, solution_name: &str)
    where
        F: Fn(Args) -> O,
        I: Into<Args> + Clone,
    {
        let result = self.execute(solution);
        assert_eq!(
            result, self.expected,
            "{} failed for input {:?}",
            solution_name, self.input
        );
    }
}

// endregion: --- Структуры данных

// region:    --- Трейты

/// Трейт для удобного тестирования решений
pub trait SolutionTester<I, O>
where
    I: Clone,
    O: PartialEq + std::fmt::Debug,
{
    fn test_inputs(&self) -> Vec<(I, O)>;

    fn run_and_verify<F>(&self, solution: F, solution_name: &str)
    where
        F: Fn(I) -> O,
    {
        for (i, (input, expected)) in self.test_inputs().iter().enumerate() {
            let result = solution(input.clone());
            assert_eq!(
                result, *expected,
                "{} failed test case {}: expected {:?}, got {:?}",
                solution_name, i, expected, result
            );
        }
        println!("✅ {} passed all test cases", solution_name);
    }
}

// Реализация SolutionTester для TestCase
impl<I, O> SolutionTester<I, O> for TestCase<I, O>
where
    I: Clone + std::fmt::Debug,
    O: PartialEq + std::fmt::Debug + Clone,
{
    fn test_inputs(&self) -> Vec<(I, O)> {
        vec![(self.input.clone(), self.expected.clone())]
    }
}

// endregion: --- Трейты

// region:    --- Функции бенчмаркинга

/// Запуск бенчмарка
pub fn benchmark<F, I, O>(name: &str, f: F, input: I, iterations: usize) -> BenchmarkResult
where
    F: Fn(I) -> O,
    I: Clone,
{
    // Прогрев
    for _ in 0..10 {
        let _ = f(input.clone());
    }

    let start = Instant::now();

    for _ in 0..iterations {
        let input_clone = input.clone();
        let _ = f(input_clone);
    }

    let duration = start.elapsed();
    BenchmarkResult::new(name, duration, iterations)
}

/// Сравнение нескольких решений
pub fn compare_solutions<F, I, O>(
    solutions: Vec<(&str, F)>,
    input: &I,
    iterations: usize,
) -> Vec<BenchmarkResult>
where
    F: Fn(I) -> O,
    I: Clone,
{
    solutions
        .into_iter()
        .map(|(name, f)| benchmark(name, f, input.clone(), iterations))
        .collect()
}

/// Сравнение решений с Box (для разных типов замыканий)
pub fn compare_solutions_boxed<I, O>(
    solutions: Vec<(&str, Box<dyn Fn(I) -> O>)>,
    input: I,
    iterations: usize,
) -> Vec<BenchmarkResult>
where
    I: Clone,
{
    solutions
        .into_iter()
        .map(|(name, f)| {
            // Прогрев
            for _ in 0..10 {
                let _ = f(input.clone());
            }

            let start = Instant::now();
            for _ in 0..iterations {
                let input_clone = input.clone();
                let _ = f(input_clone);
            }
            let duration = start.elapsed();

            BenchmarkResult::new(name, duration, iterations)
        })
        .collect()
}

// endregion: --- Функции бенчмаркинга

// region:    --- Вывод результатов

/// Вывод результатов в таблицу
pub fn print_comparison_table(results: &[BenchmarkResult]) {
    println!("\n┌────────────────────────────────────────────┐");
    println!("│          Performance Comparison           │");
    println!("├────────────┬────────────┬─────────────────┤");
    println!("│ Solution   │ Avg Time   │ Operations/sec  │");
    println!("├────────────┼────────────┼─────────────────┤");

    for result in results {
        let time_str = if result.avg_ms() >= 1.0 {
            format!("{:.3} ms", result.avg_ms())
        } else if result.avg_ns() >= 1000.0 {
            format!("{:.1} µs", result.avg_ns() / 1000.0)
        } else {
            format!("{:.0} ns", result.avg_ns())
        };

        println!(
            "│ {:10} │ {:10} │ {:8.0} ops/sec │",
            result.name,
            time_str,
            result.ops_per_sec()
        );
    }

    println!("└────────────┴────────────┴─────────────┘");
}

/// Анализ результатов бенчмарка
pub fn analyze_benchmark_results(results: &[BenchmarkResult]) {
    if results.is_empty() {
        return;
    }

    // Находим самое быстрое решение
    let fastest = results
        .iter()
        .min_by_key(|r| r.duration)
        .expect("Should have at least one result");

    println!("\n📊 Performance Analysis:");
    println!(
        "🏆 Fastest: {} ({:.1} ns/iter)",
        fastest.name,
        fastest.avg_ns()
    );

    // Сравниваем все остальные решения с самым быстрым
    for result in results {
        if result.name != fastest.name {
            let ratio = result.duration.as_nanos() as f64 / fastest.duration.as_nanos() as f64;
            println!(
                "  {} is {:.1}x slower ({:.1} ns/iter)",
                result.name,
                ratio,
                result.avg_ns()
            );
        }
    }

    // Статистика
    if results.len() > 1 {
        let avg_ns: f64 = results.iter().map(|r| r.avg_ns()).sum::<f64>() / results.len() as f64;
        let min_ns = fastest.avg_ns();
        let max_ns = results.iter().map(|r| r.avg_ns()).fold(0.0, f64::max);

        println!("\n📈 Statistics:");
        println!("  Average: {:.1} ns/iter", avg_ns);
        println!(
            "  Range: {:.1} - {:.1} ns/iter ({:.1}x spread)",
            min_ns,
            max_ns,
            max_ns / min_ns
        );
        println!("  Total solutions compared: {}", results.len());
    }
}

// endregion: --- Вывод результатов

// region:    --- Генераторы данных

/// Генераторы тестовых данных
pub mod generators {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    /// Генерация случайного вектора целых чисел
    pub fn random_i32_vec(size: usize, min: i32, max: i32, seed: u64) -> Vec<i32> {
        let mut rng = StdRng::seed_from_u64(seed);
        (0..size).map(|_| rng.random_range(min..=max)).collect()
    }

    /// Генерация отсортированного вектора
    pub fn sorted_i32_vec(size: usize, start: i32, step: i32) -> Vec<i32> {
        (0..size).map(|i| start + i as i32 * step).collect()
    }

    /// Создать вектор с уникальными значениями
    pub fn unique_i32_vec(size: usize, start: i32) -> Vec<i32> {
        (start..start + size as i32).collect()
    }
}

// endregion: --- Генераторы данных

// region:    --- Конфигурации

/// Конфигурация бенчмарков
pub mod bench_config {
    pub const SMALL_ITERATIONS: usize = 100_000;
    pub const MEDIUM_ITERATIONS: usize = 10_000;
    pub const LARGE_ITERATIONS: usize = 1_000;

    /// Стандартный тестовый вход (10 элементов)
    pub fn standard_test_input() -> Vec<i32> {
        vec![2, 7, 11, 15, 19, 22, 25, 30, 35, 40]
    }

    /// Большой тестовый вход (1000 элементов)
    pub fn large_test_input() -> Vec<i32> {
        (1..=1000).collect()
    }
}

// endregion: --- Конфигурации
