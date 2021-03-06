#![allow(soft_unstable)]
#![feature(test)]

extern crate test;

use test::Bencher;

#[path = "../src/address.rs"] mod address;

#[bench]
fn bench_check_addr_ip4(b: &mut Bencher) {
    // let mut val : u32 = 0;
    // let mut rng = rand::IsaacRng::new_unseeded();
    // let mut map = std::collections::HashMap::new();

    b.iter(|| address::check_addr("192.168.0.21"))
}

#[bench]
fn bench_check_addr_ip4_subnet(b: &mut Bencher) {
    // let mut val : u32 = 0;
    // let mut rng = rand::IsaacRng::new_unseeded();
    // let mut map = std::collections::HashMap::new();

    b.iter(|| address::check_addr("192.168.0.21/30"))
}

#[bench]
fn bench_check_addr_ip6(b: &mut Bencher) {
    // let mut val : u32 = 0;
    // let mut rng = rand::IsaacRng::new_unseeded();
    // let mut map = std::collections::HashMap::new();

    b.iter(|| address::check_addr("2606:4700:0030:0000:0000:0000:681b:b9e0"))
}

#[bench]
fn bench_check_addr_wrong_address(b: &mut Bencher) {
    // let mut val : u32 = 0;
    // let mut rng = rand::IsaacRng::new_unseeded();
    // let mut map = std::collections::HashMap::new();

    b.iter(|| address::check_addr("192.168.1110.21/30"))
}