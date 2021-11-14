use ndarray::{Array2, ArrayView1};

use rayon::prelude::*;

use num_complex::Complex64;

use rand::{thread_rng, Rng};
//use num_traits::{Float, FloatConst, NumAssign, NumCast};

use rspfb::{
    ospfb::{
        Analyzer
    }
    , frac_delayer::{
        FracDelayer
        , cfg2delayer
    }
    , windowed_fir::{coeff}
};


use crate::{
utils::{
        angle2xyz
        , apply_delay
        , apply_delay_with_shift
    }
    , constants::LIGHT_SPEED
    , cfg::{
        StationCfg
    }
};




pub struct Antenna{
    pos: [f64;3],
    channelizer: Analyzer<f64,f64>,
    delayer: FracDelayer<f64>,
}

impl Antenna{
    pub fn new(pos:[f64;3], nch: usize, coeff:ArrayView1<f64>, delayer: FracDelayer<f64>)->Self{
        let channelizer=Analyzer::new(nch, coeff);
        Antenna{pos, channelizer, delayer}
    }

    pub fn acquire(&mut self, azimuth: f64, zenith: f64, signal: &[f64], dt: f64)->Array2<Complex64>{
        let dc=angle2xyz(azimuth, zenith);//direction cosine
        let delay=-dc.iter().zip(self.pos.iter()).map(|(&x, &y)| x*y).sum::<f64>()/LIGHT_SPEED/dt;
        let delayed_signal=self.delayer.delay(signal, delay);
        self.channelizer.analyze(&delayed_signal)
    }
}

pub struct Station{
    ants: Vec<Antenna>,
    //synthesizer: Synthesizer<f64, f64>,
    dt: f64,
}

impl Station{
    pub fn new(pos: &[[f64;3]], ncoarse_ch: usize, coeff_stage1: ArrayView1<f64>, delayer: FracDelayer<f64>, dt: f64)->Self{
        let ants:Vec<_>=pos.iter().map(|pos|{
            Antenna::new(*pos, ncoarse_ch, coeff_stage1, delayer.clone())
        }).collect();

        Station{ants, dt}
    }

    pub fn calc_required_digital_delay(&self, azimuth: f64, zenith: f64)->Vec<f64>{
        let dc=angle2xyz(azimuth, zenith);
        self.ants.iter().map(|ant| dc.iter().zip(ant.pos.iter()).map(|(&x, &y)| x*y).sum::<f64>()/LIGHT_SPEED/self.dt ).collect()
    }

    pub fn acquire(&mut self, azimuth: f64, zenith: f64, signal: &[f64], digital_delay: &[f64])->Array2<Complex64>{
        let dt=self.dt;

        self.ants.par_iter_mut().zip(digital_delay.par_iter()).map(|(ant, &d)|{
            let mut channelized=ant.acquire(azimuth, zenith, signal, dt);
            apply_delay(&mut channelized, d);
            channelized
        }).reduce_with(|a,b| a+b).unwrap()
        //self.synthesizer.synthesize(result.view())
    }

    pub fn acquire_with_shift(&mut self, azimuth: f64, zenith: f64, signal: &[f64], 
        digital_delay: &[f64], _i: usize, n: usize)->Array2<Complex64>{
        let dt=self.dt;
        let mut rng=thread_rng();
        let ilist:Vec<_>=(0..self.ants.len()).map(|_| rng.gen_range(0..n)).collect();
        self.ants.par_iter_mut().zip(digital_delay.par_iter().zip(ilist.par_iter())).map(|(ant, (&d, &i))|{
            let mut channelized=ant.acquire(azimuth, zenith, signal, dt);
            apply_delay_with_shift(&mut channelized, d, i, n);
            channelized
        }).reduce_with(|a,b| a+b).unwrap()
        //self.synthesizer.synthesize(result.view())
    }
}


pub fn preferred_station(pos: &[[f64;3]], delayer: FracDelayer<f64>, dt: f64)->Station{
    let nch1=1024;
    let tap1=20;
    let k1=1.6;
    let coeff1 = coeff::<f64>(nch1, tap1, k1);
    //let coeff2 = coeff::<f64>(nch2*2, tap2, Some(k2));

    Station::new(pos, nch1, coeff1.view(), delayer, dt)
}


pub fn cfg2station(cfg: &StationCfg)->Station{
    let nch=cfg.coarse_ch.nch;
    let tap=cfg.coarse_ch.tap;
    let k=cfg.coarse_ch.k;
    let dt=cfg.dt;
    let coeff1=coeff::<f64>(nch/2, tap, k);
    let pos=&cfg.pos;
    let delayer=cfg2delayer(&cfg.delayer);
    Station::new(pos,nch, coeff1.view(), delayer, dt)
}

