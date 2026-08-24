use smart_dynamic_gravity_tsp::{City, dynamic_gravity_solve};
use std::time::Instant;

fn generate_cities(seed_phrase: &str, count: usize) -> Vec<City> {
    let mut map_seed = 2166136261u64;
    for byte in seed_phrase.bytes() {
        map_seed = (map_seed ^ byte as u64).wrapping_mul(1099511628211);
    }
    let mut map_rand = move || {
        map_seed =
            (map_seed.wrapping_mul(6364136223846793005).wrapping_add(1)) & 0x7fffffffffffffff;
        map_seed
    };

    let mut cities = Vec::with_capacity(count);

    for _ in 0..count {
        let rx = (map_rand() % 500) as f64;
        let ry = (map_rand() % 500) as f64;
        cities.push(City { x: rx, y: ry });
    }

    cities
}

fn print_path_preview(path: &[usize], name: &str) {
    let total = path.len();
    let first_n = 5;
    let last_n = 5;

    print!("   Path {}: ", name);

    if total <= 10 {
        for i in 0..total {
            print!("{}", path[i]);
            if i < total - 1 {
                print!(" -> ");
            }
        }
        println!(" ({} total, closed)", total);
        return;
    }

    for i in 0..first_n {
        print!("{}", path[i]);
        if i < first_n - 1 {
            print!(" -> ");
        }
    }

    print!(" -> ... -> ");

    for i in (total - last_n)..total {
        print!("{}", path[i]);
        if i < total - 1 {
            print!(" -> ");
        }
    }

    println!(" ({} total, closed)", total);
}

fn main() {
    println!("========================================================================");
    println!("DYNAMIC GRAVITY TSP SOLVER");
    println!("========================================================================");

    let seed_phrase = "SmartLegionLab_PCH_2026";
    let num_cities = 1000;

    println!("\nCONFIGURATION:");
    println!("  Cities:            {}", num_cities);
    println!("  Seed phrase:       '{}'", seed_phrase);
    println!("  Parameters:");
    println!("    Delta (inertia):     0.9");
    println!("    Angle penalty:       0.3");
    println!("    2-opt window:        50");
    println!("    Post-optimize:       true");
    println!("    2-opt iterations:    100");

    println!("\nGenerating {} cities...", num_cities);
    let cities = generate_cities(seed_phrase, num_cities);

    println!("\nRunning Dynamic Gravity algorithm...");
    let start = Instant::now();
    let (distance, path) = dynamic_gravity_solve(
        &cities, 0.9,  // delta
        true, // post_optimize
        100,  // max_2opt_iter
        0.3,  // angle_penalty_weight
        true, // use_angle_penalty
        50,   // _2opt_window
    );
    let time = start.elapsed().as_secs_f64();

    println!("\nRESULTS:");
    println!("  Distance: {:.2}", distance);
    println!("  Time: {:.3}s", time);
    println!("  Cities: {}", path.len() - 1);
    print_path_preview(&path, "DYNAMIC GRAVITY");

    println!("\n========================================================================");
    println!(
        "✅ Dynamic Gravity solved {} cities in {:.3}s",
        num_cities, time
    );
    println!("   Distance: {:.2}", distance);
    println!("========================================================================");
}
