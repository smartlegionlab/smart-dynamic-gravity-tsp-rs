# smart-dynamic-gravity-tsp-rs <sup>v0.1.0</sup>

[![Crates.io](https://img.shields.io/crates/v/smart-dynamic-gravity-tsp)](https://crates.io/crates/smart-dynamic-gravity-tsp)
[![Documentation](https://docs.rs/smart-dynamic-gravity-tsp/badge.svg)](https://docs.rs/smart-dynamic-gravity-tsp)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/smartlegionlab/smart-dynamic-gravity-tsp-rs)](https://github.com/smartlegionlab/smart-dynamic-gravity-tsp-rs)
[![GitHub forks](https://img.shields.io/github/forks/smartlegionlab/smart-dynamic-gravity-tsp-rs?style=social)](https://github.com/smartlegionlab/smart-dynamic-gravity-tsp-rs/network/members)

A high-performance Rust library for solving the Traveling Salesman Problem (TSP) using the novel **Dynamic Gravity** algorithm.

## Features

- **Dynamic Gravity Algorithm**: Physics-inspired heuristic with inertia and angle penalty
- **Near-optimal solutions**: ~3.0% better than classical greedy on 1000 cities
- **Fast execution**: ~0.082s for 1000 cities (110x faster than greedy)
- **2-opt optimization**: Optional post-processing for improved solutions
- **Simple API**: Easy to integrate into your projects
- **No dependencies**: Only uses Rust standard library

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
smart-dynamic-gravity-tsp = "0.1"
```

## Quick Start

```rust
use smart_dynamic_gravity_tsp::{City, dynamic_gravity_solve};

let cities = vec![
    City { x: 0.0, y: 0.0 },
    City { x: 1.0, y: 0.0 },
    City { x: 0.0, y: 1.0 },
    City { x: 1.0, y: 1.0 },
];

let (distance, path) = dynamic_gravity_solve(
    &cities,
    0.9,    // inertia coefficient (0.0-1.0)
    true,   // enable 2-opt optimization
    100,    // 2-opt iterations
    0.3,    // angle penalty weight (0.0-1.0)
    true,   // use angle penalty
    50,     // 2-opt search window
);

println!("Distance: {:.2}", distance);
println!("Path: {:?}", path);
```

## Algorithm Details

### Dynamic Gravity

A novel physics-inspired heuristic that simulates attraction and inertia:

- **Inertia mechanism**: Maintains directional memory for smoother routes
- **Angle penalty**: Penalizes sharp turns for more natural paths
- **2-opt optimization**: Post-processing for improved solutions

### Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `delta` | f64 | 0.9 | Inertia coefficient (higher = more inertia) |
| `post_optimize` | bool | true | Enable 2-opt optimization |
| `max_2opt_iter` | usize | 100 | Maximum 2-opt iterations |
| `angle_penalty_weight` | f64 | 0.3 | Penalty for sharp turns |
| `use_angle_penalty` | bool | true | Enable/disable angle penalty |
| `_2opt_window` | usize | 50 | Search window for 2-opt |

## Performance

### Benchmark: 1000 Cities (Random Distribution)

```
========================================================================
DYNAMIC GRAVITY vs GREEDY TSP COMPARISON
========================================================================

CONFIGURATION:
  Cities:            1000
  Seed phrase:       'SmartLegionLab_PCH_2026'

[1/2] GREEDY ALGORITHM (BASELINE)
   Distance: 13695.34
   Time: 9.177s
   Path GREEDY: 776 -> 688 -> 337 -> 364 -> 356 -> ... -> 831 -> 5 -> 765 -> 241 -> 776 (1001 total, closed)

[2/2] DYNAMIC GRAVITY ALGORITHM
   Parameters:
     Delta (inertia):     0.9
     Angle penalty:       0.3
     2-opt window:        50
     Post-optimize:       true
     2-opt iterations:    100
   Distance: 13291.14
   Time: 0.083s
   Path DYNAMIC GRAVITY: 70 -> 918 -> 745 -> 213 -> 785 -> ... -> 133 -> 420 -> 680 -> 183 -> 70 (1001 total, closed)

========================================================================
COMPARISON
========================================================================
   Greedy:            13695.34
   Dynamic Gravity:   13291.14

   Improvement: 404.20 (3.0%)
   Speed: 9.177s vs 0.083s (110.18988660022674x)

   ✅ DYNAMIC GRAVITY WINS!
========================================================================
```

| Algorithm | Distance | Time (1000 cities) | Speedup vs Greedy | Complexity |
|-----------|----------|-------------------|-------------------|------------|
| **Dynamic Gravity** | **13,291.14** | **0.082s** | **110x faster** 🚀 | O(n²) |
| Greedy (baseline) | 13,695.34 | 9.107s | 1x (baseline) | O(n²) |

**Key insight:** Both algorithms have **O(n²)** complexity, but Dynamic Gravity is **110x faster** while also producing **better quality solutions** (3.0% improvement). This is achieved through:
- Avoiding expensive `sqrt()` operations in the main loop
- Smarter heuristics that build better routes from the start
- Efficient 2-opt optimization with limited search window

## Examples

Run the basic example:
```bash
cargo run --example basic
```

Output:
```
========================================================================
DYNAMIC GRAVITY TSP SOLVER
========================================================================

CONFIGURATION:
  Cities:            1000
  Seed phrase:       'SmartLegionLab_PCH_2026'
  Parameters:
    Delta (inertia):     0.9
    Angle penalty:       0.3
    2-opt window:        50
    Post-optimize:       true
    2-opt iterations:    100

Generating 1000 cities...

Running Dynamic Gravity algorithm...

RESULTS:
  Distance: 13291.14
  Time: 0.082s
  Cities: 1000
   Path DYNAMIC GRAVITY: 70 -> 918 -> 745 -> 213 -> 785 -> ... -> 133 -> 420 -> 680 -> 183 -> 70 (1001 total, closed)

========================================================================
✅ Dynamic Gravity solved 1000 cities in 0.082s
   Distance: 13291.14
========================================================================
```

Run comparison with greedy:
```bash
cargo run --example compare
```

Run benchmarks:
```bash
cargo bench
```

## Ecosystem

This library is part of the [**NP Problem Ecosystem**](https://smartlegionlab.com/ecosystems.html) - a comprehensive suite of exact and heuristic solvers for the Traveling Salesman Problem:

| Project | Description | Language |
|---------|-------------|----------|
| **[Exact TSP Solver](https://github.com/smartlegionlab/exact-tsp-solver)** | High-performance exact solver using Branch and Bound | Go |
| **[Smart TSP Oracle](https://github.com/smartlegionlab/smart-tsp-oracle)** | Exact solver with adaptive thresholding | Python |
| **[Smart TSP Solver](https://github.com/smartlegionlab/smart-tsp-solver)** | Heuristic solver with Angular-Radial & Dynamic Gravity | Python |
| **[Smart TSP Benchmark](https://github.com/smartlegionlab/smart-tsp-benchmark)** | Professional testing infrastructure | Python |
| **smart-dynamic-gravity-tsp** | High-performance Rust library for TSP | Rust |

All projects are grounded in the [**Position-Candidate-Hypothesis (PCH)**](https://smartlegionlab.com/research.html) paradigm for NP-complete problems.

---

## Development

```bash
# Clone repository
git clone https://github.com/smartlegionlab/smart-dynamic-gravity-tsp-rs
cd smart-dynamic-gravity-tsp-rs

# Build
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic
cargo run --example compare

# Run benchmarks
cargo bench

# Build documentation
cargo doc --open
```

## Publishing

```bash
# Login to crates.io (one time)
cargo login

# Publish new version
# 1. Update version in Cargo.toml
# 2. Run:
cargo publish

# Create and push git tag
git tag v0.1.0
git push origin v0.1.0
```

---

## License

[BSD 3-Clause License](https://github.com/smartlegionlab/smart-dynamic-gravity-tsp-rs/blob/master/LICENSE)

Copyright © 2026, [Alexander Suvorov](https://github.com/smartlegionlab)

## Author

**Alexander Suvorov**

- GitHub: [smartlegionlab](https://github.com/smartlegionlab)
- Website: [smartlegionlab.com](https://smartlegionlab.com)

---

## ⚠️ Disclaimer

**By using this software, you agree to the full disclaimer terms.**

**Summary:** Software provided "AS IS" without warranty. You assume all risks.

**Full legal disclaimer:** See [DISCLAIMER.md](https://github.com/smartlegionlab/smart-dynamic-gravity-tsp-rs/blob/master/DISCLAIMER.md)
