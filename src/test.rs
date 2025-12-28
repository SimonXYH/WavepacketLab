use ndarray::{Array1, Array2, ShapeBuilder};
use num_complex::Complex32;

pub fn d2(n: usize) -> Array2<Complex32> {
    let mut d2 = Array2::<Complex32>::zeros((n,n));
    for i in 0..n {
        for j in 0..n {
            if i == j {
                d2[[i,j]] = Complex32::new(-2.,0.);
            }
            if i == j - 1 || i == j + 1{
                d2[[i,j]] = Complex32::new(1.,0.0);
            }
        }
    }
    d2
}

pub fn main(){
    println!("{}", d2(10))
    
}



