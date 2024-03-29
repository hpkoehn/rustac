extern crate serde;
extern crate math;

use serde::{Serialize, Deserialize};
use crate::gamestate::LocationVec;
use crate::UPDATES_PER_SECOND;

// the default movement speed (fields per second)
pub const DEFAULT_SPEED: f64 = 5f64;
// number of decimal digits for rounding
const PRECISION: f64 = 0.0000001;

#[derive(Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

/// Describes the intent of an entity to move to a certain location
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MoveIntent {
    Vector(LocationVec, f64),
    Position(LocationVec, f64)
}

impl MoveIntent {

    // move towards goal and give new Location vec, will change if Vector MoveInten
    // param location: current location
    // return: new location

    /// Lets the `MoveIntent` progress one step towards the target Location
    /// One step is the speed of the `MoveIntent` divided by 'game updates per second'.
    /// This means a `MoveIntent` will progress speed amount of fields after 'game updates per second' times.
    /// This function should be called for each MoveIntent after each game update, resulting in
    /// speed amount of fields per second.
    /// 
    /// It is possible to force_move the entity belonging to the `MoveIntent` to a different place while an `MoveIntent`
    /// is inplace, but depending on the type of `MoveIntent` this might result in weird egde cases.
    /// 
    /// ### Arguments
    /// `location` - The current location from which the `MoveIntent` shall make its move. Normally this is the location
    /// of the entity this intent is attached to
    /// 
    /// ### Returns
    /// The new location after moving a step
    /// 
    pub fn move_from(&mut self, location: &LocationVec) -> LocationVec {
        match self {
            MoveIntent::Position(target_location, speed) => {
                // map this to MoveIntent::Vector
                let distance_vector = LocationVec {
                    x: target_location.x - location.x,
                    y: target_location.y - location.y
                };
                let mut dummy_move_intent = MoveIntent::Vector(distance_vector, *speed);
                dummy_move_intent.move_from(location)
            },
            MoveIntent::Vector(movement_vec, speed) => {
                let step = *speed / UPDATES_PER_SECOND as f64;

                // get the direction of axis we will move towards
                let x_direction = if movement_vec.x == 0.0 { 0.0 } else {movement_vec.x / movement_vec.x.abs()};
                let y_direction = if movement_vec.y == 0.0 { 0.0 } else {movement_vec.y / movement_vec.y.abs()};

                // amount we will move in said direction
                let mut x_step = movement_vec.x.abs() / (movement_vec.x.abs() + movement_vec.y.abs()) * step;
                let mut y_step = movement_vec.y.abs() / (movement_vec.x.abs() + movement_vec.y.abs()) * step;

                // do not move to far, if a step is bigger than distance left
                x_step = x_step.min(movement_vec.x.abs());
                y_step = y_step.min(movement_vec.y.abs());

                // check if the rest of the path is shorten than the precision
                let mut new_intent_x = movement_vec.x - x_step * x_direction;
                let mut new_intent_y = movement_vec.y - y_step * y_direction;

                if new_intent_x.abs() < PRECISION {
                    new_intent_x = 0f64;
                }

                if new_intent_y.abs() < PRECISION {
                    new_intent_y = 0f64
                }

                *self = MoveIntent::Vector(LocationVec {
                    x: new_intent_x,
                    y: new_intent_y,
                }, *speed);


                // check if truncating is required
                // (required to fight unprecision of f64s)
                let mut new_x = location.x + x_step * x_direction;
                if new_x.abs() - new_x.trunc().abs() < PRECISION && new_x > new_x.trunc() {
                    new_x = new_x.round();
                } else if new_x.abs() - (new_x + PRECISION).trunc().abs() < PRECISION{
                    new_x = new_x.round();
                }

                let mut new_y = location.y + y_step * y_direction;
                if new_y.abs() - new_y.trunc().abs() < PRECISION && new_y > new_y.trunc() {
                    new_y = new_y.round();
                } else if new_y.abs() - (new_y + PRECISION).trunc().abs() < PRECISION{
                    new_y = new_y.round();
                }

                // return new location
                LocationVec {
                    x: new_x,
                    y: new_y
                }
            }
        }
    }

    /// Tests if the `MoveIntent` has reached its goal being at a certain location
    /// 
    /// ### Arguments
    /// * current_location - The current location to test from
    /// 
    /// ### Returns
    /// True if the `MoveIntent` has reacheds its goal, else false
    /// 
    pub fn has_arrived(&self, current_location: &LocationVec) -> bool {
        match self {
            MoveIntent::Position(target_location, _) => {
                current_location == target_location
            },
            MoveIntent::Vector(movement_vec, _) => {
                movement_vec.x == 0.0 && movement_vec.y == 0.0
            }
        }
    }

    /// Computes the location the MovementIntent will go towards from a given location
    /// 
    /// ### Arguments
    /// * current_location - The current location to compute from
    /// 
    /// ### Returns
    /// The target location
    /// 
    pub fn target_goal(&self, current_location: &LocationVec) -> LocationVec {
        match self {
            MoveIntent::Position(target_location, _) => *target_location,
            MoveIntent::Vector(movement_vec, _) => LocationVec {
                x: current_location.x + movement_vec.x,
                y: current_location.y + movement_vec.y
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LocationVec, UPDATES_PER_SECOND, MoveIntent};
    const ZERO_VEC: LocationVec = LocationVec {x: 0.0, y: 0.0};

    #[test]
    fn arrived_vector() {
        assert!(MoveIntent::Vector(ZERO_VEC, 1.0).has_arrived(&ZERO_VEC));
        assert!(!MoveIntent::Vector(LocationVec{x:1.0, y:0.0}, 1.0).has_arrived(&ZERO_VEC));
    }

    #[test]
    fn arrived_position() {
        assert!(MoveIntent::Position(ZERO_VEC, 1.0).has_arrived(&ZERO_VEC));
        assert!(MoveIntent::Position(LocationVec{x:2.0, y:1.0}, 1.0).has_arrived(&LocationVec{x:2.0, y:1.0}));
        assert!(!MoveIntent::Position(LocationVec{x:1.0, y:2.0}, 1.0).has_arrived(&LocationVec{x:2.0,y:1.0}));
    }

    #[test]
    fn move_vector() {
        let mut intent = MoveIntent::Vector(LocationVec{x:1.0, y: 0.0}, 1.0);
        let mut location = ZERO_VEC;
        for _ in 0..UPDATES_PER_SECOND {
            location = intent.move_from(&location);
            print!("\n{:?}\n", &location);
            print!("{:?}\n\n", &intent);
        }
        print!("{:?}", &location);
        assert!(intent.has_arrived(&location));
    }
}