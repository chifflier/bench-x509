#![feature(test)]

extern crate test;
use test::Bencher;

static IGCA_DER: &'static [u8] = include_bytes!("../assets/IGC_A.der");

#[bench]
fn bench_parse_x509_rs(b: &mut Bencher) {
    b.iter(|| {
        let _ = bench_x509::decode_x509_rs(IGCA_DER);
    });
}

#[bench]
fn bench_parse_x509_c(b: &mut Bencher) {
    b.iter(|| {
        let _ = bench_x509::decode_x509_c(IGCA_DER);
    });
}