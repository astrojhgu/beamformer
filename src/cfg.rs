use serde::{Serialize, Deserialize};
use rsdsp::{
    cfg::{
        DelayerCfg
    }
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StationCfg{
    pub dt: f64,
    pub pos:Vec<[f64;3]>,
    pub coarse_ch: ChCfg,
    pub delayer: DelayerCfg,
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct ChCfg{
    pub nch: usize,
    pub tap: usize, 
    pub k: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FineChCfg{
    pub nch: usize,
    pub tap: usize, 
    pub k: f64,
    pub coarse_ch_list: Vec<usize>
}



#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArrayCfg{
    pub station: StationCfg,
    pub fine_ch: FineChCfg,
}
