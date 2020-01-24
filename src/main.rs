#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unreachable_code)]

use std::any::type_name;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::exit;
use std::slice::Split;
use std::time::{Duration, Instant};

use encoding_rs::mem::ensure_utf16_validity;
use encoding_rs::WINDOWS_1251;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use ipnet::{IpNet, Ipv4Net, Ipv6Net};

const CSV_FILE: &str = "A:\\dump2.csv";

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[derive(Debug)]
struct Stat {
    total_file_lines: u32,
    ipv4: u32,
    ipv4_networks: u32,
    ipv6: u32,
    ipv6_networks: u32,
}

fn read_file(file: File) -> Vec<Ipv4Net> {
    //    color_backtrace::install();
    let mut input = String::new();
    //    println!("hello {}", input);
    //    stdin().read_line(&mut input).expect("Couldn't read line");

    let line_sep = &("=".repeat(120));

    let mut stat = Stat {
        total_file_lines: 0,
        ipv4: 0,
        ipv4_networks: 0,
        ipv6: 0,
        ipv6_networks: 0,
    };

    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1251))
        .build(file);
    let file_reader = BufReader::new(transcoded);

    let mut net_list: Vec<Ipv4Net> = Vec::with_capacity(3_000_000);
    //    println!("hello {}", input);
    //    stdin().read_line(&mut input).expect("Couldn't read line");
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
                AddressType::IPv4 => {
                    stat.ipv4 += 1;
                    let mut net = substring.to_owned() + "/32";
                    let net: Ipv4Net = net.parse().unwrap();
                    net_list.push(net);
                }
                AddressType::IPv4Net => {
                    stat.ipv4_networks += 1;
                    let net: Ipv4Net = substring.parse().unwrap();
                    net_list.push(net);
                }
                AddressType::IPv6 => stat.ipv6 += 1,
                _ => {
                    if !substring.starts_with("http") && substring != "" {
                        println!("{}", substring)
                    }
                }
            };
        }

        //        if total_file_lines_cnt == 2 {
        //            break;
        //        }
    }

    //    println!("hello {}", input);
    //    stdin().read_line(&mut input).expect("Couldn't read line");

    println!("{}", line_sep);
    println!("{:#?}", stat);

    net_list.sort();
    net_list.dedup();
    net_list
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
    //    color_backtrace::install();
    let start = Instant::now();

    let line_sep = &("=".repeat(120));
    println!("{}", line_sep);

    let file = File::open(CSV_FILE).unwrap();

    //    let mut net_list = read_file(file);
    let mut net_list: Vec<Ipv4Net> = vec![
        "10.0.0.0/32".parse().unwrap(),
        "10.0.0.1/32".parse().unwrap(),
        "10.0.0.2/32".parse().unwrap(),
        "10.0.0.3/32".parse().unwrap(),
        "10.0.0.4/32".parse().unwrap(),
        "10.0.0.5/32".parse().unwrap(),
        "10.0.0.6/32".parse().unwrap(),
        "10.0.0.7/32".parse().unwrap(),
        "10.0.0.8/32".parse().unwrap(),
        "10.0.0.9/32".parse().unwrap(),
        "10.0.0.10/32".parse().unwrap(),
        "10.0.0.11/32".parse().unwrap(),
        "10.10.0.8/24".parse().unwrap(),
    ];
    println!("Records:   {:#?}", net_list.len());
    println!("Addresses: {:#?}", net_list.size());
    for net in &net_list {
        println!("{:#?} -> {:#?}", net, net.size());
    }
    println!("{}", line_sep);
    println!("{}", line_sep);

    //    net_list = Ipv4Net::aggregate(&net_list);
    //    println!("Records after normalization:   {:#?}", net_list.len());
    //    println!("Addresses after normalization: {:#?}", net_list.size());

    let new_prefix: u8 = 30;
    let mut index: usize = 0;
    let mut net_buff: Vec<Ipv4Net> = Vec::with_capacity(2);
    //    for net in &net_list {
    //        if net.prefix_len() < new_prefix {
    //            continue;
    //        }
    //
    //        let new_net = Ipv4Net::new(net.addr(), new_prefix).unwrap();
    //        println!("{:#?} -> {:#?} -> {:#?}", net, net.size(), new_net);
    //    }
    loop {
        println!("{}", line_sep);
        println!("{:#?}", index);
        for net in &net_list {
            println!("{:#?} -> {:#?}", net, net.size());
        }
        if index == net_list.len() - 1 {
            break;
        }

        let net = net_list[index];
        if net.prefix_len() < new_prefix {
            println!("net{:#?}", index);
            index += 1;
            continue;
        }

        let next_net = net_list[index + 1];
        if next_net.prefix_len() < new_prefix {
            println!("next_net{:#?}", index);
            index += 1;
            continue;
        }

        let new_net = Ipv4Net::new(net.addr(), new_prefix).unwrap();
        let new_next_net = Ipv4Net::new(next_net.addr(), new_prefix).unwrap();
        println!("{:#?} -> {:#?} -> {:#?}", net, new_net, new_next_net);

//        if new_next_net.is_sibling(&new_net) {
//            println!("{:#?} -> {:#?}", new_net, new_next_net);
//        }
        net_buff.clear();
        net_buff.push(new_net);
        net_buff.push(new_next_net);
        net_buff = Ipv4Net::aggregate(&net_buff);
        println!("net_buff: {:#?}", net_buff);
        if net_buff.len() == 1 {
            net_list[index] = net_buff[0];
            net_list.remove(index+1);
            continue;
        }

        index += 1;
    }

    println!("{}", line_sep);
    println!("{}", line_sep);
    for net in &net_list {
        println!("{:#?} -> {:#?}", net, net.size());
    }

    let duration = start.elapsed();
    println!("{}", line_sep);
    println!("Duration:              {:#?}", duration);
}

trait NetSize {
    fn size(&self) -> u32;
    fn resize(&self) -> ();
}

impl NetSize for Ipv4Net {
    fn size(&self) -> u32 {
        let base: u32 = 2;
        let p = 32 - self.prefix_len();
        base.pow(p.into())
    }

    fn resize(&self) {}
}

impl NetSize for Vec<Ipv4Net> {
    fn size(&self) -> u32 {
        let mut total: u32 = 0;

        for n in self.iter() {
            total += n.size();
        }
        total
    }

    fn resize(&self) {}
}
