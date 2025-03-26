#![cfg(all(feature = "next", feature = "arbitrary"))]

use arbitrary::{Arbitrary, Unstructured};
use rand::RngCore;
use stellar_xdr::next::{ScMap,ScVal,Limits,WriteXdr};

#[test]
fn arb() {
    let bytes: Vec<u8> = (1u8..255).collect();
    let mut unstructured = Unstructured::new(&bytes);
    for _ in 1..10 {
        let x: ScMap = ScMap::arbitrary(&mut unstructured).unwrap();
        eprintln!("{x:?}");
    }
}

#[test]
fn serial_order_is_structured_order() {
    let mut rng = rand::thread_rng();
    let mut bytes: Vec<u8> = vec![0; 10000];
    for _ in 1..10000 {
        rng.fill_bytes(&mut bytes);
        let mut unstructured = Unstructured::new(&bytes);
        let x: ScVal = ScVal::arbitrary(&mut unstructured).unwrap();
        let y: ScVal = ScVal::arbitrary(&mut unstructured).unwrap();
        let xdr_x = x.to_xdr(Limits::none()).unwrap();
        let xdr_y = y.to_xdr(Limits::none()).unwrap();
        let x_lt_y = x < y;
        let xdrx_lt_xdry = xdr_x < xdr_y;
        if x_lt_y != xdrx_lt_xdry {
            dbg!(x);
            dbg!(y);
            dbg!(xdr_x);
            dbg!(xdr_y);
            panic!("x < y ({x_lt_y}) != xdr_x < xdr_y ({xdrx_lt_xdry})");
        }
    }
}
