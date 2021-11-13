
#![allow(non_snake_case)]
//use ndarray::parallel::prelude::*;
use ndarray::{Array2, Axis};
use num_complex::Complex;
use num_traits::{Float, FloatConst};

pub fn fftfreq<T>(n: usize) -> Vec<T>
where
    T: Float,
{
    let n = n as isize;
    let result = (0..=((n - 1) / 2))
        .chain(-n / 2..=-1)
        .map(|x| T::from(x).unwrap() / T::from(n).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(result.len(), n as usize);
    result
}



//y+:=az=0deg
//x+:=az=90deg
pub fn angle2xyz<T>(azimuth: T, zenith: T)->[T;3]
where T: Float + FloatConst{
    let ca=azimuth.cos();
    let sa=azimuth.sin();
    let cz=zenith.cos();
    let sz=zenith.sin();

    let x=sz*sa;
    let y=sz*ca;
    let z=cz;

    [x,y,z]
}

pub fn apply_delay<T>(x: &mut Array2<Complex<T>>, d: T)
where
    T: Float + Copy + FloatConst + std::fmt::Debug,
{
    let two = T::one() + T::one();
    let freqs = fftfreq::<T>(x.shape()[0]);

    for (r, k) in x.axis_iter_mut(Axis(0)).zip(
        freqs
            .into_iter()
            .map(|f| Complex::<T>::new(T::zero(), -two * T::PI() * f * d).exp()),
    ) {
        for x1 in r {
            *x1 = *x1 * k;
        }
    }
}

pub fn apply_delay_with_shift<T>(x: &mut Array2<Complex<T>>, d: T, i: usize, n: usize)
where
    T: Float + Copy + FloatConst + std::fmt::Debug,
{
    let two = T::one() + T::one();
    let freqs = fftfreq::<T>(x.shape()[0]);
    let df=freqs[1];
    assert!(n%2==0);
    let ddf=df/T::from(n).unwrap();
    let fshift=ddf*(T::from(i).unwrap()-T::from(n/2).unwrap()+T::one()/two);
    //println!("{} {:?} {:?} {:?}",i, fshift, freqs[0], freqs[1]);
    for (r, k) in x.rows_mut().into_iter().zip(
        freqs
            .into_iter()
            .map(|f| Complex::<T>::new(T::zero(), two * T::PI() * (f+fshift) * d).exp()),
    ) {
        for x1 in r {
            *x1 = *x1 * k;
        }
    }
}

