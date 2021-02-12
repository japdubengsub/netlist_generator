#![allow(soft_unstable)]
#![feature(test)]

extern crate test;

use test::Bencher;

use ipnet::Ipv4Net;

use netlist_generator::Resize;

fn get_data() -> Vec<Ipv4Net> {
    let net_list: Vec<Ipv4Net> = vec![
        "10.0.0.0/32".parse().unwrap(),
        "10.0.0.1/32".parse().unwrap(),
        "10.0.0.2/32".parse().unwrap(),
        "10.0.0.3/32".parse().unwrap(),
        //
        "10.0.0.4/32".parse().unwrap(),
        // "10.0.0.5/32".parse().unwrap(),
        "10.0.0.6/32".parse().unwrap(),
        "10.0.0.7/32".parse().unwrap(),
        //
        "10.0.0.8/32".parse().unwrap(),
        // "10.0.0.9/32".parse().unwrap(),
        // "10.0.0.10/32".parse().unwrap(),
        "10.0.0.11/32".parse().unwrap(),
        //
        // "10.0.0.12/32".parse().unwrap(),
        // "10.0.0.13/32".parse().unwrap(),
        // "10.0.0.14/32".parse().unwrap(),
        "10.0.0.15/32".parse().unwrap(),
        //
        "10.0.0.16/32".parse().unwrap(),
        //
        "10.0.1.0/24".parse().unwrap(),
    ];

    return net_list;
}

#[bench]
fn bench_get_data(b: &mut Bencher) {
    b.iter(|| get_data())
}

#[bench]
fn bench_clone_vec(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = get_data();
    b.iter(|| net_list.clone())
}

#[bench]
fn prefix32(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = get_data();
    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 32))
}

#[bench]
fn prefix31(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = get_data();
    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 31))
}

#[bench]
fn prefix30(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = get_data();
    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 30))
}

#[bench]
fn prefix_none(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = vec![
        "10.0.0.0/29".parse().unwrap(),
        "10.0.0.16/32".parse().unwrap(),
        "10.0.1.0/24".parse().unwrap(),
    ];
    // let answer: Vec<Ipv4Net> = vec![
    //     "10.0.0.0/27".parse().unwrap(),
    //     "10.0.1.0/24".parse().unwrap(),
    // ];

    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 26))
}

#[bench]
fn test_no_resize_aggr(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = vec![
        "10.0.0.0/32".parse().unwrap(),
        "10.0.0.1/32".parse().unwrap(),
    ];
    // let answer: Vec<Ipv4Net> = vec!["10.0.0.0/31".parse().unwrap()];

    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 30))
}

#[bench]
fn test_no_resize_not_aggr(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = vec![
        "10.0.0.1/32".parse().unwrap(),
        "10.0.0.2/32".parse().unwrap(),
    ];
    // let answer: Vec<Ipv4Net> = vec![
    //     "10.0.0.1/32".parse().unwrap(),
    //     "10.0.0.2/32".parse().unwrap(),
    // ];

    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 32))
}

#[bench]
fn test_resize_aggr(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = vec![
        "10.0.0.1/32".parse().unwrap(),
        "10.0.0.2/32".parse().unwrap(),
    ];
    // let answer: Vec<Ipv4Net> = vec!["10.0.0.0/30".parse().unwrap()];

    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 24))
}

#[bench]
fn test_resize_aggr2(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = vec![
        "10.0.0.0/32".parse().unwrap(),
        "10.0.0.2/32".parse().unwrap(),
        "10.0.1.0/24".parse().unwrap(),
    ];
    // let answer: Vec<Ipv4Net> = vec!["10.0.0.0/23".parse().unwrap()];

    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 24))
}

#[bench]
fn test_resize_aggr3(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = vec![
        "10.0.0.0/32".parse().unwrap(),
        "10.0.0.4/32".parse().unwrap(),
        "10.0.0.10/30".parse().unwrap(),
    ];
    // let answer: Vec<Ipv4Net> = vec!["10.0.0.0/28".parse().unwrap()];

    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 25))
}

#[bench]
fn test_resize_aggr4(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = vec![
        "10.0.0.0/32".parse().unwrap(),
        "10.0.0.4/32".parse().unwrap(),
        "10.0.1.10/25".parse().unwrap(),
    ];
    // let answer: Vec<Ipv4Net> = vec![
    //     "10.0.0.0/23".parse().unwrap(),
    // ];

    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 24))
}

#[bench]
// #[ignore]
fn test_empty(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = vec![];
    // let answer: Vec<Ipv4Net> = vec![];

    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 25))
}

#[bench]
fn test_just_one(b: &mut Bencher) {
    let net_list: Vec<Ipv4Net> = vec!["10.0.0.0/32".parse().unwrap()];
    // let answer: Vec<Ipv4Net> = vec!["10.0.0.0/32".parse().unwrap()];

    b.iter(|| Vec::<Ipv4Net>::resize_with_prefix(&mut net_list.clone(), 25))
}
