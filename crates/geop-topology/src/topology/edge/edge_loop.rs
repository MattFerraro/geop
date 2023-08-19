use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::{edge::edge::EdgeIntersection, vertex::Vertex};

use super::edge::Edge;

#[derive(Debug)]
pub struct EdgeLoop {
    pub edges: Vec<Rc<Edge>>,
}

// An EdgeLoop is a closed loop of edges which is not self intersecting (because otherwise project would not be defined for self intersection point).
impl EdgeLoop {
    pub fn new(edges: Vec<Rc<Edge>>) -> EdgeLoop {
        for i in 0..edges.len() {
            let edge = edges[i].clone();
            let next_edge = edges[(i + 1) % edges.len()].clone();
            assert!(edge.end == next_edge.start);
        }
        assert!(edges.len() > 0);
        assert!(edges[0].start == edges[edges.len() - 1].end);
        EdgeLoop { edges }
    }

    pub fn point_at(&self, u: f64) -> Point {
        let mut u = u % 1.0;
        u = u * self.edges.len() as f64;
        let i = u.floor() as usize;
        u = u - i as f64;
        let edge = self.edges[i].clone();
        edge.point_at(u)
    }

    pub fn project(&self, point: &Point) -> Option<f64> {
        let mut u = 0.0;
        for edge in self.edges.iter() {
            match edge.project(point) {
                Some(u_p) => {
                    return Some((u + u_p) / self.edges.len() as f64);
                }
                None => {
                    u += 1.0;
                }
            }
        }
        None
    }

    pub fn rasterize(&self) -> Vec<Point> {
        self.edges
            .iter()
            .map(|edge| edge.rasterize())
            .flatten()
            .collect()
    }

    fn get_subcurve(&self, start: Vertex, end: Vertex) -> Result<Vec<Rc<Edge>>, &'static str> {
        let u_start = match self.project(&start.point) {
            Some(it) => it,
            None => return Err("First point is not on the edge loop"),
        };
        let mut u_end = match self.project(&end.point) {
            Some(it) => it,
            None => return Err("Second point is not on the edge loop"),
        };
        if u_end < u_start {
            u_end += 1.0;
        }
        let mut sub_edges: Vec<Rc<Edge>> = Vec::new();
        let n = self.edges.len();
        // Keep in mind that for u == 1.0, start_i or end_i can be n.
        let start_i = (u_start * n as f64).floor() as usize;
        let end_i = (u_end * n as f64).floor() as usize;
        if start_i == end_i {
            sub_edges.push(Rc::new(Edge::new(
                start,
                end,
                self.edges[start_i % n].curve.clone(),
                self.edges[start_i % n].direction.clone(),
            )));
        } else {
            sub_edges.push(Rc::new(Edge::new(
                start,
                self.edges[start_i % n].end.clone(),
                self.edges[start_i % n].curve.clone(),
                self.edges[start_i % n].direction.clone(),
            )));
            for i in start_i + 1..end_i {
                sub_edges.push(self.edges[i % n].clone());
            }
            sub_edges.push(Rc::new(Edge::new(
                self.edges[end_i % n].start.clone(),
                end,
                self.edges[end_i % n].curve.clone(),
                self.edges[end_i % n].direction.clone(),
            )));
        }
        Ok(sub_edges)
    }

    // Takes 2 EdgeLoops and connects them at intersecting points with new vertices.
    // If there are overlapping edges, there will be a vertex for the beginning and the end of the overlapping edges, and a connecting edge for each loop.
    // If there are no intersections, the outer vector will have length 1.
    fn cutting_split(&self, other: &EdgeLoop) -> Option<(Vec<Vec<Rc<Edge>>>, Vec<Vec<Rc<Edge>>>)> {
        // First, find all intersections and order them by position on the edge loop.
        let mut split_verts: Vec<Vertex> = Vec::new();
        for edge_self in self.edges.iter() {
            for edge_other in other.edges.iter() {
                let intersections = edge_self.cutting_intersections(&edge_other);
                for intersection in intersections {
                    match intersection {
                        EdgeIntersection::Vertex(intersect_vertex) => {
                            // If we have another intersection at the same point, this is a touching edge.
                            println!("Intersection at {:?}", intersect_vertex.point);
                            let index = split_verts.iter().position(|vert| vert == &intersect_vertex);
                            match index {
                                // Some(index) => {
                                //     split_verts.swap_remove(index);
                                // },
                                None => {
                                    split_verts.push(intersect_vertex);
                                },
                                _ => {}
                            }
                        }
                        EdgeIntersection::Edge(_) => {
                            // If two squares are touching with one side, the edge points will be found 3 times.
                            // A1--- + --- B1 <-- this point is going to be found 3 times
                            //       | <-- Edge A2 and B2
                            //       |
                            // A3--- + --- B3
                            // One time for A1 & B1, A1 & B2, A2 & B1
                            // A2 & B2 (counts twice since it is an intersection edge and is hence ignored)
                        }
                    }
                }
            }
        }

        // Due to the properties of edge loops, the edges intersecting the other edge loop can be generated by zipping two following vertices.
        let n: usize = split_verts.len();
        assert!(n % 2 == 0);
        if n == 0 {
            return None;
        }

        let mut split_verts_self: Vec<(f64, Vertex)> = Vec::new();
        let mut split_verts_other: Vec<(f64, Vertex)> = Vec::new();
        for vert in split_verts.iter() {
            split_verts_self.push((
                self.project(&vert.point)
                    .expect("Intersection point has to be on edge"),
                vert.clone(),
            ));
            split_verts_other.push((
                other
                    .project(&vert.point)
                    .expect("Intersection point has to be on edge"),
                vert.clone(),
            ));
        }
        split_verts_self.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        split_verts_other.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());


        let mut segments_self = Vec::with_capacity(n);
        for i in 0..n {
            let start_vert = split_verts_self[i].1.clone();
            let end_vert = split_verts_self[(i + 1) % n].1.clone();
            let edge_self = self
                .get_subcurve(start_vert, end_vert)
                .expect("Intersection points have to be on edge");
            segments_self.push(edge_self);
        }

        let mut segments_other = Vec::with_capacity(n);
        for i in 0..n {
            let start_vert = split_verts_other[i].1.clone();
            let end_vert = split_verts_other[(i + 1) % n].1.clone();
            let edge_other = other
                .get_subcurve(start_vert, end_vert)
                .expect("Intersection points have to be on edge");
            segments_other.push(edge_other);
        }

        Some((segments_self, segments_other))
    }

    // Splits this edge loop with another edge loop.
    // This makes sure that the resulting edge loops are closed and do not intersect each other anymore.
    // Neighbouring edge loops will share the same end points for the edges, and the two neighbouring edges will face opposite direction.
    pub fn remesh_self_other(&self, other: &EdgeLoop) -> Option<Vec<EdgeLoop>> {
        let (mut segments_self, mut segments_other) = self.cutting_split(other)?;
        for segment in segments_self.iter() {
            println!("New segment self");
            for edge in segment.iter() {
                println!("Edge: {:?} - {:?}", edge.start.point, edge.end.point);
            }
        }

        for segment in segments_other.iter() {
            println!("New segment other");
            for edge in segment.iter() {
                println!("Edge: {:?} - {:?}", edge.start.point, edge.end.point);
            }
        }

        let mut edge_loops = Vec::new();
        let mut next_segment_is_self = false;
        while let Some(mut next_segment) = segments_self.pop() {
            let mut edge_loop: Vec<Rc<Edge>> = next_segment.drain(..).collect();
            while edge_loop.len() == 0 || edge_loop[edge_loop.len() - 1].end != edge_loop[0].start {
                let relevant_segments = if next_segment_is_self {
                    &mut segments_self
                } else {
                    &mut segments_other
                };
                for (i, segment) in relevant_segments.iter().enumerate() {
                    if segment[0].start == edge_loop[edge_loop.len() - 1].end {
                        next_segment = relevant_segments.remove(i);
                        break;
                    }
                }
                edge_loop.extend(next_segment.drain(..));
                next_segment_is_self = !next_segment_is_self;
            }
            edge_loops.push(EdgeLoop::new(edge_loop));
        }
        Some(edge_loops)
    }

    // If no intersection is there, the result is None. Otherwise we can be sure that the result is a single edge loop.
    pub fn union(&self, other: &EdgeLoop) -> Option<EdgeLoop> {
        let mut edge_loops = self.remesh_self_other(other)?;

        // Find an outer vertex
        let mut outer_edge = &edge_loops[0].edges[0];
        for edge_loop in edge_loops.iter() {
            for edge in edge_loop.edges.iter() {

                match edge.start.point.x.partial_cmp(&outer_edge.start.point.x) {
                    Some(std::cmp::Ordering::Less) => {
                        outer_edge = edge;
                    },
                    _ => {
                        match edge.start.point.y.partial_cmp(&outer_edge.start.point.y) {
                            Some(std::cmp::Ordering::Less) => {
                                outer_edge = edge;
                            },
                            _ => {
                                match edge.start.point.z.partial_cmp(&outer_edge.start.point.z) {
                                    Some(std::cmp::Ordering::Less) => {
                                        outer_edge = edge;
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                };
            }
        }

        // Find the edge loop which contain the outer vertex.
        let outer_edge_loop_index = edge_loops.iter().position(|edge_loop| edge_loop.edges.contains(&outer_edge)).unwrap();

        Some(edge_loops.swap_remove(outer_edge_loop_index))
    }
}
