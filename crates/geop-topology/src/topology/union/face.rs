use crate::topology::{face::Face, remesh::face::{FaceSplit, face_split, face_remesh}};


pub fn face_union(face_self: &Face, face_other: &Face) -> Face {
    assert!(
        face_self.surface == face_other.surface,
        "Faces must have the same surface",
    );

    let edges = face_split(face_self, face_other)
        .drain(..)
        .filter(|mode| match mode {
            FaceSplit::AinB(_) => false,
            FaceSplit::AonBSameSide(_) => true,
            FaceSplit::AonBOpSide(_) => false,
            FaceSplit::AoutB(_) => true,
            FaceSplit::BinA(_) => false,
            FaceSplit::BonASameSide(_) => false,
            FaceSplit::BonAOpSide(_) => false,
            FaceSplit::BoutA(_) => true,
        }).collect::<Vec<FaceSplit>>();

    return face_remesh(face_self.surface.clone(), edges);
}
