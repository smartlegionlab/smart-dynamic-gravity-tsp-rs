use smart_dynamic_gravity_tsp::{City, dynamic_gravity_solve, greedy_solve};

#[test]
fn test_small_problem() {
    let cities = vec![
        City { x: 0.0, y: 0.0 },
        City { x: 1.0, y: 0.0 },
        City { x: 0.0, y: 1.0 },
        City { x: 1.0, y: 1.0 },
    ];

    let (distance, path) = dynamic_gravity_solve(&cities, 0.9, true, 100, 0.3, true, 50);

    assert!(distance > 0.0);
    assert!(path.len() == cities.len() + 1);
    assert!(path[0] == path[path.len() - 1]);
    let mut visited = vec![false; cities.len()];
    for &city in &path[..path.len() - 1] {
        visited[city] = true;
    }
    assert!(visited.iter().all(|&v| v));
}

#[test]
fn test_empty_cities() {
    let cities: Vec<City> = vec![];
    let (distance, path) = dynamic_gravity_solve(&cities, 0.9, true, 100, 0.3, true, 50);
    assert_eq!(distance, 0.0);
    assert!(path.is_empty());
}

#[test]
fn test_single_city() {
    let cities = vec![City { x: 0.0, y: 0.0 }];
    let (distance, path) = dynamic_gravity_solve(&cities, 0.9, true, 100, 0.3, true, 50);
    assert_eq!(distance, 0.0);
    assert_eq!(path, vec![0, 0]);
}

#[test]
fn test_two_cities() {
    let cities = vec![City { x: 0.0, y: 0.0 }, City { x: 1.0, y: 0.0 }];
    let (distance, path) = dynamic_gravity_solve(&cities, 0.9, true, 100, 0.3, true, 50);
    assert_eq!(distance, 2.0);
    assert_eq!(path.len(), 3);
    assert_eq!(path[0], path[2]);
}

#[test]
fn test_dynamic_gravity_without_optimization() {
    let cities = vec![
        City { x: 0.0, y: 0.0 },
        City { x: 1.0, y: 0.0 },
        City { x: 0.0, y: 1.0 },
        City { x: 1.0, y: 1.0 },
    ];
    let (distance, path) = dynamic_gravity_solve(&cities, 0.9, false, 0, 0.3, true, 50);
    assert!(distance > 0.0);
    assert!(path.len() == cities.len() + 1);
    assert!(path[0] == path[path.len() - 1]);
}

#[test]
fn test_dynamic_gravity_without_angle_penalty() {
    let cities = vec![
        City { x: 0.0, y: 0.0 },
        City { x: 1.0, y: 0.0 },
        City { x: 0.0, y: 1.0 },
        City { x: 1.0, y: 1.0 },
    ];
    let (distance, path) = dynamic_gravity_solve(&cities, 0.9, true, 100, 0.0, false, 50);
    assert!(distance > 0.0);
    assert!(path.len() == cities.len() + 1);
    assert!(path[0] == path[path.len() - 1]);
}

#[test]
fn test_dynamic_gravity_different_delta() {
    let cities = vec![
        City { x: 0.0, y: 0.0 },
        City { x: 1.0, y: 0.0 },
        City { x: 0.0, y: 1.0 },
        City { x: 1.0, y: 1.0 },
    ];
    let (d1, _) = dynamic_gravity_solve(&cities, 0.1, true, 100, 0.3, true, 50);
    let (d2, _) = dynamic_gravity_solve(&cities, 0.9, true, 100, 0.3, true, 50);
    assert!(d1 > 0.0);
    assert!(d2 > 0.0);
}

#[test]
fn test_greedy_small() {
    let cities = vec![
        City { x: 0.0, y: 0.0 },
        City { x: 1.0, y: 0.0 },
        City { x: 0.0, y: 1.0 },
        City { x: 1.0, y: 1.0 },
    ];
    let (distance, path) = greedy_solve(&cities);
    assert!(distance > 0.0);
    assert!(path.len() == cities.len() + 1);
    assert!(path[0] == path[path.len() - 1]);
}

#[test]
fn test_greedy_empty() {
    let cities: Vec<City> = vec![];
    let (distance, path) = greedy_solve(&cities);
    assert_eq!(distance, 0.0);
    assert!(path.is_empty());
}

#[test]
fn test_greedy_single() {
    let cities = vec![City { x: 0.0, y: 0.0 }];
    let (distance, path) = greedy_solve(&cities);
    assert_eq!(distance, 0.0);
    assert_eq!(path, vec![0, 0]);
}

#[test]
fn test_greedy_vs_dynamic_gravity_consistency() {
    let cities = vec![
        City { x: 0.0, y: 0.0 },
        City { x: 1.0, y: 0.0 },
        City { x: 0.0, y: 1.0 },
        City { x: 1.0, y: 1.0 },
        City { x: 2.0, y: 2.0 },
    ];
    let (greedy_dist, _) = greedy_solve(&cities);
    let (dg_dist, _) = dynamic_gravity_solve(&cities, 0.9, true, 100, 0.3, true, 50);
    assert!(greedy_dist > 0.0);
    assert!(dg_dist > 0.0);
}

#[test]
fn test_large_problem_doesnt_crash() {
    let mut cities = Vec::with_capacity(100);
    for i in 0..100 {
        cities.push(City {
            x: (i % 10) as f64,
            y: (i / 10) as f64,
        });
    }
    let (distance, path) = dynamic_gravity_solve(&cities, 0.9, true, 100, 0.3, true, 50);
    assert!(distance > 0.0);
    assert_eq!(path.len(), cities.len() + 1);
    assert_eq!(path[0], path[path.len() - 1]);
}
