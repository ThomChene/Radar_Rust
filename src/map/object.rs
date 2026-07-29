use ndarray::{Array1,Array2,NdFloat};
use crate::map::face::Face;
use crate::map::ray::Ray;
use crate::algorithms::signal::Signal;
use std::fmt;
/// An `Object` contains one or multiple `Face`.
/// If movable is true, the object can move either by translation or rotation. Every point constituting the `Object` will move as well following the same motion.
/// 
pub struct Object<'a,T:NdFloat>{
    pub faces: Vec<Face<'a,T>>,
    pub movable: bool,
    pub type_object: TypeObject<T>,
}
pub struct Transmitter<T:NdFloat>{
    origin_rays: Array1< T>
}
pub struct Receiver<T:NdFloat>{
    signal_received: Signal<T>
}
pub struct Transceiver<T:NdFloat>{
    origin_rays: Array1<T>,
    signal_received: Signal<T>
}
pub struct Neutral{
}

pub enum TypeObject<T:NdFloat>{
    Neutral(Neutral),
    Receiver(Receiver<T>),
    Transmitter(Transmitter<T>),
    Transceiver(Transceiver<T>),
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
pub trait CanReceive<T:NdFloat>{
    fn add_ray(&self){}
}

impl<'a,T:NdFloat> Object<'a,T>{
    pub fn ray_received(&self,ray: &Ray<T>,dist:T,intersection:Array1<T>)->(T,T,Array1<T>,Array1<T>){
        let new_amplitude;
        match &self.type_object{
            TypeObject::Neutral(_neut)=> new_amplitude = T::one()/(dist*dist),
            TypeObject::Receiver(rec)=> {   
                rec.add_ray();
                new_amplitude = T::zero()},
            TypeObject::Transceiver(transc)=> {   
                transc.add_ray();
                new_amplitude = T::zero()},
            TypeObject::Transmitter(_transm)=> new_amplitude = T::zero(),
        }
        let new_direction = &ray.direction*(-T::one());
        let out_ray= (new_amplitude*ray.amplitude, dist+ray.distance, intersection, new_direction);
        out_ray
    }
}


impl<T:NdFloat> CanTransmit for Transmitter<T>{}
impl<T:NdFloat> CanReceive<T> for Receiver<T>{}

impl<T:NdFloat> CanTransmit for Transceiver<T>{}
impl<T:NdFloat> CanReceive<T> for Transceiver<T>{}

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
    pub fn new(points: Vec<&'a Array1<T>>,indices:Array2<usize>,movable:bool,type_object:TypeObject<T>)->Self{
        let mut faces = Vec::new();
        for n_point in 0..indices.dim().0{
            let p1 = points[indices[[n_point,0]]];
            let p2 = points[indices[[n_point,1]]];
            let p3 = points[indices[[n_point,2]]];
            let face = Face::new((p1,p2,p3));
            faces.push(face);
        }
        Object{
            faces,
            movable,
            type_object,
        }
    }
}