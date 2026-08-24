use crate::core::{City, calculate_cycle_distance, compute_dist_matrix};

fn fast_2opt(
    route: &[usize],
    dist_matrix: &[Vec<f64>],
    max_iter: usize,
    window: usize,
) -> Vec<usize> {
    let n = route.len();
    let mut best_route = route.to_vec();
    let mut improved = true;
    let mut iteration = 0;

    while improved && iteration < max_iter {
        improved = false;
        for i in 1..n - 2 {
            let a = best_route[i - 1];
            let b = best_route[i];
            let ab = dist_matrix[a][b];

            let max_j = (i + window).min(n - 1);
            for j in i + 1..max_j {
                let c = best_route[j];
                let d = best_route[j + 1];
                let current = ab + dist_matrix[c][d];
                let proposed = dist_matrix[a][c] + dist_matrix[b][d];

                if proposed < current - 1e-6 {
                    best_route[i..=j].reverse();
                    improved = true;
                    break;
                }
            }
            if improved {
                break;
            }
        }
        iteration += 1;
    }

    best_route
}

/// Solves TSP using the Dynamic Gravity algorithm.
///
/// The algorithm simulates a particle moving through the city space with inertia.
/// It maintains directional memory (last vector) and penalizes sharp turns,
/// resulting in smoother, more natural routes.
///
/// # Algorithm Steps
/// 1. Find the city closest to the geometric center as starting point
/// 2. For each step, select the nearest unvisited city
/// 3. Apply angle penalty to discourage sharp turns
/// 4. Update position with inertia (delta parameter)
/// 5. Optionally apply 2-opt optimization to improve the route
///
/// # Arguments
/// * `cities` - Slice of cities with x,y coordinates
/// * `delta` - Inertia coefficient (0.0-1.0). Higher = more inertia
/// * `post_optimize` - Whether to apply 2-opt optimization
/// * `max_2opt_iter` - Maximum iterations for 2-opt
/// * `angle_penalty_weight` - Weight for angle penalty (0.0-1.0)
/// * `use_angle_penalty` - Enable/disable angle penalty
/// * `_2opt_window` - Search window for 2-opt
///
/// # Returns
/// * `(f64, Vec<usize>)` - (Total distance, Path as city indices)
///
/// # Example
/// ```
/// use smart_dynamic_gravity_tsp::{City, dynamic_gravity_solve};
///
/// let cities = vec![
///     City { x: 0.0, y: 0.0 },
///     City { x: 1.0, y: 0.0 },
///     City { x: 0.0, y: 1.0 },
/// ];
///
/// let (distance, path) = dynamic_gravity_solve(
///     &cities,
///     0.9,    // delta
///     true,   // post_optimize
///     100,    // max_2opt_iter
///     0.3,    // angle_penalty_weight
///     true,   // use_angle_penalty
///     50,     // _2opt_window
/// );
///
/// println!("Distance: {:.2}", distance);
/// ```
pub fn dynamic_gravity_solve(
    cities: &[City],
    delta: f64,
    post_optimize: bool,
    max_2opt_iter: usize,
    angle_penalty_weight: f64,
    use_angle_penalty: bool,
    _2opt_window: usize,
) -> (f64, Vec<usize>) {
    let n = cities.len();
    if n == 0 {
        return (0.0, vec![]);
    }

    let center_x = cities.iter().map(|c| c.x).sum::<f64>() / n as f64;
    let center_y = cities.iter().map(|c| c.y).sum::<f64>() / n as f64;

    let mut start_idx = 0;
    let mut min_dist_to_center = f64::MAX;
    for (i, city) in cities.iter().enumerate() {
        let dx = city.x - center_x;
        let dy = city.y - center_y;
        let dist = dx * dx + dy * dy;
        if dist < min_dist_to_center {
            min_dist_to_center = dist;
            start_idx = i;
        }
    }

    let mut route = Vec::with_capacity(n + 1);
    let mut remaining = vec![true; n];
    let mut current_pos = (center_x, center_y);
    let mut last_vector = (0.0, 0.0);

    route.push(start_idx);
    remaining[start_idx] = false;

    for i in 1..n {
        let mut min_score = f64::MAX;
        let mut nearest = 0;

        for j in 0..n {
            if remaining[j] {
                let dx = cities[j].x - current_pos.0;
                let dy = cities[j].y - current_pos.1;
                let mut dist_sq = dx * dx + dy * dy;

                if use_angle_penalty && i > 1 {
                    let dot_product = dx * last_vector.0 + dy * last_vector.1;
                    let last_norm_sq =
                        last_vector.0 * last_vector.0 + last_vector.1 * last_vector.1;

                    if last_norm_sq > 0.0 {
                        let norm = (dist_sq * last_norm_sq).sqrt();
                        if norm > 0.0 {
                            let cos_angle = dot_product / norm;
                            let angle_penalty = angle_penalty_weight * (1.0 - cos_angle);
                            dist_sq *= 1.0 + angle_penalty;
                        }
                    }
                }

                if dist_sq < min_score {
                    min_score = dist_sq;
                    nearest = j;
                }
            }
        }

        route.push(nearest);
        remaining[nearest] = false;

        last_vector = (
            cities[nearest].x - current_pos.0,
            cities[nearest].y - current_pos.1,
        );
        current_pos = (
            cities[nearest].x * delta + current_pos.0 * (1.0 - delta),
            cities[nearest].y * delta + current_pos.1 * (1.0 - delta),
        );
    }

    route.push(route[0]);

    if post_optimize {
        let dist_matrix = compute_dist_matrix(cities);
        route = fast_2opt(&route, &dist_matrix, max_2opt_iter, _2opt_window);
    }

    let distance = calculate_cycle_distance(&route, cities);
    (distance, route)
}
