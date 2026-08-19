use std::f64::consts::PI as Pi_64;

use ndarray::NdFloat;

fn sinc<T:NdFloat>(x:T)->Result<T,T>{
    if x==T::zero(){
        Ok(T::zero())
    }
    else{
        let pi_opt = T::from(Pi_64);
        match pi_opt{
            Some(pi) => Ok(T::sin(pi*x)/(pi*x)),
            None => Err(T::zero()),
        }
    }
}

pub struct Beamforming<T>
{
    beam_pattern: Vec<Vec<T>>,
    n_phi: u32,
    n_theta: u32,
    min_phi: T,
    max_phi: T,
    min_theta: T,
    max_theta: T,
    max_abs_value_pattern: T,
}


impl<T:NdFloat> Beamforming<T>{
    /// Creates a matrix with the intensity of the beam. It is in spheric coordinates(radians).
    /// For a rectangular antenna with dimensions L and l the beam shape in the direction normal to the antenna is in sinc.
    /// The pattern is only computed in a relevant region around the main lobe.
    pub fn new_rectangular_antenna(l_x:T, l_y:T, wavelength:T, precision:(u32,u32))->Self{
        let pi_opt = T::from(Pi_64);
        let pi;
        match pi_opt{
            Some(pi_t) => pi = pi_t,
            None => panic!("f64 cannot be converted to type T"),
        }

        // Min values and Max values around the main lobe
        let min_phi = -pi;
        let max_phi = pi;
        let min_theta = T::zero();
        let max_theta;
        let l;
        let mut max_abs_value_pattern = T::min_value();
        match l_x<l_y{
            true => l = l_x,
            false => l = l_y,
        }

        match pi<pi*wavelength/l{
            true => max_theta = pi,
            false => max_theta = pi*wavelength/l,
        }

        let (max_row, max_col) = precision;
        let mut beam_pattern = Vec::new();
        for row in 0..max_row{
            let mut vec_col = Vec::new();
            for col in 0..max_col{
                let opt_col = T::from(col);
                let opt_max_col = T::from(max_col);
                let opt_row = T::from(row);
                let opt_max_row = T::from(max_row);

                let (phi_radian,theta_radian);
                match (opt_col, opt_max_col, opt_row, opt_max_row){
                    (Some(col_norm), Some(max_col), Some(row_norm), Some(max_row)) => {
                        phi_radian = (max_phi-min_phi)*col_norm/max_col + min_phi;
                        theta_radian = (max_theta-min_theta)*T::acos(row_norm/max_row + row_norm/max_row - T::one())/pi+ min_theta;},
                    _ => panic!("Type of L*y/wavelength cannot be converted to type T"),
                }
                let x = T::sin(theta_radian)*T::cos(phi_radian);
                let y = T::sin(theta_radian)*T::sin(phi_radian);

                let opt_sinc_x = sinc(l_x*x/wavelength);
                let opt_sinc_y = sinc(l_y*y/wavelength);
                match (opt_sinc_x,opt_sinc_y){
                    (Ok(sinc_x),Ok(sinc_y)) => {
                        if max_abs_value_pattern<T::abs(sinc_x*sinc_y){
                            max_abs_value_pattern = T::abs(sinc_x*sinc_y);
                        }
                        vec_col.push(sinc_x*sinc_y);},
                    _ => panic!("Type of L*y/wavelength cannot be converted to type T"),
                }
            }
            beam_pattern.push(vec_col);
        }

        let beamforming = Beamforming
        {
            beam_pattern,
            n_theta: max_row,
            n_phi: max_col,
            min_phi,
            max_phi,
            min_theta,
            max_theta,
            max_abs_value_pattern,
        };
        beamforming
    }

    pub fn get_pattern(&self)->&Vec<Vec<T>>{
        &self.beam_pattern
    }

    pub fn get_values(&self)->(u32,u32,T,T,T,T,T){
        (self.n_phi, self.n_theta, self.min_phi, self.max_phi, self.min_theta, self.max_theta, self.max_abs_value_pattern)
    }
}