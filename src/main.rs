use std::time::Instant;

use ipnet::Ipv4Net;

use fileops::{print_header, print_sep, print_stat, read_file, Stat, write_file};
use netlist_generator::{NetSize, Resize};

mod argparse;
mod fileops;

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
    print_header();
    print_sep();
    print_stat(&net_list, &original_stat, start_timestamp, "Found in file:");

    let timestamp = Instant::now();
    net_list.sort();
    net_list.dedup();
    print_stat(&net_list, &original_stat, timestamp, "After sort + dedup:");

    let timestamp = Instant::now();
    net_list = Ipv4Net::aggregate(&net_list);
    print_stat(&net_list, &original_stat, timestamp, "After normalization");

    let min_net_mask: u8;
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

    for prefix in (min_net_mask..max_net_mask + 1).rev() {
        if net_list.len() <= routes_max {
            break;
        }

        let stage = format!("Resizing with prefix /{:#?}", prefix);
        let timestamp = Instant::now();
        net_list = net_list.resize_with_prefix(prefix);
        print_stat(&net_list, &original_stat, timestamp, &stage);
    }

    print!("Writing file");
    let timestamp = Instant::now();
    write_file(net_list, &options.output);
    let duration = format!("{:#?}", timestamp.elapsed());
    println!("{:>108}", duration);

    print_sep();
    let duration = format!("{:#?}", start_timestamp.elapsed());
    println!("Total time: {:>108}", duration);
}
