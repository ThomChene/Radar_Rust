use ndarray::{arr1, arr2, Array1, Array2};
use radar_and_tomography::map::face::Face;
// Todo:
// Check R star
// faire une struct point ou bien vérifier que les valeurs soient en 3D 
// movable object are reconstructed at each instant

fn main() {
    let p1 = arr1(&[1.0,0.0,0.0]);
    let p2 = arr1(&[0.0,1.0,0.0]);
    let p3 = arr1(&[0.0,0.0,1.0]);
    
    let face1 = Face::new((&p1,&p2,&p3));
}