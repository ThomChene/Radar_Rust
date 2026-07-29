//use ndarray::{arr1, arr2, Array1, Array2};
//use radar_and_tomography::map::raytracing;
//use radar_and_tomography::map::object::Object;
// Todo:
// Check R star
// faire une struct point ou bien vérifier que les valeurs soient en 3D 
// movable object are reconstructed at each instant
// conditions pour lesquelles face est une ligne en fonction du cross product
// add sized to faces?
// look at rayon

fn main() {
    let n_rays = 100;
    let min_amplitude_raytracing = 1e-6;

    // Create map with unmovable objects
    //let group_object = Vec::new();

    let max_x_i = 10;
    let max_y_i = 10;
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
            let position = (x,y,z);
            positions.push(position);
        }
    } 

    //let vec_impulsional_response = Vec::new();
    for position in positions{
        // Create transmitter
        //let transceiver = Object::new();
        //raytracing::raytracing(n_rays, min_amplitude_raytracing, &trans, &group_object);

        //vec_impulsional_response.push(transceiver.impulsional_response());
    }
}