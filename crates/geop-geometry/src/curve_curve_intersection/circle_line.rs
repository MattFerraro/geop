use crate::{curves::{circle::Circle, line::Line}, points::point::Point, EQ_THRESHOLD};

pub enum CircleLineIntersection {
    TwoPoint(Point, Point),
    OnePoint(Point),
    None
}

pub fn circle_line_intersection(a: &Circle, b: &Line) -> CircleLineIntersection {
    let v = b.basis - a.basis;
    let u = b.direction.dot(b.direction);
    let v = v.dot(b.direction);
    let w = v * v - u * (v * v - a.radius * a.radius);
    if w < -EQ_THRESHOLD {
        CircleLineIntersection::None
    } else if w < EQ_THRESHOLD {
        CircleLineIntersection::OnePoint(a.basis + b.direction * v / u)
    } else {
        let w = w.sqrt();
        CircleLineIntersection::TwoPoint(a.basis + b.direction * (v - w) / u, a.basis + b.direction * (v + w) / u)
    }
}
