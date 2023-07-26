use crate::geometry::{surfaces::plane::Plane, curves::line3d::Line3d, points::point3d::Point3d};

pub enum PlanePlaneIntersection {
    Plane(Plane),
    Line3d(Line3d),
    None
}

pub fn intersect(a: &Plane, b: &Plane) -> PlanePlaneIntersection {
    let n_a = a.normal();
    let n_b = b.normal();
    let b_a: Point3d = a.basis;
    let b_b: Point3d = b.basis;

    let v = n_a.cross(n_b);
    if v.norm() > crate::EQ_THRESHOLD {
        let t = (n_a.dot(b_b) - n_a.dot(b_a)) / n_a.dot(v);
        PlanePlaneIntersection::Line3d(Line3d::new(b_a + v * t, v))
    } else {
        let n = n_a.dot(b_a - b_b);
        if n.abs() < crate::EQ_THRESHOLD {
            PlanePlaneIntersection::Plane(Plane::new(a.basis, a.u_slope, a.v_slope))
        } else {
            PlanePlaneIntersection::None
        }
    }
}
