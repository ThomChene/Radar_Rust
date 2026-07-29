use ndarray::NdFloat;
use std::fmt;
use super::object::{CanTransmit, Object};
use super::ray::Ray;

pub fn raytracing<'a,T,U>(n_rays: u32, min_amplitude_raytracing: T, transmitter: &U, group_object: &Vec<Object<'a,T>>)->fmt::Result
where
T:NdFloat,
U:CanTransmit,
{
    let mut rays: Vec<Ray<T>> = transmitter.create_rays(n_rays);
    // Can be parallelized
    for ray in rays.iter_mut(){
        let new_propagation = true;
        while new_propagation && ray.amplitude>min_amplitude_raytracing{
            ray.propagate(group_object);
        }
    }
    Ok(())
}