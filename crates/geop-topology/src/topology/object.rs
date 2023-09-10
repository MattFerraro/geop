use std::rc::Rc;

use super::{contour::Contour, face::Face, vertex::Vertex};

pub struct Object {
    pub faces: Vec<Rc<Face>>,
}

pub enum ObjectIntersection {
    TouchingContour(Contour),
    CrossingContour(Contour),
    TouchingVertex(Vertex),
}

impl Object {
    pub fn new(faces: Vec<Rc<Face>>) -> Object {
        Object { faces }
    }

    pub fn intersect(&self, _other: &Object) -> Vec<Rc<ObjectIntersection>> {
        todo!("Implement intersect");
    }
}
