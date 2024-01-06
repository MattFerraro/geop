use std::rc::Rc;

use geop_geometry::{
    curve_surface_intersection::curve_surface::curve_surface_intersection,
    points::point::Point, surfaces::surface::Surface, transforms::Transform,
};

use crate::topology::contains::edge_point::EdgePointContains;

use super::{edge::Edge, contour::ContourTangent};

#[derive(Clone, Debug)]
pub struct Face {
    pub boundaries: Vec<Edge>,
    // pub boundary_points: Vec<Point>, // TODO
    pub surface: Rc<Surface>,
}


// Implements a Face. A Face is bounded by the outer_loop and might have holes in inner_loops.
// outer_loop has to be clockwise, if the face is looked at from normal direction (normal facing towards you).
// inner_loops have to be counter-clockwise, if the face is looked at from normal direction (normal facing towards you).
// The contours are not allowed to intersect in any way. Keep in mind that a point is not considered an intersection, hence it is allowed that the contours touch each other at points.
impl Face {
    pub fn new(boundaries: Vec<Edge>, surface: Rc<Surface>) -> Face {
        assert!(boundaries.len() > 0, "Face must have at least one boundary");
        for edge in boundaries.iter() {
            assert!(curve_surface_intersection(&*edge.curve, &surface).is_curve());
        }
        // for (i, contour_a) in boundaries.iter().enumerate() {
        //     for edge_a in contour_a.edges.iter() {
        //         for contour_b in boundaries[..i].iter() {
        //             for edge_b in contour_b.edges.iter() {
        //                 assert!(
        //                     edge_edge_intersections(edge_a, edge_b).len() == 0,
        //                     "Contours are not allowed to intersect"
        //                 );
        //             }
        //         }
        //     }
        // }
        Face {
            boundaries,
            surface,
        }
    }

    pub fn transform(&self, transform: Transform) -> Face {
        Face::new(
            self.boundaries
                .iter()
                .map(|contour| contour.transform(transform))
                .collect(),
            Rc::new(self.surface.transform(transform)),
        )
    }

    pub fn all_points(&self) -> Vec<Rc<Point>> {
        let mut points = Vec::<Rc<Point>>::new();

        for contour in self.boundaries.iter() {
            points.extend(contour.all_points());
        }
        return points;
    }

    pub fn edge_from_to(&self, from: Rc<Point>, to: Rc<Point>) -> Rc<Edge> {
        Rc::new(Edge::new(
            vec![(from.clone(), to.clone())],
            Rc::new(self.surface.geodesic(*from, *to)),
        ))
    }

    pub fn boundary_tangent(&self, p: Point) -> ContourTangent {
        for contour in self.boundaries.iter() {
            match contour.contains(p) {
                EdgePointContains::Inside => return contour.tangent(p),
                EdgePointContains::OnPoint(_) => return contour.tangent(p),
                EdgePointContains::Outside => continue,
            }
        }
        panic!("Point is not on boundary");
    }

    pub fn neg(&self) -> Face {
        Face {
            boundaries: self.boundaries.iter().rev().map(|l| l.neg()).collect(),
            surface: self.surface.clone(),
        }
    }

    pub fn flip(&self) -> Face {
        Face {
            boundaries: self.boundaries.iter().rev().map(|l| l.neg()).collect(),
            surface: Rc::new(self.surface.neg()),
        }
    }
}

// pretty print
impl std::fmt::Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self.surface {
            Surface::Plane(p) => {
                writeln!(
                    f,
                    "Plane at basis = {:?} with normal = {:?}",
                    p.basis,
                    p.u_slope.cross(p.v_slope)
                )?;
                for contour in self.boundaries.iter() {
                    writeln!(f, "Contour:")?;
                    for edge in contour.edges.iter() {
                        writeln!(f, "  {}", edge)?;
                    }
                }
            }
            Surface::Sphere(_s) => {
                writeln!(f, "sphere is still todo")?;
            }
        };
        writeln!(f, "Boundaries:")
    }
}
