use crate::core::{City, calculate_cycle_distance};

/// Solves TSP using the classic Greedy (nearest-neighbor) algorithm.
///
/// This algorithm starts from each city and repeatedly visits the nearest unvisited city.
/// The best route among all starting points is returned.
///
/// This algorithm serves as a baseline for comparison with Dynamic Gravity.
///
/// # Complexity
/// * Time: O(n²) where n is the number of cities
/// * Space: O(n)
///
/// # Arguments
/// * `cities` - Slice of cities with x,y coordinates
///
/// # Returns
/// * `(f64, Vec<usize>)` - (Total distance, Path as city indices)
///
/// # Example
/// ```
/// use smart_dynamic_gravity_tsp::{City, greedy_solve};
///
/// let cities = vec![
///     City { x: 0.0, y: 0.0 },
///     City { x: 1.0, y: 0.0 },
///     City { x: 0.0, y: 1.0 },
/// ];
///
/// let (distance, path) = greedy_solve(&cities);
/// println!("Distance: {:.2}", distance);
/// ```
pub fn greedy_solve(cities: &[City]) -> (f64, Vec<usize>) {
    let n = cities.len();
    if n == 0 {
        return (0.0, vec![]);
    }

    let mut best_path = Vec::with_capacity(n + 1);
    let mut best_distance = f64::MAX;

    for start in 0..n {
        let mut path = Vec::with_capacity(n + 1);
        let mut visited = vec![false; n];
        let mut current = start;
        path.push(current);
        visited[current] = true;

        for _ in 0..n - 1 {
            let mut next_city = 0;
            let mut min_dist = f64::MAX;

            for candidate in 0..n {
                if !visited[candidate] {
                    let dx = cities[current].x - cities[candidate].x;
                    let dy = cities[current].y - cities[candidate].y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < min_dist {
                        min_dist = dist;
                        next_city = candidate;
                    }
                }
            }

            visited[next_city] = true;
            path.push(next_city);
            current = next_city;
        }

        path.push(path[0]);

        let dist = calculate_cycle_distance(&path, cities);

        if dist < best_distance {
            best_distance = dist;
            best_path = path.clone();
        }
    }

    (best_distance, best_path)
}
