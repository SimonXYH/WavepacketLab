use std::f32::consts::PI;
use ndarray::{Array, Array1, Array2};
use num::traits::real::Real;
use num_complex::{Complex32, Complex64, ComplexFloat};

const H_BAR: f32 = 1.;
const H_BAR_SQ: f32 = H_BAR * H_BAR; 

pub struct WaveVector{
    pub base: Array1<f32>,
    pub values: Array1<Complex32>,
    pub mass: f32,
    pub d2_matrix: Array2<Complex32>,
    pub probability: Array1<f32>,
}

pub fn gaussian(x:f32, center: f32, std_dev: f32) -> Complex32 {
    Complex32::new((1./(2. * PI * std_dev)).powf(0.25) * (-(x - center).powi(2) /(4. * std_dev.powi(2))).exp(),0.)
}

pub fn gaussian_pulse(base: &Array1<f32>, center: f32, std_dev: f32) -> Array1<Complex32> {
    let v = base.iter().map(|x| gaussian(*x, center, std_dev)).collect();
    Array1::from_vec(v)
    
}

pub fn d2_matrix(n:usize, dx:f32) -> Array2<Complex32> {
    let mut d2 = Array2::<Complex32>::zeros((n, n));
    let k = 1./(dx * dx);
    for i in 0..n {
        for j in 0..n {
            if i == j {
                d2[[i,j]] = Complex32::new(-2. * k, 0.0);
            }
            if i == j - 1 || i == j + 1{
                d2[[i,j]] = Complex32::new(k, 0.0);
            } 
        }
    }
    d2
}

pub fn base_new(start:f32, end:f32, res: usize) -> Array1<f32> {
    Array1::linspace(start, end, res)
}

impl WaveVector{
    pub fn new(start: f32, end: f32, res: usize, base: Array1<f32>,values: Array1<Complex32>, mass: f32) -> WaveVector{
        let dx = (end - start)/res as f32;
        let probability = values.map(|x| x.abs().powi(2));
        WaveVector{base, values, mass,
        d2_matrix: d2_matrix(res, dx), probability}
    }
    
    pub fn vel(&self, values: &Array1<Complex32>,  potential: &Array1<Complex32>) ->  Array1<Complex32> {
        let p: f32 = -H_BAR_SQ / (2. * self.mass);
        -Complex32::i() / H_BAR * (self.d2_matrix.dot(values) * p + potential * values)
    }
    
    pub fn update_rk4(&mut self, potential: &Array1<Complex32>, dt: f32){
        let v1 =  self.vel(&self.values, potential);
        let v2 =  self.vel(&(&self.values + &v1 * 0.5 * dt), potential);
        let v3 = self.vel(&(&self.values + &v2 * 0.5 * dt), potential);
        let v4 = self.vel(&(&self.values + &v3 * dt), potential);
        let vf = (v1 + v2 * 2. + v3 * 2. + v4)/6.;
        self.values += &(vf * dt);
        self.probability = self.values.map(|x| x.abs().powi(2));
    }
}

pub fn harmonic_oscillator(x: f32, mass: f32,  omega: f32) -> Complex32 {
    Complex32::new(0.5 * mass * omega.powi(2) * x.powi(2),0.) 
    
}

