use crate::Pixel;

/// Represents a unit vector direction in 2D space.
///
/// A `Direction` is always normalized to unit length, with the invariant
/// enforced at construction time. The direction is stored as `(dx, dy)`
/// where `dx² + dy² = 1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Direction {
    /// Normalized horizontal component of the unit vector.
    pub dx: f32,
    /// Normalized vertical component of the unit vector.
    pub dy: f32,
}

impl Direction {
    /// Creates a normalized unit vector from the given direction components.
    ///
    /// The components are automatically normalized to unit length. If both
    /// components are zero, returns `None` (a zero vector cannot be normalized).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let dir = Direction::new(3.0, 4.0)?; // Normalizes to (0.6, 0.8)
    /// assert!(dir.magnitude() - 1.0 < 1e-6);
    /// ```
    pub fn new(dx: f32, dy: f32) -> Option<Self> {
        let magnitude = (dx * dx + dy * dy).sqrt();

        if magnitude == 0.0 {
            return None;
        }

        Some(Self {
            dx: dx / magnitude,
            dy: dy / magnitude,
        })
    }

    /// Creates a unit vector pointing from one pixel to another.
    ///
    /// Calculates the direction from `from` to `to` by computing the difference
    /// and normalizing it. Returns `None` if both pixels are at the same position.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p1 = Pixel::new(0, 0);
    /// let p2 = Pixel::new(3, 4);
    /// let dir = Direction::from_pixels(p1, p2)?; // Normalizes (3, 4) to (0.6, 0.8)
    /// ```
    pub fn from_pixels(from: Pixel, to: Pixel) -> Option<Self> {
        let dx = to.x as f32 - from.x as f32;
        let dy = to.y as f32 - from.y as f32;
        Self::new(dx, dy)
    }

    /// Creates a unit vector from an angle in radians.
    ///
    /// The angle is measured counterclockwise from the positive x-axis (east),
    /// following standard mathematical convention.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let dir = Direction::from_angle(0.0);           // East: (1, 0)
    /// let dir = Direction::from_angle(std::f32::consts::PI / 2.0); // North: (0, -1)
    /// ```
    pub fn from_angle(angle: f32) -> Self {
        Self {
            dx: angle.cos(),
            dy: -angle.sin(), // Negative because y increases downward in screen space
        }
    }

    /// Returns the angle of this direction in radians, in the range `[-π, π]`.
    ///
    /// The angle is measured counterclockwise from the positive x-axis (east).
    /// A direction pointing east returns `0.0`, north returns `π/2`, etc.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let dir = Direction::new(1.0, 0.0).unwrap(); // East
    /// assert!((dir.angle() - 0.0).abs() < 1e-6);
    /// ```
    pub fn angle(&self) -> f32 {
        (-self.dy).atan2(self.dx)
    }

    /// Returns the opposite direction (180° rotation).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let east = Direction::new(1.0, 0.0).unwrap();
    /// let west = east.opposite();
    /// assert!((west.dx - (-1.0)).abs() < 1e-6);
    /// ```
    pub fn opposite(&self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }

    /// Returns a vector along this direction scaled by the given magnitude.
    ///
    /// Since `Direction` is always a unit vector, this produces a vector
    /// of length `magnitude` pointing in this direction.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let dir = Direction::new(1.0, 0.0).unwrap();
    /// let (dx, dy) = dir.scale(10.0);
    /// assert!((dx - 10.0).abs() < 1e-6);
    /// ```
    pub fn scale(&self, magnitude: f32) -> (f32, f32) {
        (self.dx * magnitude, self.dy * magnitude)
    }

    /// Calculates the dot product with another direction.
    ///
    /// For unit vectors, the dot product equals `cos(angle_between)`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let east = Direction::new(1.0, 0.0).unwrap();
    /// let north = Direction::new(0.0, -1.0).unwrap();
    /// assert!((east.dot(north) - 0.0).abs() < 1e-6); // Perpendicular
    /// ```
    pub fn dot(&self, other: Direction) -> f32 {
        self.dx * other.dx + self.dy * other.dy
    }

    /// Compares the alignment of this direction with another, returning a score in `[-1, 1]`.
    ///
    /// - `1.0` means perfectly aligned (same direction)
    /// - `0.0` means perpendicular
    /// - `-1.0` means opposite directions
    ///
    /// This is equivalent to the dot product for unit vectors.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let east = Direction::new(1.0, 0.0).unwrap();
    /// let north = Direction::new(0.0, -1.0).unwrap();
    /// let west = east.opposite();
    ///
    /// assert!((east.alignment(east) - 1.0).abs() < 1e-6);      // Same direction
    /// assert!((east.alignment(north)).abs() < 1e-6);           // Perpendicular
    /// assert!((east.alignment(west) - (-1.0)).abs() < 1e-6);   // Opposite
    /// ```
    pub fn alignment(&self, other: Direction) -> f32 {
        self.dot(other)
    }

    /// Calculates the 2D cross product (signed area) with another direction.
    ///
    /// Returns the z-component of the 3D cross product. Positive if `other`
    /// is counterclockwise from `self`, negative if clockwise.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let east = Direction::new(1.0, 0.0).unwrap();
    /// let north = Direction::new(0.0, -1.0).unwrap();
    /// assert!(east.cross(north) > 0.0); // north is counterclockwise from east
    /// ```
    pub fn cross(&self, other: Direction) -> f32 {
        self.dx * other.dy - self.dy * other.dx
    }

    /// Rotates this direction by the given angle in radians (counterclockwise).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let east = Direction::new(1.0, 0.0).unwrap();
    /// let north = east.rotate(std::f32::consts::PI / 2.0);
    /// assert!((north.dx - 0.0).abs() < 1e-6);
    /// assert!((north.dy - (-1.0)).abs() < 1e-6);
    /// ```
    pub fn rotate(&self, angle: f32) -> Self {
        let current_angle = self.angle();
        Self::from_angle(current_angle + angle)
    }
}
