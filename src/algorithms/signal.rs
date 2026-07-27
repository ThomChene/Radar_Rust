use ndarray::{Array1, NdFloat};

pub struct Signal<T>{
    amplitude:T
}

pub fn l2_norm<T:NdFloat>(x: &Array1<T>) -> T {
    x.view();
    x.dot(x).sqrt()
}

pub fn scalar_prod<T:NdFloat>(x: &Array1<T>,y: &Array1<T>) -> T {
    x.view();
    y.view();
    x.dot(y)
}