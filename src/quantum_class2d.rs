use builder_pattern::Builder;
use ndarray::{Array2, Zip};
use num::traits::float::FloatCore;
use num_complex::{Complex32, ComplexFloat};
use rayon::prelude::*;
use std::f32::consts::PI;

const HBAR: f32 = 1.;
const HBAR_SQ: f32 = HBAR * HBAR;

pub struct WaveVector2D {
    pub grid: Vec<Vec<(f32, f32)>>,
    pub values: Array2<Complex32>,
    pub mass: f32,
    //  pub d2_matrix: Array2<Complex32>,
    pub probability: Array2<f32>,
    pub shape: (usize, usize),
}

pub fn create_grid(
    center: (f32, f32),
    width: f32,
    height: f32,
    shape: (usize, usize),
) -> Vec<Vec<(f32, f32)>> {
    let mut grid = vec![vec![(0., 0.); shape.1]; shape.0];
    for i in 0..shape.0 {
        for j in 0..shape.1 {
            let x = center.0
                + width * ((i as f32 - (shape.0 as f32 - 1.) / 2.) / (shape.0 as f32 - 1.));
            let y = center.1
                + height * ((j as f32 - (shape.1 as f32 - 1.) / 2.) / (shape.1 as f32 - 1.));
            grid[i][j] = (x, y);
        }
    }
    grid
}

pub fn square_modulus(value: &Array2<Complex32>) -> Array2<f32> {
    value.map(|x| x.abs().powi(2))
}

pub fn d2_matrix(shape: (usize, usize), dx: f32, dy: f32) -> Array2<Complex32> {
    todo!()
}

pub fn d2(value: &Array2<Complex32>) -> Array2<Complex32> {
    todo!()
}

pub fn d2_elementwise(
    values: &Array2<Complex32>,
    shape: (usize, usize),
    dx: f32,
    dy: f32,
) -> Array2<Complex32> {
    let kernel = [[0., 1., 0.], [1., -2., 1.], [0., 1., 0.]];
    let dx_sq = dx * dx;
    let dy_sq = dy * dy;
    let (rows, cols) = shape;
    let mut result = Array2::<Complex32>::zeros(shape);
    Zip::indexed(values.windows((3, 3))).for_each(|(i, j), window| {
        let v = (window[[0, 1]] + window[[2, 1]]) / dy_sq
            + (window[[1, 0]] + window[[1, 2]]) / dx_sq
            - window[[1, 1]] * (2.0 / dx_sq + 2.0 / dy_sq);
        result[[i + 1, j + 1]] = v;
    });

    result
}

impl WaveVector2D {
    pub fn new(grid: Vec<Vec<(f32, f32)>>, values: Array2<Complex32>, mass: f32) -> WaveVector2D {
        let probability = square_modulus(&values);
        let shape = (grid.len(), grid[0].len());
        // let d2_matrix = todo!();
        WaveVector2D {
            grid,
            values,
            mass,
            probability,
            shape,
        }
    }

    pub fn vel(
        &self,
        values: &Array2<Complex32>,
        potential: &Array2<Complex32>,
        dx: f32,
        dy: f32,
    ) -> Array2<Complex32> {
        let p: f32 = -HBAR_SQ / (2. * self.mass);
        let values_d2 = d2_elementwise(values, self.shape, dx, dy);
        -Complex32::i() / HBAR * (values_d2 * p + potential * values)
    }

    pub fn update_rk4(&mut self, potential: &Array2<Complex32>, dt: f32, dx: f32, dy: f32) {
        let v1 = self.vel(&self.values, potential, dx, dy);
        let v2 = self.vel(&(&self.values + &v1 * 0.5 * dt), potential, dx, dy);
        let v3 = self.vel(&(&self.values + &v2 * 0.5 * dt), potential, dx, dy);
        let v4 = self.vel(&(&self.values + &v3 * dt), potential, dx, dy);
        let vf = (v1 + v2 * 2. + v3 * 2. + v4) / 6.;
        self.values += &(vf * dt);
        self.probability = self.values.map(|x| x.abs().powi(2));
    }
}

pub fn gaussian2d(pos: (f32, f32), std_dev: (f32, f32), center: (f32, f32)) -> f32 {
    let factor = 1. / (PI * std_dev.0 * std_dev.1).sqrt();
    factor
        * (-(pos.0 - center.0).powi(2) / (2. * std_dev.0 * std_dev.0)
            - (pos.1 - center.1).powi(2) / (2. * std_dev.1 * std_dev.1))
            .exp()
}
pub fn gaussian2d_pulse(
    std_dev: (f32, f32),
    center: (f32, f32),
    base: &Vec<Vec<(f32, f32)>>,
) -> Array2<Complex32> {
    let row_num = base.len();
    let col_num = base[0].len();
    Array2::from_shape_fn((row_num, col_num), |(i, j)| {
        let pos = base[i][j];
        let gaussian_value = gaussian2d(pos, std_dev, center);
        Complex32::new(gaussian_value, 0.0)
    })
}

pub fn harmonic_oscialltor_2d(
    x: f32,
    y: f32,
    center: (f32, f32),
    omega: f32,
    mass: f32,
) -> Complex32 {
    let value = 0.5 * mass * omega.powi(2) * ((x - center.0).powi(2) + (y - center.1).powi(2));
    Complex32::new(value, 0.)
}

#[derive(Builder)]
pub struct DoubleSlit {
    pub center: (f32, f32),
    pub height: f32,
    pub angle: f32,
    pub thickness: f32,
    pub slit_width: f32,
    pub slit_spacing: f32,
}

impl DoubleSlit {
    pub fn new_slit(
        center: (f32, f32),
        height: f32,
        angle: f32,
        thickness: f32,
        slit_width: f32,
        slit_spacing: f32,
    ) -> DoubleSlit {
        DoubleSlit {
            center,
            height,
            angle,
            thickness,
            slit_width,
            slit_spacing,
        }
    }

    pub fn potential_value(&self, x: f32, y: f32) -> Complex32 {
        let (x, y) = (x - self.center.0, y - self.center.1);
        let (a_cos, a_sin) = (self.angle.cos(), self.angle.sin());
        let vec = (a_cos * x + a_sin * y) / self.thickness;
        let wave = self.height * vec.cos() / vec.cos().abs() * ((vec.abs() < 0.5) as i32) as f32;
        let perp_vec = (-a_cos * y + a_sin * x).abs();
        let value = wave
            * ((perp_vec < self.slit_spacing) as i32 as f32
                + (perp_vec > self.slit_width + self.slit_spacing) as i32 as f32);

        // let gaussian = (-(a_cos * x + a_sin * y).powi(2)).exp() * self.height;
        //  let perp_dist = (-a_cos * y + a_sin * x).abs();
        // let value = ((perp_dist < self.slit_spacing) as i32
        //     + (perp_dist > self.slit_spacing + self.slit_width) as i32) as f32
        //     * gaussian;
        Complex32::new(value, 0.)
    }
}
#[derive(Builder)]
pub struct StepPotential {
    pub center: (f32, f32),
    pub height: f32,
    pub angle: f32,
    pub thickness: f32,
}

impl StepPotential {
    pub fn new_step(center: (f32, f32), height: f32, angle: f32, thickness: f32) -> StepPotential {
        StepPotential {
            center,
            height,
            angle,
            thickness,
        }
    }
    pub fn potential_value(&self, x: f32, y: f32) -> Complex32 {
        let (x, y) = (x - self.center.0, y - self.center.1);
        let (a_cos, a_sin) = (self.angle.cos(), self.angle.sin());
        let vec = (a_cos * x + a_sin * y) / self.thickness;
        let value = vec.cos() / vec.cos().abs() * ((vec.abs() < 0.5) as i32) as f32;
        Complex32::new(value, 0.)
    }
}

#[derive(Builder)]
pub struct RampPotential {
    pub thickness: f32,
    pub width: f32,
    pub gradient: f32,
    pub vertical_offset: f32,
    pub angle: f32,
    pub center: (f32, f32),
}

impl RampPotential {
    pub fn new_ramp(
        thickness: f32,
        width: f32,
        gradient: f32,
        vertical_offset: f32,
        angle: f32,
        center: (f32, f32),
    ) -> RampPotential {
        RampPotential {
            thickness,
            width,
            gradient,
            vertical_offset,
            angle,
            center,
        }
    }

    pub fn potential_value(&self, x: f32, y: f32) -> Complex32 {
        let (x, y) = (x - self.center.0, y - self.center.1);
        let (a_cos, a_sin) = (self.angle.cos(), self.angle.sin());
        let vec = (a_cos * x + a_sin * y);
        let perp = (-a_cos * y + a_sin * x);
        let value = (self.vertical_offset + self.gradient * vec)
            * ((vec.abs() < self.thickness) as i32) as f32
            * (((perp < self.width) as i32) as f32);
        Complex32::new(value, 0.)
    }
}
