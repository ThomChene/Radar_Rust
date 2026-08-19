use ndarray::{NdFloat, Array1};
use super::object::{Object, Transmitter};
use super::object;
use super::ray::Ray;

pub fn raytracing<T>(n_rays: u32, n_receiver: u32, transmitter: &Transmitter<T>, objects: &Vec<Object<T>>, min_amplitude_ray: T)->(Vec<Vec<T>>, Vec<Vec<T>>, Vec<(Array1<T>,Array1<T>)>)
where
T:NdFloat,
{
    let mut rays: Vec<Ray<T>> = transmitter.create_rays(n_rays, min_amplitude_ray);

    let mut all_amplitudes_receivers = Vec::new();
    let mut all_distances_receivers = Vec::new();
    let mut to_plot_rays = Vec::new();
    for _receiver in 0..n_receiver{
        all_amplitudes_receivers.push(Vec::new());
        all_distances_receivers.push(Vec::new());
    }
    let faces = object::from_objects_to_faces(objects);
    while !rays.is_empty(){
        // Should be parallelized
        let option_ray  = rays.pop();
        // faces = face_tree(ray,faces);
        match option_ray{
            Some(ray) => {
                // To plot rays
                let (_,_,origin,_,_) = ray.get_values();
                let begin_ray = (origin[0],origin[1],origin[2]);
                // Propagate the ray
                let (rays_reflected, option_received) = ray.propagate(&faces);

                // To plot rays
                if !rays_reflected.is_empty(){
                    let (_,_,end,_,_) = (rays_reflected[0]).get_values();
                    let end_ray: (T, T, T) = (end[0],end[1],end[2]);
                    to_plot_rays.push((begin_ray,end_ray));
                }

                for new_ray in rays_reflected{
                    rays.push(new_ray);
                }
                match option_received{
                    Some((amplitude,distance,index_receiver)) =>{
                        all_amplitudes_receivers[index_receiver].push(amplitude);
                        all_distances_receivers[index_receiver].push(distance);}
                    None => (),
                }
            }
            None =>(),
    }
    }
    (all_amplitudes_receivers,all_distances_receivers,to_plot_rays)
}