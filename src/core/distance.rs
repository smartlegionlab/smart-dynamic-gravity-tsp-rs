use super::city::City;

/// Calculates the total distance of a cycle path through cities.
///
/// # Arguments
/// * `path` - Slice of city indices representing the route (must be closed, first == last)
/// * `cities` - Slice of cities with coordinates
///
/// # Returns
/// * Total Euclidean distance of the cycle
///
/// # Example
/// ```
/// use smart_dynamic_gravity_tsp::{City, calculate_cycle_distance};
///
/// let cities = vec![
///     City { x: 0.0, y: 0.0 },
///     City { x: 1.0, y: 0.0 },
/// ];
/// let path = vec![0, 1, 0];
/// let dist = calculate_cycle_distance(&path, &cities);
/// assert_eq!(dist, 2.0);
/// ```
pub fn calculate_cycle_distance(path: &[usize], cities: &[City]) -> f64 {
    if path.is_empty() {
        return f64::MAX;
    }
    let mut total = 0.0;

    for i in 0..path.len() - 1 {
        let c1 = cities[path[i]];
        let c2 = cities[path[i + 1]];
        let dx = c1.x - c2.x;
        let dy = c1.y - c2.y;
        total += (dx * dx + dy * dy).sqrt();
    }
    total
}

/// Computes a symmetric distance matrix for all city pairs.
///
/// # Arguments
/// * `cities` - Slice of cities with coordinates
///
/// # Returns
/// * A 2D vector where `matrix[i][j]` is the Euclidean distance between city i and j
///
/// # Example
/// ```
/// use smart_dynamic_gravity_tsp::{City, compute_dist_matrix};
///
/// let cities = vec![
///     City { x: 0.0, y: 0.0 },
///     City { x: 3.0, y: 4.0 },
/// ];
/// let matrix = compute_dist_matrix(&cities);
/// assert_eq!(matrix[0][1], 5.0);
/// ```
pub fn compute_dist_matrix(cities: &[City]) -> Vec<Vec<f64>> {
    let n = cities.len();
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in i..n {
            let dx = cities[i].x - cities[j].x;
            let dy = cities[i].y - cities[j].y;
            let dist = (dx * dx + dy * dy).sqrt();
            matrix[i][j] = dist;
            matrix[j][i] = dist;
        }
    }
    matrix
}
