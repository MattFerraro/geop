use geop_geometry::{
    curves::curve::Curve,
    points::point::Point,
    surface_surface_intersection::surface_surface::{
        surface_surface_intersection, FaceSurfaceIntersection,
    },
};

use geop_topology::{
    contains::face_point::{face_point_contains, FacePointContains},
    topology::{edge::Edge, face::Face},
};

use crate::remesh::face::{face_remesh, face_split, normalize_faces, FaceSplit};

use super::face_edge::{face_edge_intersection, FaceEdgeIntersection};

pub fn face_face_same_surface_intersection(face_self: &Face, face_other: &Face) -> Vec<Face> {
    assert!(
        face_self.surface == face_other.surface,
        "Faces must have the same surface",
    );

    let edges = face_split(face_self, face_other)
        .drain(..)
        .filter(|mode| match mode {
            FaceSplit::AinB(_) => true,
            FaceSplit::AonBSameSide(_) => true,
            FaceSplit::AonBOpSide(_) => false,
            FaceSplit::AoutB(_) => false,
            FaceSplit::BinA(_) => true,
            FaceSplit::BonASameSide(_) => false,
            FaceSplit::BonAOpSide(_) => false,
            FaceSplit::BoutA(_) => false,
        })
        .collect::<Vec<FaceSplit>>();

    let contours = face_remesh(edges);
    return normalize_faces(contours, face_self.surface.clone());
}

pub enum FaceFaceIntersection {
    None,
    EdgesAndPoints(Vec<Point>, Vec<Edge>),
    Faces(Vec<Face>),
}

fn curve_face_intersection_same_surface(_curve: Curve, _face: Face) -> Vec<Edge> {
    todo!()
}

pub fn face_face_intersection(face_self: &Face, face_other: &Face) -> FaceFaceIntersection {
    match surface_surface_intersection(&face_self.surface, &face_other.surface) {
        FaceSurfaceIntersection::None => FaceFaceIntersection::None,
        FaceSurfaceIntersection::CurvesAndPoints(curves, points) => {
            let mut points = points
                .iter()
                .filter(|p| {
                    face_point_contains(face_self, **p) != FacePointContains::Outside
                        && face_point_contains(face_other, **p) != FacePointContains::Outside
                })
                .cloned()
                .collect::<Vec<Point>>();

            let curves = curves
                .iter()
                .map(|curve| curve_face_intersection_same_surface(curve.clone(), face_self.clone()))
                .flatten()
                .map(|edge| face_edge_intersection(face_other, &edge))
                .collect::<Vec<FaceEdgeIntersection>>();

            let mut edges = Vec::<Edge>::new();
            for curve in curves.iter() {
                match curve {
                    FaceEdgeIntersection::Points(ps) => {
                        points.extend(ps);
                    }
                    FaceEdgeIntersection::Edges(es) => {
                        edges.extend(es.clone());
                    }
                    FaceEdgeIntersection::None => {}
                }
            }

            FaceFaceIntersection::EdgesAndPoints(points, edges)
        }
        FaceSurfaceIntersection::Surface(_surface) => {
            FaceFaceIntersection::Faces(face_face_same_surface_intersection(face_self, face_other))
        }
    }
}
