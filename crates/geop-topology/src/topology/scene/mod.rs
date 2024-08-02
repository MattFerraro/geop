use std::ops::Mul;

use geop_geometry::points::point::Point;

use super::{edge::Edge, face::Face, volume::Volume};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn standard_pallet(dark_mode: bool) -> (Color, Color, Color, Color) {
        let background_color = if dark_mode {
            Color::from_brightness(0.2)
        } else {
            Color::from_brightness(1.0)
        };
        let face_color = if dark_mode {
            Color::from_brightness(0.2)
        } else {
            Color::from_brightness(0.6)
        };
        let edge_color = if dark_mode {
            Color::from_brightness(0.7)
        } else {
            Color::from_brightness(0.2)
        };
        let point_color = if dark_mode {
            Color::from_brightness(0.8)
        } else {
            Color::from_brightness(0.1)
        };

        (background_color, face_color, edge_color, point_color)
    }

    pub fn from_brightness(brightness: f32) -> Color {
        Color {
            r: brightness,
            g: brightness,
            b: brightness,
            a: 1.0,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }

    pub fn red() -> Color {
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub volumes: Vec<(Volume, Color)>,
    pub faces: Vec<(Face, Color)>,
    pub edges: Vec<(Edge, Color)>,
    pub points: Vec<(Point, Color)>,
}

impl Scene {
    pub fn new(
        volumes: Vec<(Volume, Color)>,
        faces: Vec<(Face, Color)>,
        edges: Vec<(Edge, Color)>,
        points: Vec<(Point, Color)>,
    ) -> Scene {
        Scene {
            volumes,
            faces,
            edges,
            points,
        }
    }
}
