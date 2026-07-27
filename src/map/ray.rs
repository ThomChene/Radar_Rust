use ndarray::{Array1,NdFloat};
use std::fmt;
use crate::{algorithms::signal, map::{face::Face,object::Object}};

pub struct Ray<T:NdFloat>{
    amplitude:T,
    distance:T,
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

    pub fn propagate<'a>(&self,objects: &'a Vec<Object<'a,T>>)->Option<(usize,Array1<T>,&Face<'a,T>)>
    {
        let mut result:Option<(usize,Array1<T>,&Face<'a,T>)> = None;
        let mut closest_dist = T::max_value();
        for (i, obj) in objects.iter().enumerate(){
            let option: Option<(Array1<T>,&Face<'a,T>)> = obj.intersect(self);
            if !option.is_none(){
                let (intersection, face) = option.unwrap();
                let diff = &self.origin - &intersection;
                let dist = signal::l2_norm(&diff);
                if closest_dist>dist{
                    closest_dist = dist;
                    result = Some((i,intersection,face));
                }
            }
        }
        result
    }
}
