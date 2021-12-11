extern crate beamformer;

use beamformer::{
    station::{
        Station
        , cfg2station
    }
    , cfg::{
        StationCfg
        , ChCfg
    }
};

use rsdsp::{
    frac_delayer::{
        FracDelayer
        , cfg2delayer
    }
    , windowed_fir::{
        pfb_coeff
    }
    , cfg::{
        DelayerCfg
    }
};

use ndarray_npy::{
    NpzWriter
};

use rand::{
    thread_rng
};

use rand_distr::{Distribution, Normal};

fn main() {
    //let delayer=FracDelayer::<f64,f64>::new(200, 100);
    //let mut station=Station::new(&[[0.0, -10.0, 0.0], [0.0, 10.0, 0.0]], 1024, coeff(512, 16, 1.1).view(), delayer, 2.5e-9);

    let delayer_cfg=DelayerCfg{max_delay: 200, half_tap: 100};
    
    let station_cfg=StationCfg{dt: 2.5e-9, coarse_ch: ChCfg{nch: 1024, tap: 16, k: 1.1}
    , pos: vec![[0.0, -10.0, 0.0], [0.0, 10.0, 0.0]]
    , delayer: delayer_cfg
    };

    let mut station=cfg2station(&station_cfg);
    


    //let dd=station.calc_required_digital_delay(0.0, 45_f64.to_radians());
    let dd=vec![0.0, 0.0];

    let signal_len=65536;
    let normal=Normal::new(0.0, 1.0).unwrap();
    let signal:Vec<_>=normal.sample_iter(thread_rng()).take(signal_len).collect();
    
    let output=station.acquire(0.0, 45_f64.to_radians(), &signal, &dd);
    //println!("{:?}", angle2xyz(90_f64.to_radians(), 5_f64.to_radians()));
    let outfile=std::fs::File::create("a.npz").unwrap();
    let mut npz=NpzWriter::new(outfile);
    npz.add_array("coarse", &output).unwrap();
}
