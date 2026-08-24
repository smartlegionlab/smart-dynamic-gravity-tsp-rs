//! Algorithm implementations for TSP solving.
//!
//! This module provides both the main Dynamic Gravity algorithm
//! and the Greedy baseline for comparison.

pub mod dynamic_gravity;
pub mod greedy;

pub use dynamic_gravity::dynamic_gravity_solve;
pub use greedy::greedy_solve;
