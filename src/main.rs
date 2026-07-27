use ndarray::{arr1, arr2, Array1, Array2};
use radar_and_tomography::map::face::Face;
use radar_and_tomography::map::object::Object;
// Todo:
// Check R star
// faire une struct point ou bien vérifier que les valeurs soient en 3D 
// movable object are reconstructed at each instant
// conditions pour lesquelles face est une ligne en fonction du cross product

fn main() {
    // Create map with unmovable objects

    // Create transceiver
    // look at rayon
    // for positions transceiver
    // transceiver = pos
    // raytracing transceiver
    // retrieve filter (a signal)
    // store it into a vec
    // filter conv signal emitted by transmitter
    // 

    let p1 = arr1(&[1.0,0.0,0.0]);
    let p2 = arr1(&[0.0,1.0,0.0]);
    let p3 = arr1(&[0.0,0.0,1.0]);
    
    let face = Face::new((&p1,&p2,&p3));
    println!("{face}");

    let p4 = arr1(&[0.0,1.0,1.0]);
    let indices = arr2(&[[0,1,2],[0,1,3]]);
    let object = Object::new(Vec::from([&p1,&p2,&p3,&p4]),indices,true);

    println!("{object}");
}