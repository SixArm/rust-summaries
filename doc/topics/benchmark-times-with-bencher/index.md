# Benchmark times with Bencher

Benchmarking is the process of measuring the performance of code by running it multiple times under different conditions to identify areas where improvement can be made. It is an essential process for optimizing Rust code for faster execution and better resource utilization.

To conduct Rust benchmarking, the following steps are usually taken:

1. Identify the code or function to be benchmarked

2. Write a benchmarking harness to execute the code multiple times and record metrics such as execution time and memory usage.

3. Run the benchmark multiple times to obtain a baseline performance metric

4. Identify areas of improvement in the code and make changes to optimize its performance

5. Repeat the benchmarking process after making modifications to gauge the impact on performance.

A typical way is:

1. Create a top-level folder named `benches`.

2. Create a typical Rust file with your own function.

4. Annotate the function with the `#[bench]` attribute.

Rust provides built-in support for benchmarking through its libtest framework, and the function annotation `[#bench]`.

As of this writing (2023-03-23) the Rust nightly channel has benchmarking as a feature, whereas the Rust stable channel does not. See below for troubleshooing information about this.

For simple benchmarks, you can use the Bencher crate, which is a simple Rust-stable-compatible benchmark runner. For real-world projects, we suggest the Criterion crate, which is newer, more popular with current Rust teams, and provides more capabilities.
