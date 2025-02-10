use std::fs::write;
use std::env;

use prost::Message;
use se_1::my_se;

pub mod se_1 {
    include!(concat!(env!("OUT_DIR"), "/se_1.rs"));
}

fn pack() -> se_1::MySe
{
    // create request
    let mut se1_req = se_1::MySe::default();
    se1_req.color = "red".to_string();
    // se1_req.size = se_1:::my_se::Size::Large;
    se1_req.set_size(se_1::my_se::Size::Large);
    se1_req
}

fn main() {
    let _y:se_1::MySe = self::pack();
    let mut myvec = Vec::new();
    
    myvec.reserve(_y.encoded_len());
    _y.encode(&mut myvec).unwrap();

    println!("{:?} ", myvec);
    write("ret.bin", myvec).unwrap();
}
 