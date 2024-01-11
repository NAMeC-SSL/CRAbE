use crabe_math::shape::{Circle, Line, Rectangle};
use nalgebra::Point2;
use serde::Serialize;

mod goal;
pub use self::goal::Goal;
mod penalty;
pub use self::penalty::Penalty;

/// The `Field` struct represent the SSL field.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    /// Width of the SSL field in meters.
    pub width: f64,
    /// Length of the SSL field in meters.
    pub length: f64,
}

/// The `Geometry` struct contains all the geometric information of the SSL field.
/// By default, the geometry corresponds to the size of the SSL Division B field.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    /// The width of the boundary around the field in meters.
    pub boundary_width: f64,
    /// The dimensions of the field.
    pub field: Field,
    /// The position and size of the ally goal.
    pub ally_goal: Goal,
    /// The position and size of the enemy goal.
    pub enemy_goal: Goal,
    /// The position and size of the ally penalty area.
    pub ally_penalty: Penalty,
    ///  The position and size of the enemy penalty area.
    pub enemy_penalty: Penalty,
    /// The center circle of the field (position in meters and radius in radian).
    pub center: Circle,
}

impl Default for Geometry {
    fn default() -> Self {
        Self {
            boundary_width: 0.3,
            field: Field {
                length: 9.0,
                width: 6.0,
            },

            ally_goal: Goal::new(
                1.0, 0.18,
                Point2::new(-4.68, -0.5),
                true
            ),
            enemy_goal: Goal::new(
                1.0, 0.18,
            Point2::new(4.68, 0.5),
                false
            ),
            ally_penalty: Penalty {
                area: Rectangle::new(
                    1.0, 2.0, Point2::new(-4.5, -1.0)
                ),
            },
            enemy_penalty: Penalty {
                area: Rectangle::new(
                    1.0, 2.0, Point2::new(4.5, 1.0)
                ),
            },
            center: Circle {
                center: Point2::new(0.0, 0.0),
                radius: 0.5,
            },
        }

    }
}
