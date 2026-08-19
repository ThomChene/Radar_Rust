use ndarray::{arr1, arr2, Array1, Array2, NdFloat};
use std::fmt;
use rand::RngExt;

use crate::algorithms::beamforming::Beamforming;
use super::ray::Ray;
use super::face::{Face, GroupFaces, TypeFace};
/// An `Object` contains one or multiple `GroupFaces`, and each `GroupFaces` contains one or multiple `Face` that have the same `TypeFace`.
/// An `Object` can move either by translation or rotation. Every point constituting the `Object` will move following the same motion. When points are moved, faces will be updated accordingly.
/// An `Object` contains one or multiple `GroupPoints`, even though each face contains points as well which is redundant, each `Face` is self sufficient and no extra information is required to run raytracing on it.
/// 
pub struct Object<T:NdFloat>{
    group_faces: Vec<GroupFaces<T>>,
    group_points: Vec<GroupPoints<T>>,
}

impl<T:NdFloat> Object<T>{
    pub fn get_group_faces(&self)->&Vec<GroupFaces<T>>{
        &self.group_faces
    }
}
/// `GroupPoint`
pub struct GroupPoints<T:NdFloat>{
    points: Vec<Array1<T>>,
    indices: Array2<usize>,
}

impl<T:NdFloat> GroupPoints<T>{
    pub fn get_point(&self,i:usize)->&Array1<T>{
        &self.points[i]
    }
    pub fn get_indices(&self,n: usize)->(usize,usize,usize){
        (self.indices[[n,0]],self.indices[[n,1]],self.indices[[n,2]])
    }
    pub fn get_number_faces(&self)->usize{
        self.indices.dim().0
    }
    fn move_points(&mut self, vector: &Array1<T>){
        for mut point in self.points.iter(){
            point = &(vector + point);
        }
    }
}

pub fn from_objects_to_faces<T:NdFloat>(objects: &Vec<Object<T>>)->Vec<&Face<T>>{
    let mut vec_faces = Vec::new();
    for object in objects.iter(){
        for group_faces in object.group_faces.iter(){
            vec_faces.extend(group_faces.get_faces());
        }
    }
    vec_faces
}

/// `Transmitter`
pub struct Transmitter<T:NdFloat>{
    beamforming: Beamforming<T>,
    origin_rays: Array1<T>,
    phi_theta_center_direction_ray: (T,T),
}

impl<T:NdFloat> Transmitter<T>{
    pub fn new(beamforming: Beamforming<T>, origin_rays: Array1<T>, phi_theta_center_direction_ray: (T,T))->Self{
        Transmitter{
            beamforming,
            origin_rays,
            phi_theta_center_direction_ray,
        }
    }
    /// Accept/Reject sampling to generate rays according to the beam pattern 
    pub fn create_rays(&self, n_rays: u32, min_amplitude_ray: T)->Vec<Ray<T>>{
        // Rays are centered around this direction
        let (phi_center, theta_center) = self.phi_theta_center_direction_ray;

        // Generate randomly rays around the center
        let mut rng = rand::rng();

        let pattern = self.beamforming.get_pattern();
        let (n_phi, n_theta, min_phi, max_phi, min_theta, max_theta, max_abs_value_pattern) = self.beamforming.get_values();
        let n_unif = 1000;

        let mut n_accepted_rays = 0;
        let mut rays = Vec::new();
        while n_accepted_rays<n_rays {
            let rand_int_phi = rng.random_range(0..n_phi);
            let rand_int_theta = rng.random_range(0..n_theta);

            let option_rand_phi = T::from(rand_int_phi);
            let option_n_phi = T::from(n_phi);
            let option_rand_theta = T::from(rand_int_theta);
            let option_n_theta = T::from(n_theta);
            let option_rand_unif = T::from(rng.random_range(0..n_unif));
            let option_n_unif = T ::from(n_unif);

            let (rand_unif, rand_phi, rand_theta);
            match (option_rand_unif, option_n_unif, option_rand_phi, option_n_phi, option_rand_theta, option_n_theta){
                (Some(unif), Some(n_unif), Some(phi), Some(n_phi), Some(theta), Some(n_theta)) => {
                    rand_unif = unif/n_unif;
                    rand_phi = phi/n_phi;
                    rand_theta = theta/n_theta;},
                _ => panic!("usize/usize cannot be converted to type T"),
            }

            if rand_unif < T::abs(pattern[rand_int_theta as usize][rand_int_phi as usize])/max_abs_value_pattern{
                // Accepts the ray / else rejects it
                let theta_radian = min_theta + (max_theta-min_theta)*rand_theta + theta_center;
                let phi_radian = min_phi + (max_phi-min_phi)*rand_phi + phi_center;

                let direction = arr1(&[T::zero(),T::zero(),T::one()]);

                let mat_rotation_y = arr2(&[
                    [theta_radian.cos(),T::zero(),theta_radian.sin()],
                    [T::zero(),T::one(),T::zero()],
                    [-theta_radian.sin(),T::zero(),theta_radian.cos()]]);
                let mat_rotation_z = arr2(&[
                    [phi_radian.cos(),-phi_radian.sin(),T::zero()],
                    [phi_radian.sin(),phi_radian.cos(),T::zero()],
                    [T::zero(),T::zero(),T::one()]]);

                let direction = direction.dot(&mat_rotation_y.t()).dot(&mat_rotation_z.t());
                let (vec_direction,_) = direction.into_raw_vec_and_offset();
                let direction = arr1(&vec_direction);

                let ray = Ray::new(T::one(), T::zero(), self.origin_rays.clone(), direction, min_amplitude_ray);

                rays.push(ray);
                n_accepted_rays = n_accepted_rays + 1;
            }
        }
        rays
    }
}

/// Display:
/// 
impl<'a,T:NdFloat> fmt::Display for Object<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This Object with faces:\n")?;
        for group_face in &self.group_faces{
            let vec_faces = group_face.get_faces();
            for v in vec_faces.iter() {
                write!(f, "p1: {}\n", v)?;
            }
        }
        write!(f, "")
    }
}

impl<'a,T:NdFloat> Object<T>{
    pub fn new(group_points: Vec<GroupPoints<T>>, type_face: Vec<TypeFace<T>>)->Self{
        let mut group_faces = Vec::new();
        for (i,group_point) in group_points.iter().enumerate(){
            let group_face = GroupFaces::new(group_point, type_face[i]);
            group_faces.push(group_face);
        }
        let object = Object{
            group_faces,
            group_points,
        };
        object
    }

    pub fn move_object(&mut self,vector:&Array1<T>){
        for group_point in self.group_points.iter_mut(){
            group_point.move_points(vector);
        }
    }

    /// Builds a parallelogram from 3 points
    pub fn parallelogram_points(mut points: Vec<Array1<T>>)->GroupPoints<T>{
        assert!(points.len()==3);
        points.push(&points[1]-&points[0]+&points[2]);
        let indices = arr2(&[[0,1,3],[3,2,0]]);
        GroupPoints{
            points,
            indices,
        }
    }

    /// Builds a parallelepiped from 4 points
    pub fn parallelepiped_points(mut points: Vec<Array1<T>>)->GroupPoints<T>{
        points.push(&points[1]-&points[0]+&points[2]);
        points.push(&points[1]-&points[0]+&points[3]);
        points.push(&points[2]-&points[0]+&points[5]);
        points.push(&points[2]-&points[0]+&points[3]);

        let indices = arr2(&[
            [0,1,4],[4,2,0],
            [2,4,6],[6,7,2],
            [0,1,5],[5,3,0],
            [3,5,6],[6,7,3],
            [1,5,6],[6,4,1],
            [0,3,7],[7,2,0]]);
        GroupPoints{
            points,
            indices,
        }
    }
}
