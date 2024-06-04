use crate::ball::Ball;
use crate::point_2d::Point2D;

pub fn calculate_postcollision_positions(mut ball_1: &Ball, mut ball_2: &Ball) -> Option<(Point2D, Point2D)> {
    let vector = ball_1.verlet_object.position_current.vector_to(&ball_2.verlet_object.position_current);
    let min_distance = ball_1.radius + ball_2.radius;

    if vector.length >= min_distance {
        return None;
    }

    let normalized_vector = vector.to_normalized();
    let delta = min_distance - vector.length;
    let half_delta = delta / 2.;

    Some((
        Point2D {
            x: ball_1.verlet_object.position_current.x - normalized_vector.x * half_delta,
            y: ball_1.verlet_object.position_current.y - normalized_vector.y * half_delta
        },
        Point2D {
            x: ball_2.verlet_object.position_current.x + normalized_vector.x * half_delta,
            y: ball_2.verlet_object.position_current.y + normalized_vector.y * half_delta
        }
    ))
}
