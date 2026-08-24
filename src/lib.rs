//! High-performance TSP solver library featuring the Dynamic Gravity algorithm.
//!
//! The Dynamic Gravity algorithm is a novel physics-inspired heuristic that simulates
//! attraction and inertia to find near-optimal solutions for the Traveling Salesman Problem.
//!
//! # Example
//! ```
//! use smart_dynamic_gravity_tsp::{City, dynamic_gravity_solve};
//!
//! let cities = vec![
//!     City { x: 0.0, y: 0.0 },
//!     City { x: 1.0, y: 0.0 },
//!     City { x: 0.0, y: 1.0 },
//! ];
//!
//! let (distance, path) = dynamic_gravity_solve(
//!     &cities,
//!     0.9,    // inertia coefficient
//!     true,   // enable 2-opt optimization
//!     100,    // 2-opt iterations
//!     0.3,    // angle penalty weight
//!     true,   // use angle penalty
//!     50,     // 2-opt search window
//! );
//!
//! println!("Distance: {:.2}", distance);
//! ```

pub mod algorithms;
pub mod core;

pub use algorithms::dynamic_gravity_solve;
pub use core::City;
pub use core::distance::{calculate_cycle_distance, compute_dist_matrix};

#[doc(hidden)]
pub use algorithms::greedy_solve;
