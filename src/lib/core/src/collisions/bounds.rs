use typename::TypeName;

#[derive(TypeName)]
pub struct CollisionBounds {
    // Given a start position, where the bounding box starts on the x-axis.
    pub x_offset_start: f64,
    // Given a start position, where the bounding box ends on the x-axis.
    pub x_offset_end: f64,
    // Given a start position, where the bounding box starts on the y-axis.
    pub y_offset_start: f64,
    // Given a start position, where the bounding box ends on the y-axis.
    pub y_offset_end: f64,
    // Given a start position, where the bounding box starts on the z-axis.
    pub z_offset_start: f64,
    // Given a start position, where the bounding box ends on the z-axis.
    pub z_offset_end: f64,
}

impl Default for CollisionBounds {
    fn default() -> Self {
        CollisionBounds {
            x_offset_start: 0.0,
            x_offset_end: 0.0,
            y_offset_start: 0.0,
            y_offset_end: 0.0,
            z_offset_start: 0.0,
            z_offset_end: 0.0,
        }
    }
}

impl CollisionBounds {
    #[inline]
    pub fn collides(
        &self,
        own_pos: (f64, f64, f64),
        other_bounds: &CollisionBounds,
        other_pos: (f64, f64, f64),
    ) -> bool {
        let (own_x, own_y, own_z) = own_pos;
        let (other_x, other_y, other_z) = other_pos;

        // Pre-calculate bounds
        let own_x_start = own_x + self.x_offset_start;
        let own_x_end = own_x + self.x_offset_end;
        let own_y_start = own_y + self.y_offset_start;
        let own_y_end = own_y + self.y_offset_end;
        let own_z_start = own_z + self.z_offset_start;
        let own_z_end = own_z + self.z_offset_end;

        let other_x_start = other_x + other_bounds.x_offset_start;
        let other_x_end = other_x + other_bounds.x_offset_end;
        let other_y_start = other_y + other_bounds.y_offset_start;
        let other_y_end = other_y + other_bounds.y_offset_end;
        let other_z_start = other_z + other_bounds.z_offset_start;
        let other_z_end = other_z + other_bounds.z_offset_end;

        // Check collisions axis by axis
        (own_x_start < other_x_end && own_x_end > other_x_start)
            && (own_y_start < other_y_end && own_y_end > other_y_start)
            && (own_z_start < other_z_end && own_z_end > other_z_start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collides_when_boxes_overlap() {
        let bounds1 = CollisionBounds {
            x_offset_start: 0.0,
            x_offset_end: 2.0,
            y_offset_start: 0.0,
            y_offset_end: 2.0,
            z_offset_start: 0.0,
            z_offset_end: 2.0,
        };
        let bounds2 = CollisionBounds {
            x_offset_start: 1.0,
            x_offset_end: 3.0,
            y_offset_start: 1.0,
            y_offset_end: 3.0,
            z_offset_start: 1.0,
            z_offset_end: 3.0,
        };
        assert!(bounds1.collides((0.0, 0.0, 0.0), &bounds2, (0.0, 0.0, 0.0)));
    }

    #[test]
    fn does_not_collide_when_boxes_do_not_overlap() {
        let bounds1 = CollisionBounds {
            x_offset_start: 0.0,
            x_offset_end: 1.0,
            y_offset_start: 0.0,
            y_offset_end: 1.0,
            z_offset_start: 0.0,
            z_offset_end: 1.0,
        };
        let bounds2 = CollisionBounds {
            x_offset_start: 2.0,
            x_offset_end: 3.0,
            y_offset_start: 2.0,
            y_offset_end: 3.0,
            z_offset_start: 2.0,
            z_offset_end: 3.0,
        };
        assert!(!bounds1.collides((0.0, 0.0, 0.0), &bounds2, (0.0, 0.0, 0.0)));
    }

    #[test]
    fn collides_when_boxes_touch_edges() {
        let bounds1 = CollisionBounds {
            x_offset_start: 0.0,
            x_offset_end: 1.0,
            y_offset_start: 0.0,
            y_offset_end: 1.0,
            z_offset_start: 0.0,
            z_offset_end: 1.0,
        };
        let bounds2 = CollisionBounds {
            x_offset_start: 1.0,
            x_offset_end: 2.0,
            y_offset_start: 1.0,
            y_offset_end: 2.0,
            z_offset_start: 1.0,
            z_offset_end: 2.0,
        };
        assert!(!bounds1.collides((0.0, 0.0, 0.0), &bounds2, (0.0, 0.0, 0.0)));
    }

    #[test]
    fn collides_when_one_box_inside_another() {
        let bounds1 = CollisionBounds {
            x_offset_start: 0.0,
            x_offset_end: 3.0,
            y_offset_start: 0.0,
            y_offset_end: 3.0,
            z_offset_start: 0.0,
            z_offset_end: 3.0,
        };
        let bounds2 = CollisionBounds {
            x_offset_start: 1.0,
            x_offset_end: 2.0,
            y_offset_start: 1.0,
            y_offset_end: 2.0,
            z_offset_start: 1.0,
            z_offset_end: 2.0,
        };
        assert!(bounds1.collides((0.0, 0.0, 0.0), &bounds2, (0.0, 0.0, 0.0)));
    }

    #[test]
    fn does_not_collide_when_positions_are_far_apart() {
        let bounds1 = CollisionBounds {
            x_offset_start: 0.0,
            x_offset_end: 1.0,
            y_offset_start: 0.0,
            y_offset_end: 1.0,
            z_offset_start: 0.0,
            z_offset_end: 1.0,
        };
        let bounds2 = CollisionBounds {
            x_offset_start: 0.0,
            x_offset_end: 1.0,
            y_offset_start: 0.0,
            y_offset_end: 1.0,
            z_offset_start: 0.0,
            z_offset_end: 1.0,
        };
        assert!(!bounds1.collides((0.0, 0.0, 0.0), &bounds2, (10.0, 10.0, 10.0)));
    }
}
