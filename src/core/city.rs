/// Represents a city with 2D coordinates.
///
/// # Example
/// ```
/// use smart_dynamic_gravity_tsp::City;
///
/// let city = City { x: 10.0, y: 20.0 };
/// ```
#[derive(Clone, Copy, Debug)]
pub struct City {
    pub x: f64,
    pub y: f64,
}
