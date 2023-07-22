use crate::geometry::curves::Curve3d;
use crate::geometry::points::Point3d;

struct Line3d {
    basis: Point3d,
    slope: Point3d,
    is_normalized: bool
}

impl Curve3d for Line3d {
    fn get_value(&self, u: f64) -> Point3d {
        self.basis + u * self.slope
    }

    fn project(&self, x: Point3d) -> f64 {
        let v = x - self.basis;
        v.dot(self.slope) / self.slope.norm()
    }

    fn normalize(&mut self) {
        if !self.is_normalized {
            self.slope /= self.slope.norm();
            self.is_normalized = true;
        }
    }

    fn is_normalized(&self) -> bool {
        self.is_normalized
    }
}
