use ndarray::{Array1,Array2,NdFloat};
use crate::map::face::Face;
/// An `Object` contains different points and `Face`
/// The object can move either by translation or rotation. Every point stored will move as well following the same movement. Each `Face` will be updated accordingly.
/// 
struct Object<'a,T:NdFloat>{
    faces: Vec<Face<'a,T>>,
    points: Vec<&'a Array1<T>>,
    movable: bool,
    type_object: char,
}

impl<'a,T:NdFloat> Object<'a,T>{
    pub fn new(points: Vec<&'a Array1<T>>,indices:Array2<usize>,movable:bool,type_object: char)->Self{
        let mut faces_object = Vec::new();
        for n_point in 0..indices.dim().0{
            let p1 = points[indices[[n_point,0]]];
            let p2 = points[indices[[n_point,1]]];
            let p3 = points[indices[[n_point,2]]];
            let face = Face::new((p1,p2,p3));
            faces_object.push(face);
        }
        Object{
            faces:faces_object,
            points,
            movable,
            type_object,
        }
    }
}