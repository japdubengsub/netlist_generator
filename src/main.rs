#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unreachable_code)]

mod address;
mod argparse;
mod fileops;

// use std::any::type_name;
use std::fs::{File, OpenOptions};
// use std::io::{stdin, BufRead, BufReader, BufWriter, Read, Write};
// use std::mem;
// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
// use std::process::exit;
// use std::slice::Split;
use fileops::{print_sep, print_stat, read_file, write_file, Stat};
use std::time::{Duration, Instant};

// use encoding_rs::mem::ensure_utf16_validity;
// use encoding_rs::WINDOWS_1251;
// use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use ipnet::{IpNet, Ipv4Net, Ipv6Net};
//
// use netlist_generator::print_sep;
use netlist_generator::NetSize;
use netlist_generator::Resize;
use std::cmp::max;
// use std::fmt::{format, Debug};
// use std::iter::Inspect;
// use std::ops::Add;
// use std::path::Path;

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

fn main() {
    let start_timestamp = Instant::now();

    print_sep();
    println!("RKN-NETLIST GENERATOR");
    print_sep();

    let options = argparse::parse_cmd_args();

    let mut net_list = read_file(&options.input);
    let original_stat = Stat {
        total_file_lines: 0,
        ipv4_hosts: net_list.size(),
        ipv4_networks: net_list.len() as u32,
        ipv6_hosts: 0,
        ipv6_networks: 0,
    };
    println!("IN FILE");
    print_stat(&net_list, &original_stat, start_timestamp);

    let timestamp = Instant::now();
    net_list.sort();
    // net_list.sort_unstable();
    net_list.dedup();
    println!("AFTER SORT+DEDUP");
    print_stat(&net_list, &original_stat, timestamp);

    let timestamp = Instant::now();
    net_list = Ipv4Net::aggregate(&net_list);
    println!("AFTER NORMALIZATION");
    print_stat(&net_list, &original_stat, timestamp);

    let mut min_net_mask: u8;
    let mut max_net_mask: u8 = 31;
    let mut routes_max: usize = 0;

    match options.routes_max {
        None => {
            min_net_mask = options.net_mask.unwrap_or(max_net_mask);
            max_net_mask = min_net_mask;
        }
        Some(value) => {
            min_net_mask = options.net_mask.unwrap_or(0);
            routes_max = value as usize;
        }
    }

    for prefix in (min_net_mask..max_net_mask+1).rev() {
        if net_list.len() <= routes_max {
            break;
        }

        println!("RESIZING WITH PREFIX = {:#?} ...", prefix);
        let timestamp = Instant::now();
        net_list = net_list.resize_with_prefix(prefix);
        print_stat(&net_list, &original_stat, timestamp);
    }

    println!("WRITING FILE...");
    let timestamp = Instant::now();
    write_file(net_list, &options.output);
    println!("Duration: {:#?}", timestamp.elapsed());

    print_sep();
    println!("Duration: {:#?}", start_timestamp.elapsed());
}
