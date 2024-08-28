use crate::{
    curves::{circle::Circle, curve::Curve},
    points::point::Point,
    transforms::Transform,
    EQ_THRESHOLD,
};

use super::surface::TangentPoint;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub basis: Point,
    pub radius: f64,
    pub normal_outwards: bool,
}

pub enum SphereTransform {
    Sphere(Sphere),
    Ellipsoid(), // TODO: Implement this
}

impl Sphere {
    pub fn new(basis: Point, radius: f64, normal_outwards: bool) -> Sphere {
        Sphere {
            basis,
            radius,
            normal_outwards,
        }
    }

    pub fn transform(&self, transform: Transform) -> SphereTransform {
        let basis = transform * self.basis;
        let radius = self.radius * transform.uniform_scale_factor();
        SphereTransform::Sphere(Sphere::new(basis, radius, self.normal_outwards))
    }

    pub fn normal(&self, p: Point) -> Point {
        assert!(self.on_surface(p));
        if self.normal_outwards {
            (p - self.basis).normalize()
        } else {
            (self.basis - p).normalize()
        }
    }

    pub fn neg(&self) -> Sphere {
        Sphere::new(self.basis, self.radius, !self.normal_outwards)
    }

    pub fn on_surface(&self, p: Point) -> bool {
        let diff = p - self.basis;
        let dist = diff.norm_sq();
        (dist - self.radius * self.radius).abs() < EQ_THRESHOLD
    }

    pub fn metric(&self, _x: Point, u: TangentPoint, v: TangentPoint) -> f64 {
        u.dot(v)
    }

    pub fn distance(&self, x: Point, y: Point) -> f64 {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        let angle = (x - self.basis).angle(y - self.basis);
        self.radius * angle
    }

    pub fn exp(&self, x: Point, u: TangentPoint) -> Point {
        assert!(self.on_surface(x));

        if u.norm() < EQ_THRESHOLD {
            return x;
        }
        let u_norm = u.norm();
        let u_normalized = u / u_norm;
        x * u_norm.cos() + u_normalized * u_norm.sin() * self.radius + self.basis
    }

    pub fn log(&self, x: Point, y: Point) -> Option<TangentPoint> {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));

        if x == y {
            return Some(Point::new_zero());
        }
        let x2 = (x - self.basis) / self.radius;
        let y2 = (y - self.basis) / self.radius;
        let dir = y2 - x2.dot(y2) * x2;
        let dir_norm = dir.norm();
        // For the case that we are on the opposite side of the sphere
        if dir_norm < EQ_THRESHOLD {
            return None;
        }
        Some(self.distance(x, y) * dir / dir_norm)
    }

    pub fn parallel_transport(
        &self,
        v: Option<TangentPoint>,
        x: Point,
        y: Point,
    ) -> Option<TangentPoint> {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        match v {
            None => {
                return None;
            }
            Some(v) => {
                let x = (x - self.basis) / self.radius;
                let y = (y - self.basis) / self.radius;
                let u = self.log(x, y);
                match u {
                    None => return Some(-y),
                    Some(u) => {
                        let u_norm = u.norm();
                        if u_norm < EQ_THRESHOLD {
                            return Some(v);
                        }
                        let u_normalized = u / u_norm;
                        Some(
                            -x * u_norm.sin() * u_normalized.dot(v)
                                + u_normalized * u_norm.cos() * u_normalized.dot(v)
                                + v
                                + u_normalized * u_normalized.dot(v),
                        )
                    }
                }
            }
        }
    }

    pub fn geodesic(&self, p: Point, q: Point) -> Curve {
        assert!(self.on_surface(p));
        assert!(self.on_surface(q));
        assert!(p != q);
        let normal = (p - self.basis).cross(q - self.basis).normalize();
        let circle = Circle::new(self.basis, normal, self.radius);
        Curve::Circle(circle)
    }

    pub fn point_grid(&self, density: f64) -> Vec<Point> {
        let n = (16.0 * density) as usize;
        let m = (16.0 * density) as usize;
        let mut points = Vec::with_capacity(n * m);
        for i in 0..n {
            for j in 0..m {
                let theta = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                let phi = std::f64::consts::PI * j as f64 / (m - 1) as f64;
                let x = self.basis.x + self.radius * theta.cos() * phi.sin();
                let y = self.basis.y + self.radius * theta.sin() * phi.sin();
                let z = self.basis.z + self.radius * phi.cos();
                points.push(Point::new(x, y, z));
            }
        }
        points
    }

    pub fn project(&self, point: Point) -> Point {
        let diff = point - self.basis;
        let dist = diff.norm();
        if dist < EQ_THRESHOLD {
            return self.basis;
        }
        self.basis + diff * self.radius / dist
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Sphere) -> bool {
        self.basis == other.basis
            && (self.radius - other.radius).abs() < EQ_THRESHOLD
            && self.normal_outwards == other.normal_outwards
    }
}
