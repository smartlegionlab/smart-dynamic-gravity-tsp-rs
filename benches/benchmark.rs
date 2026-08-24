//! Benchmarks for the Dynamic Gravity TSP solver.
//!
//! Measures performance on different city sizes: 100, 500, 1000, 2000.
//!
//! # Run benchmarks
//! ```bash
//! cargo bench
//! ```
//!
//! # Run specific benchmark
//! ```bash
//! cargo bench -- dynamic_gravity_1000
//! ```

use criterion::{Criterion, criterion_group, criterion_main};
use smart_dynamic_gravity_tsp::{City, dynamic_gravity_solve};

/// Generates random cities with a fixed seed for reproducible benchmarks.
fn generate_cities(count: usize) -> Vec<City> {
    let mut seed = 42u64;
    let mut cities = Vec::with_capacity(count);

    for _ in 0..count {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let x = (seed % 1000) as f64;
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let y = (seed % 1000) as f64;
        cities.push(City { x, y });
    }

    cities
}

/// Benchmarks Dynamic Gravity on different city sizes.
fn benchmark_dynamic_gravity(c: &mut Criterion) {
    let sizes = [100, 500, 1000, 2000];

    for &size in &sizes {
        let cities = generate_cities(size);
        c.bench_function(&format!("dynamic_gravity_{}", size), |b| {
            b.iter(|| {
                let _ = dynamic_gravity_solve(&cities, 0.9, true, 100, 0.3, true, 50);
            })
        });
    }
}

criterion_group!(benches, benchmark_dynamic_gravity);
criterion_main!(benches);
