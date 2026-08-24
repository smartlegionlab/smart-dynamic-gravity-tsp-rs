//! Core data structures and utilities.
//!
//! This module provides the `City` struct and distance calculation functions.

pub mod city;
pub mod distance;

pub use city::City;
pub use distance::{calculate_cycle_distance, compute_dist_matrix};
