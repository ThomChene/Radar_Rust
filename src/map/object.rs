use ndarray::{Array1,Array2,NdFloat};
use crate::map::face::Face;
use crate::map::ray::Ray;
use crate::algorithms::signal;
use crate::algorithms::signal::Signal;
use std::fmt;
/// An `Object` contains one or multiple `Face`.
/// If movable is true, the object can move either by translation or rotation. Every point constituting the `Object` will move as well following the same motion.
/// 
pub struct Object<'a,T:NdFloat>{
    faces: Vec<Face<'a,T>>,
    movable: bool,
}
pub struct Transmitter<'a,T:NdFloat>{
    object: Object<'a,T>,
    origin_rays: Array1<T>
}
pub struct Receiver<'a,T:NdFloat>{
    object: Object<'a,T>,
    signal_received: Signal<T>
}
pub struct Transceiver<'a,T:NdFloat>{
    object: Object<'a,T>,
    origin_rays: Array1<T>,
    signal_received: Signal<T>
}
pub struct GroupObject<'a, T:NdFloat>{
    receivers: Vec<Receiver<'a, T>>,
    transceivers: Vec<Transceiver<'a, T>>,
}
/// Trait of Objects
/// 
/// 
pub trait CanTransmit{
    fn create_rays<T:NdFloat>(&self,n_rays:u32)->Vec<Ray<T>>{
        let rays = Vec::new();
        rays
    }
}
pub trait CanReceive{
    fn add_ray(&self){}
}

impl<'a,T:NdFloat> CanTransmit for Transmitter<'a,T>{}
impl<'a,T:NdFloat> CanReceive for Receiver<'a,T>{}
impl<'a,T:NdFloat> CanTransmit for Transceiver<'a,T>{}
impl<'a,T:NdFloat> CanReceive for Transceiver<'a,T>{}


impl<'a,T:NdFloat> Object<'a,T>{
    pub fn intersect(&self,ray: &Ray<T>)->Option<(Array1<T>,&Face<'a,T>)>{
        // If there is a collision between the ray and the object
        let faces = &self.faces;
        let mut closest_dist = T::max_value();
        let mut result:Option<(Array1<T>,&Face<'a,T>)> = None;
        for (i,face) in faces.iter().enumerate(){
            // First compute the intersection between the ray and the plane
            let new_offset: T = face.offset + signal::scalar_prod(&face.normal,&ray.origin);
            let other_offset = signal::scalar_prod(&face.normal,&ray.direction);
            // If the normal is perpendicular to the direction of the ray, there is no intersection
            if !other_offset.is_zero(){
                let a = -(new_offset/other_offset);
                let intersection_face_ray = &ray.direction*a+&ray.origin;
                // Checks that the intersection is in the direction of propagation of the ray and not behind the transmitter
                if a<T::zero(){
                    // Then check if the intersection is inside the triangle which means intersection_face_ray-P1 = lambda1*(P2-P1)+lambda2*(P3-P1), with 0<lambda1+lambda2<1, and 0<lambda1<1 and and 0<lambda2<1
                    // By computing <intersection_face_ray-P1,P2-P1> and <intersection_face_ray-P1,P3-P1> we obtain a system, and by solving the system we find lambda1, lambda2 and check that 0<lambda1+lambda2<1 and 0<lambda1<1 and and 0<lambda2<1
                    // Or just 0<lambda1, 0<lambda2 and lambda1+lambda2<1
                    let v1 = &intersection_face_ray-face.p1;
                    let v2 = face.p2-face.p1;
                    let v3 = face.p3-face.p1;
                    let sc1 = signal::scalar_prod(&v1,&v2);
                    let sc2 = signal::scalar_prod(&v1,&v3);
                    let s1 = face.squared_norm_first_vector;
                    let s2 = face.squared_norm_second_vector;
                    let sp = face.scalar_product_vectors;
                    let normalization = s2*s1-sp*sp;
                    let lambda1 = sc1*s2 - sc2*sp;
                    let lambda2 = sc2*s1 - sc1*sp;

                    if T::zero()<lambda1 && T::zero()<lambda2 && lambda1+lambda2<normalization{
                        let diff = &ray.origin - &intersection_face_ray;
                        let dist = signal::l2_norm(&diff);
                        if closest_dist>dist{
                            closest_dist = dist;
                            result = Some((intersection_face_ray,&faces[i]));
                        }
                    }
                }
            }
        }
        result
    }  
}

/// Display:
/// 
impl<'a,T:NdFloat> fmt::Display for Object<'a,T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let can;
        match &self.movable{
            true => can = "can",
            false => can = "cannot",
        };
        write!(f, "This Object {} move, with faces:\n", can)?;
        let faces = &self.faces;
        for v in faces.iter() {
            write!(f, "p1: {}\n", v)?;
        }
        write!(f, "")
    }
}

/// Example:
///```
/// let p1 = arr1(&[1.0,0.0,0.0]);
/// let p2 = arr1(&[0.0,1.0,0.0]);
/// let p3 = arr1(&[0.0,0.0,1.0]);
/// let p4 = arr1(&[0.0,1.0,1.0]);
/// let indices = arr2(&[[0,1,2],[0,1,3]]);
/// let object = Object::new(Vec::from([&p1,&p2,&p3,&p4]),indices,true,TypeObject::Receiver);
/// println!("{object}");
///```
/// 
/// 
impl<'a,T:NdFloat> Object<'a,T>{
    pub fn new(points: Vec<&'a Array1<T>>,indices:Array2<usize>,movable:bool)->Self{
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
            movable,
        }
    }
}