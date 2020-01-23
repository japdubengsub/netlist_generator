#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unreachable_code)]

use std::any::type_name;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::process::exit;
use std::slice::Split;
use std::time::{Duration, Instant};

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use ipnet::IpNet;

const CSV_FILE: &str = "dump2.csv";

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[derive(Debug)]
struct Stat {
    total_file_lines: u32,
    ipv4: u32,
    ipv4_networks: u32,
    ipv4_total: u32,
    ipv6: u32,
    ipv6_networks: u32,
    ipv6_total: u32,
}

fn read_file(file: File) {
    //    color_backtrace::install();
    let line_sep = &("=".repeat(120));

    let mut total_file_lines_cnt: u32 = 0;
    let mut ipv4_cnt: u32 = 0;
    let mut ipv4_uq_cnt: u32 = 0;
    let mut ipv6_cnt: u32 = 0;
    let mut ipv6_uq_cnt: u32 = 0;
    let mut stat = Stat {
        total_file_lines: 0,
        ipv4: 0,
        ipv4_networks: 0,
        ipv4_total: 0,
        ipv6: 0,
        ipv6_networks: 0,
        ipv6_total: 0,
    };

    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1251))
        .build(file);
    let file_reader = BufReader::new(transcoded);

    let mut net_list: Vec<IpNet> = Vec::with_capacity(3_000_000);
    //    let net1: IpNet = "10.1.1.0/24".parse().unwrap();
    //    let net2: IpNet = "10.1.1.34/4".parse().unwrap();
    //    let net3: IpNet = "10.5.1.5/32".parse().unwrap();
    //    let ip1: Ipv4Addr = "10.5.1.5".parse().unwrap();
    //    net_list.push(net1);
    //    net_list.push(net2);
    //    net_list.push(net3);
    //    println!("{:#?}", net1.netmask());
    //    println!("{:#?}", net2.netmask());
    //    println!("{:#?}", net2.addr());
    //    println!("{:#?}", net2.hosts());
    //    println!("{:#?}", type_of(net2));
    //    println!("{:#?}", ip1);
    //    println!("{:#?}", net_list);
    //
    //    println!("{:#?}", IpNet::aggregate(&net_list));
    //
    //    exit(0);

    for line in file_reader.lines() {
        //        println!("{}", line_sep);
        stat.total_file_lines += 1;
        let file_line = line.unwrap();
        //        println!("{}   :::   {:?}", num, l);

        for mut substring in file_line.split(" | ") {
            //            println!("{}", "=".repeat(20));
            //            println!("{:#?}", substring);
            //            println!("{:#?}", type_of(substring));

            let substring: &str = substring.split(';').next().unwrap();
            //            println!("{:#?}", substring);

            let result = check_addr(substring);

            match result {
                AddressType::IPv4 => stat.ipv4 += 1,
                AddressType::IPv4Net => stat.ipv4_networks += 1,
                AddressType::IPv6 => stat.ipv6 += 1,
                _ => {
                    if !substring.starts_with("http") && substring != "" {
                        println!("{}", substring)
                    }
                }
            };

            //            let mut network_str = &(substring.to_owned() + "/32");
            //            println!("{:#?}", network_str);
            //            println!("{:#?}", type_of(network_str));
            //            let result = network_str.parse::<IpNet>();
            //            println!("{:#?}", result);
        }

        //        if total_file_lines_cnt == 2 {
        //            break;
        //        }
    }

    let net_list2 = IpNet::aggregate(&net_list);

    println!("{}", line_sep);
    println!("Total lines in file:   {}", total_file_lines_cnt);
    println!("Total ipv4 addresses:  {}", ipv4_cnt);
    println!("Unique ipv4 addresses: {}", ipv4_uq_cnt);
    println!("Total ipv6 addresses:  {}", ipv6_cnt);
    println!("Unique ipv6 addresses: {}", ipv6_uq_cnt);
    //    println!("Unique ipv6 addresses: {:#?}", net_list2);
    println!("Unique ipv6 addresses: {}", net_list2.len());
    println!("{:#?}", stat);
}

fn check_addr(addr_string: &str) -> AddressType {
    let mut digits_counter = 0;
    let mut letters_counter = 0;
    let mut dots_counter = 0;
    let mut slashes_counter = 0;
    let mut colons_counter = 0;

    for c in addr_string.chars() {
        if c.is_ascii_digit() {
            digits_counter += 1;
        } else if c == '.' {
            dots_counter += 1;
        // mind the order of is_ascii_digit and is_ascii_hexdigit checks
        } else if c.is_ascii_hexdigit() {
            letters_counter += 1;
        } else if c == ':' {
            colons_counter += 1;
        } else if c == '/' {
            slashes_counter += 1;
        } else {
            return AddressType::None;
        }
    }

    // ipv4
    if slashes_counter == 0
        && dots_counter == 3
        && colons_counter == 0
        && digits_counter >= 4
        && digits_counter <= 12
    {
        return AddressType::IPv4;
    // ipv6
    } else if colons_counter == 7
        && slashes_counter == 0
        && dots_counter == 0
        && (digits_counter + letters_counter) == 32
    {
        return AddressType::IPv6;
    // ipv4 subnet
    } else if slashes_counter == 1
        && dots_counter == 3
        && colons_counter == 0
        && digits_counter >= 5
        && digits_counter <= 14
    {
        return AddressType::IPv4Net;
    }

    AddressType::None
}

enum AddressType {
    None,
    IPv4,
    IPv4Net,
    IPv6,
}

fn main() {
    let start = Instant::now();

    let line_sep = &("=".repeat(120));
    println!("{}", line_sep);

    let file = File::open(CSV_FILE).unwrap();

    read_file(file);

    let duration = start.elapsed();
    println!("{}", line_sep);
    println!("Duration:              {:#?}", duration);
}
