mod bench;
mod macros;

pub use bench::{
    analyze_benchmark_results, bench_config, benchmark, compare_solutions, compare_solutions_boxed,
    generators, print_comparison_table, BenchmarkResult, SolutionTester, TestCase,
};
