use ndarray::{Array1,arr1,NdFloat};
/// Three points different from one another constitute a triangular `Face` that belongs in a 2D plane
/// We compute the normal of the plane (it will be oriented such that points p1,p2,p3 are in anti clockwise order).
/// The normal and the offset fully determine the plane, but we still need the points to check if a ray intersects the plane within the boudaries of the triangle.
/// Only stores the adress of the points, the points are owned by the `Object` in which the `Face` is.
/// 
pub struct Face<'a,T:NdFloat>{
    normal: Array1<T>,
    offset: T,
    p1:&'a Array1<T>,
    p2:&'a Array1<T>,
    p3:&'a Array1<T>,
}

/// Example:
///```
/// let p1 = arr1(&[1.0,0.0,0.0]);
/// let p2 = arr1(&[0.0,1.0,0.0]);
/// let p3 = arr1(&[0.0,0.0,1.0]);
/// let face1 = Face::new((&p1,&p2,&p3));
///```
/// 
impl<'a,T:NdFloat> Face<'a,T>{
    pub fn new(value:(&'a Array1<T>,&'a Array1<T>,&'a Array1<T>))->Self{
        /* Cross Product to compute the normal */
        let (p1,p2,p3) = value;
        let vector1 = p2-p1;
        let vector2 = p3-p1;
        let value1 = vector1[1]*vector2[2]-vector1[2]*vector2[1];
        let value2 = vector1[2]*vector2[0]-vector1[0]*vector2[2];
        let value3 = vector1[0]*vector2[1]-vector1[1]*vector2[0];
        let normal = arr1(&[value1,value2,value3]);
        let offset = -normal.dot(p1);

        Face{
            normal,
            offset,
            p1,
            p2,
            p3,
        }
    }
}
