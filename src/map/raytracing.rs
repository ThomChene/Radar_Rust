use ndarray::NdFloat;
use std::fmt;
use super::object::{CanTransmit, GroupObject};
use super::ray::Ray;

fn raytracing<'a,T,U>(n_rays:u32, transmitter: &U, group_object: &GroupObject)->fmt::Result
where
T:NdFloat,
U:CanTransmit,
{
    let rays: Vec<Ray<T>> = transmitter.create_rays(n_rays);
    let min_amplitude_raytracing = 1e-6;
    // Can be parallelized
    for ray in rays.iter_mut(){
        let new_propagation = true;
        let amplitude_ray = 1.0;
        while new_propagation && amplitude_ray>min_amplitude_raytracing{
            // Which sub group of objects can potentially be hit
            sub_objects: Vec<&Object> = f(group_object,ray);
            (object_touched,index_object,new_origin,new_direction) = ray.propagate(sub_objects);
            if object_touched{
                object = g(i,group_object);
                (new_propagation,new_origin,new_direction,amplitude_ray): (bool,Array,Array,T)= object.activate(); // Properties of the object touched
                ray = Ray::(new_origin,new_direction,amplitude_ray);
            }
        }
    }
    Ok(())
}