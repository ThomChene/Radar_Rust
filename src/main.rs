use ndarray::{arr1, arr2};
use radar_and_tomography::algorithms::beamforming::Beamforming;
use radar_and_tomography::map::face::{Neutral, Receiver, TypeFace};
use radar_and_tomography::map::raytracing;
use radar_and_tomography::plot::plot_map::plot_map;
use radar_and_tomography::{algorithms::signal::Signal, map::object::{Object,Transmitter}};
use std::f64::consts::PI as Pi;

// Todo:
// Check R star
// faire une struct point ou bien vérifier que les valeurs soient en 3D 
// conditions pour lesquelles face est une ligne en fonction du cross product
// add sized to faces?
// look at rayon
// finish create rays with beamforming
// add objects
// plot objects
// plot rays

fn main() {
    let n_receiver = 1;
    let n_rays = 10;
    let min_amplitude_ray = 1e-12;

    // Create map with unmovable objects
    let mut map = Vec::new();

    let x_1 = -1000.0;
    let x_2 = 1000.0;
    let y_1 = -1000.0;
    let y_2 = 1000.0;
    let z_1 = 0.0;

    let p1 = arr1(&[x_1,y_1,z_1]);
    let p2 = arr1(&[x_2,y_1,z_1]);
    let p3 = arr1(&[x_1,y_2,z_1]);

    let neutral = Neutral::new(1.0);
    let neutral_points = Object::parallelogram_points(vec!(p1,p2,p3));
    let neutral_object = Object::new(vec!(neutral_points), vec!(TypeFace::Neutral(neutral)));

    map.push(neutral_object);

    let length_x_transceiver = 10.0;
    let length_y_transceiver = 10.0;
    let wavelength = 0.1;
    let precision = (100,100);
    let theta_transceiver = Pi*4.0/5.0;
    let phi_transceiver = Pi/5.0;

    let max_x_i = 2;
    let max_y_i = 1;
    let min_x = 0.0;
    let max_x = 100.0;
    let min_y = 0.0;
    let max_y = 100.0;
    let z = 100.0;
    let mut positions = Vec::new();
    for x_i in 0..max_x_i{
        for y_i in 0..max_y_i{
            let x = min_x + (max_x - min_x)*f64::from(x_i/max_x_i);
            let y = min_y + (max_y - min_y)*f64::from(y_i/max_y_i);

            // Matrices to apply a rotation around axis y, then around z
            let mat_rotation_y = arr2(&[
                [theta_transceiver.cos(),0.0,theta_transceiver.sin()],
                [0.0,1.0,0.0],
                [-theta_transceiver.sin(),0.0,theta_transceiver.cos()]]);
            let mat_rotation_z = arr2(&[
                [phi_transceiver.cos(),-phi_transceiver.sin(),0.0],
                [phi_transceiver.sin(),phi_transceiver.cos(),0.0],
                [0.0,0.0,1.0]]);

            let positions_3_points = arr2(&[
                [-length_x_transceiver/2.0,-length_y_transceiver/2.0,0.0],
                [length_x_transceiver/2.0,-length_y_transceiver/2.0,0.0],
                [-length_x_transceiver/2.0,length_y_transceiver/2.0,0.0]]);

            // Center of the transceiver
            let position_center = arr1(&[x,y,z]);

            let positions_3_points_rotated = positions_3_points.dot(&mat_rotation_y.t()).dot(&mat_rotation_z.t());

            let (vec,_) = positions_3_points_rotated.into_raw_vec_and_offset();

            let position_bottom_left = arr1(&vec[0..3]) + &position_center;
            let position_bottom_right = arr1(&vec[3..6]) + &position_center;
            let position_top_right = arr1(&vec[6..9]) + &position_center;

            let beamforming = Beamforming::new_rectangular_antenna(length_x_transceiver, length_y_transceiver, wavelength, precision);

            positions.push((position_center, (phi_transceiver, theta_transceiver), vec!(position_bottom_left, position_bottom_right, position_top_right), beamforming));
        }
    } 

    //let vec_impulsional_response = Vec::new();
    for position in positions{
        let (position_center, phi_theta, position_receiver, beamforming) = position;
        // Create transmitter
        let _signal_received = Signal::new(1.0);

        let transmitter = Transmitter::new(beamforming, position_center, phi_theta);

        let receiver = Receiver::new(0);
        let receiver_points = Object::parallelogram_points(position_receiver);

        let receiver_object = Object::new(vec!(receiver_points), vec!(TypeFace::Receiver(receiver)));

        map.push(receiver_object);

        let (_all_amplitudes, _all_distances, coordinates_rays_to_plot) = raytracing::raytracing(n_rays, n_receiver, &transmitter, &map, min_amplitude_ray);

        //println!("{:?}\n",coordinates_rays_to_plot);

        map.pop();

        //vec_impulsional_response.push(transceiver.impulsional_response());
    }
    let _x = plot_map();
}