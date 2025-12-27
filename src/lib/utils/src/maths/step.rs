/// Generates a series of points between two 3D vectors (`start` and `end`) at a fixed step distance.
///
/// # Arguments
///
/// * `start` - A `Vec3A` representing the starting point of the line.
/// * `end` - A `Vec3A` representing the ending point of the line.
/// * `step` - A `f32` value representing the distance between each generated point.
///
/// # Returns
///
/// A `Vec<Vec3A>` containing the points between `start` and `end`, including the `end` point.
///
/// # Example
///
/// ```
/// use bevy_math::Vec3A;
/// let start = Vec3A::new(0.0, 0.0, 0.0);
/// let end = Vec3A::new(1.0, 1.0, 1.0);
/// let step = 0.5;
/// let points = step_between(start, end, step);
/// assert_eq!(points.len(), 3);
/// ```
pub fn step_between(
    start: bevy_math::Vec3A,
    end: bevy_math::Vec3A,
    step: f32,
) -> Vec<bevy_math::Vec3A> {
    let mut points = Vec::new();
    let direction = end - start; // Calculate the direction vector from start to end.
    let distance = direction.length(); // Calculate the total distance between start and end.
    let direction_normalized = direction / distance; // Normalize the direction vector.

    let mut current_distance = 0.0;
    while current_distance < distance {
        // Calculate the next point along the direction vector.
        let point = start + direction_normalized * current_distance;
        points.push(point); // Add the point to the list.
        current_distance += step; // Increment the distance by the step size.
    }
    points.push(end); // Ensure the end point is included in the list.
    points
}
