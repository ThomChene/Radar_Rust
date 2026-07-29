use ndarray::{Array1,NdFloat,arr1};
use std::fmt;
use crate::algorithms::signal;
use super::object::Object;

pub struct Ray<T:NdFloat>{
    pub amplitude:T,
    pub distance:T,
    pub origin:Array1<T>,
    pub direction:Array1<T>,
}

/// Display:
/// 
impl<T:NdFloat> fmt::Display for Ray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ray with amplitude {} that crossed {} meters, with origin and direction:\n",&self.amplitude,&self.distance)?;
        write!(f, "origin: {}\n", &self.origin)?;
        write!(f, "direction: {}\n", &self.direction)
    }
}

/// Example:
///```
/// let amplitude = 1;
/// let distance 0;
/// let origin = arr1(&[0.0,0.0,0.0]);
/// let direction = arr1(&[1.0,0.0,0.0]);
/// let ray = Ray::new(amplitude,distance,origin,direction);
/// println!("{ray}");
///```
/// 
impl<T:NdFloat> Ray<T>{
    pub fn new(amplitude: T,distance: T,origin: Array1<T>,direction: Array1<T>)->Self{
        Ray{
            amplitude,
            distance,
            origin,
            direction,
        }
    }

    pub fn propagate<'a>(&mut self,objects: &'a Vec<Object<'a,T>>)
    {
        let mut closest_dist = T::max_value();
        let mut closest_obj:Option<&Object<'a,T>> = None;
        let mut intersection_face_ray = arr1(&[T::zero(),T::zero(),T::zero()]);
        for obj in objects.iter(){
            let faces = &obj.faces;
            for face in faces.iter(){
                // First compute the intersection between the ray and the plane
                let new_offset: T = face.offset + signal::scalar_prod(&face.normal,&self.origin);
                let other_offset = signal::scalar_prod(&face.normal,&self.direction);
                // If the normal is perpendicular to the direction of the ray, there is no intersection
                if !other_offset.is_zero(){
                    let a = -(new_offset/other_offset);
                    intersection_face_ray = &self.direction*a + &self.origin;
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
                        let normalization = s2*s1 - sp*sp;
                        let lambda1 = sc1*s2 - sc2*sp;
                        let lambda2 = sc2*s1 - sc1*sp;

                        if T::zero()<lambda1 && T::zero()<lambda2 && lambda1+lambda2<normalization{
                            let diff = &self.origin - &intersection_face_ray;
                            let dist = signal::l2_norm(&diff);
                            if closest_dist>dist{
                                closest_dist = dist;
                                closest_obj = Some(obj);
                            }
                        }
                    }
                }
            }
        }  
        match closest_obj{
            None => self.amplitude = T::zero(),
            Some(obj) => (self.amplitude, self.distance, self.origin, self.direction)  = obj.ray_received(self, closest_dist, intersection_face_ray),

        }
    }
}
