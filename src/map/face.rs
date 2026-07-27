use ndarray::{Array1,arr1,NdFloat};
use std::fmt;

use crate::algorithms::signal;
/// Three points different from one another constitute a triangular `Face` that belongs in a 2D plane
/// We compute the normal of the plane (it will be oriented such that points p1,p2,p3 are in anti clockwise order).
/// The normal (a,b,c) and the offset (d) fully determine the plane (ax + by + cz + d = 0), but we still need the points to check if a ray intersects the plane within the boudaries of the triangle.
/// 
pub struct Face<'a,T:NdFloat>{
    pub normal: Array1<T>,
    pub offset: T,
    pub p1:&'a Array1<T>,
    pub p2:&'a Array1<T>,
    pub p3:&'a Array1<T>,
    pub squared_norm_first_vector: T,
    pub squared_norm_second_vector: T,
    pub scalar_product_vectors: T,
}

/// Display:
/// 
impl<'a,T:NdFloat> fmt::Display for Face<'a,T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Face constituted of points:\n")?;
        write!(f, "p1: {}\n", &self.p1)?;
        write!(f, "p2: {}\n", &self.p2)?;
        write!(f, "p3: {}\n", &self.p3)?;
        write!(f, "The plane has:\n")?;
        write!(f, "normal: {}\n", &self.normal)?;
        write!(f, "offset: {}\n", &self.offset)
    }
}

/// Example:
///```
/// use ndarray::{Array1,arr1,NdFloat};
/// use std::fmt;
/// use radar_and_tomography::map::face::Face;
/// 
/// let p1 = arr1(&[1.0,0.0,0.0]);
/// let p2 = arr1(&[0.0,1.0,0.0]);
/// let p3 = arr1(&[0.0,0.0,1.0]);
/// let face = Face::new((&p1,&p2,&p3));
/// println!("{face}");
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

        let squared_norm_first_vector = signal::scalar_prod(&vector1,&vector1);
        let squared_norm_second_vector = signal::scalar_prod(&vector2,&vector2);
        let scalar_product_vectors = signal::scalar_prod(&vector1,&vector2);

        Face{
            normal,
            offset,
            p1,
            p2,
            p3,
            squared_norm_first_vector,
            squared_norm_second_vector,
            scalar_product_vectors,
        }
    }
}
