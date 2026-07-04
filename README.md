# Two Sets (CSES) — Algorithm & Benchmark Analysis

## Problem

Given `n`, determine whether numbers from `1..n` can be split into two subsets with equal sum.

If total sum is odd → impossible.

Otherwise, we check if a valid subset exists.

---

## Project Structure

```text
two_sets
│
├── src
│   ├── fast_solution.rs
│   ├── slow_solution.rs
│
├── benches
│   └── benchmark.rs
│
├── tests
│   └── two_sets_tests.rs
│
├── Cargo.toml
└── README.md

Algorithm Overview

Two implementations were created to solve the same problem in different ways.

Fast Solution (Greedy / Optimal):
This approach first computes the total sum and then uses a greedy backward selection strategy to reach the target sum. It processes values efficiently and stops early once the target is achieved. This makes it significantly more efficient in practice.

Slow Solution (Brute Force Simulation):
This approach builds the full list of numbers and uses nested loops with repeated scanning. It simulates subset selection in a brute-force manner, checking many combinations unnecessarily.

Benchmark Interpretation

Fast solution uses a greedy subtraction approach and may terminate early depending on input.

Slow solution repeatedly scans the array and simulates subset search without optimization.

This shows how two different algorithmic strategies can solve the same problem with very different runtime behavior.

Complexity Analysis

Fast Solution:
Time Complexity: O(n)
Space Complexity: O(1)

Slow Solution:
Time Complexity: O(n²)
Space Complexity: O(n)

Tests

The project includes multiple tests to ensure correctness and consistency:

- correctness tests for known inputs
- consistency checks between fast and slow implementations
- small-case validation for edge cases

All tests confirm that both implementations return identical results.

Conclusion

This project demonstrates the difference between an optimized greedy approach and a brute-force simulation approach. Both produce correct results, but the fast solution is significantly more efficient in practice.