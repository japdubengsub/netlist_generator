#![allow(soft_unstable)]
#![feature(test)]

extern crate test;

use ipnet::Ipv4Net;

#[path = "../src/fileops.rs"]
mod fileops;

fn get_data() -> Vec<Ipv4Net> {
    let net_list: Vec<Ipv4Net> = vec!["10.0.0.0/32".parse().unwrap(); 1_000];
    // let net_list: Vec<Ipv4Net> = vec![
    //     "10.0.0.0/32".parse().unwrap(),
    //     "10.0.0.1/32".parse().unwrap(),
    //     "10.0.0.2/32".parse().unwrap(),
    //     "10.0.0.3/32".parse().unwrap(),
    //     //
    //     "10.0.0.4/32".parse().unwrap(),
    //     // "10.0.0.5/32".parse().unwrap(),
    //     "10.0.0.6/32".parse().unwrap(),
    //     "10.0.0.7/32".parse().unwrap(),
    //     //
    //     "10.0.0.8/32".parse().unwrap(),
    //     // "10.0.0.9/32".parse().unwrap(),
    //     // "10.0.0.10/32".parse().unwrap(),
    //     "10.0.0.11/32".parse().unwrap(),
    //     //
    //     // "10.0.0.12/32".parse().unwrap(),
    //     // "10.0.0.13/32".parse().unwrap(),
    //     // "10.0.0.14/32".parse().unwrap(),
    //     "10.0.0.15/32".parse().unwrap(),
    //     //
    //     "10.0.0.16/32".parse().unwrap(),
    //     //
    //     "10.0.1.0/24".parse().unwrap(),
    // ];

    return net_list;
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::fileops::{write_file, write_file2, write_file3};
    use super::{Ipv4Net, get_data};

    #[bench]
    fn bench_write_file(b: &mut Bencher) {
        let net_list: Vec<Ipv4Net> = get_data();
        let out_file = "a:\\dump2.txt";
        // b.iter(|| fileops::write_file(net_list.clone(), out_file))
        b.iter(|| write_file(net_list.clone(), out_file))
    }

    #[bench]
    fn bench_write_file2(b: &mut Bencher) {
        let net_list: Vec<Ipv4Net> = get_data();
        let out_file = "a:\\dump2.txt";
        // b.iter(|| fileops::write_file2(net_list.clone(), out_file))
        b.iter(|| write_file2(net_list.clone(), out_file))
    }

    #[bench]
    fn bench_write_file3(b: &mut Bencher) {
        let net_list: Vec<Ipv4Net> = get_data();
        let out_file = "a:\\dump2.txt";
        // b.iter(|| fileops::write_file3(&net_list, out_file))
        b.iter(|| write_file3(&net_list, out_file))
    }

    #[bench]
    fn format_url_macro(b: &mut Bencher) {
        let index = "test_idx".to_string();
        let name = "test_alias".to_string();

        b.iter(|| format!("/{}/_alias/{}", index, name));
    }

    #[bench]
    fn format_url_concat(b: &mut Bencher) {
        let index = "test_idx".to_string();
        let name = "test_alias".to_string();

        b.iter(|| {
            let mut url = "/".to_string();
            url = url + &index[..] + "/_alias/" + &name[..];
            url
        });
    }

    #[bench]
    fn format_url_push(b: &mut Bencher) {
        let index = "test_idx".to_string();
        let name = "test_alias".to_string();

        b.iter(|| {
            let mut url = String::with_capacity(1 + "/_alias/".len() + index.len() + name.len());
            url.push_str("/");
            url.push_str(&index);
            url.push_str("/_alias/");
            url.push_str(&name);
            url
        });
    }

    #[bench]
    fn format_url_write(b: &mut Bencher) {
        let index = "test_idx".to_string();
        let name = "test_alias".to_string();

        b.iter(|| {
            use std::fmt::Write;
            let mut url = String::with_capacity(1 + "/_alias/".len() + index.len() + name.len());
            write!(url, "/{}/_alias/{}", index, name).unwrap();
            url
        });
    }

    #[bench]
    fn format_url_vec_concat(b: &mut Bencher) {
        let index = "test_idx".to_string();
        let name = "test_alias".to_string();

        b.iter(|| {
            let url = vec!["/", &index[..], "/_alias/", &name[..]].concat();
            url
        });
    }

    #[bench]
    fn format_url_array_concat(b: &mut Bencher) {
        let index = "test_idx".to_string();
        let name = "test_alias".to_string();

        b.iter(|| {
            let url = ["/", &index[..], "/_alias/", &name[..]].concat();
            url
        });
    }
}
