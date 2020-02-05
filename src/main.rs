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

fn print_sep() {
    println!("{}", "=".repeat(120));
}

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

    for line in file_reader.lines() {
        //        println!("{}", LINE_SEP);
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

    print_sep();
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

    if addr_string.len() > 40 {
        println!("{}", "MORE THAN 40 !!!");
        return AddressType::None;
    }

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

    print_sep();

    let file = File::open(CSV_FILE).unwrap();

    //        let mut net_list = read_file(file);
    let mut net_list: Vec<Ipv4Net> = vec![
        "10.0.0.0/32".parse().unwrap(),
        "10.0.0.1/32".parse().unwrap(),
        "10.0.0.2/32".parse().unwrap(),
        "10.0.0.3/32".parse().unwrap(),
        "10.0.0.4/32".parse().unwrap(),
        //        "10.0.0.5/32".parse().unwrap(),
        "10.0.0.6/32".parse().unwrap(),
        "10.0.0.7/32".parse().unwrap(),
        "10.0.0.8/32".parse().unwrap(),
        //        "10.0.0.9/32".parse().unwrap(),
        //        "10.0.0.10/32".parse().unwrap(),
        "10.0.0.11/32".parse().unwrap(),
        //        "10.0.0.12/32".parse().unwrap(),
        //        "10.0.0.13/32".parse().unwrap(),
        //        "10.0.0.14/32".parse().unwrap(),
        "10.0.0.15/32".parse().unwrap(),
        "10.0.0.16/32".parse().unwrap(),
        "10.0.1.0/24".parse().unwrap(),
    ];
    println!("Records:   {:#?}", net_list.len());
    println!("Addresses: {:#?}", net_list.size());
    //    for net in &net_list {
    //        println!("{:#?} -> {:#?}", net, net.size());
    //    }
    //    print_sep();
    //    print_sep();

    //    net_list = Ipv4Net::aggregate(&net_list);
    //    println!("Records after normalization:   {:#?}", net_list.len());
    //    println!("Addresses after normalization: {:#?}", net_list.size());

    let prefix = 30;
    println!("resizing with prefix = {:#?} ...", prefix);
    let net_list = net_list.resize_with_prefix(prefix);
    println!("Records after resize:   {:#?}", net_list.len());
    println!("Addresses after resize: {:#?}", net_list.size());

    //    let mut intervals: Vec<(_, _)> = networks.iter().map(|n| n.interval()).collect();
    for net in &net_list {
        println!("{:#?} -> {:#?}", net, net.size());
    }

    let duration = start.elapsed();
    print_sep();
    println!("Duration:              {:#?}", duration);
}

trait Next {
    fn next(self) -> Ipv4Addr;
}

impl Next for Ipv4Addr {
    fn next(self) -> Ipv4Addr {
        let mut ip_next = u32::from(self);
        ip_next += 1;
        Ipv4Addr::from(ip_next)
    }
}

trait NetSize {
    fn size(&self) -> u32;
}

trait Resize {
    fn resize_with_prefix(&mut self, new_prefix: u8) -> Vec<Ipv4Net>;
}

impl NetSize for Ipv4Net {
    fn size(&self) -> u32 {
        let base: u32 = 2;
        let p: u8 = 32 - self.prefix_len();
        base.pow(p.into())
    }
}

//impl Resize for Ipv4Net {
//    fn resize_with_prefix(&mut self, new_prefix: u8) -> Vec<Ipv4Net> {}
//}

impl NetSize for Vec<Ipv4Net> {
    fn size(&self) -> u32 {
        let mut total: u32 = 0;

        for n in self.iter() {
            total += n.size();
        }
        total
    }
}

impl Resize for Vec<Ipv4Net> {
    fn resize_with_prefix(&mut self, new_prefix: u8) -> Vec<Ipv4Net> {
        let mut index: usize = 0;
        //        let mut net_buff: Vec<Ipv4Net> = Vec::with_capacity(2);
        //        let mut new_net: Vec<Ipv4Net> = Vec::new();

        loop {
            print_sep();
            println!("index {:#?}", index);
            for (i, net) in self.iter().enumerate() {
                println!("{:#?} -> {:#?} -> {:#?}", i, net, net.size());
            }
            if index == self.len() - 1 {
                break;
            }

            let mut net = self[index];
            let mut next_net = self[index + 1];

            //            let ip = u32::from(net.broadcast());
            //            let ip_next = u32::from(next_net.network());
            let ip = net.broadcast();
            let ip_next = next_net.network();

            println!("{:#?} -> {:#?}", net, next_net);
            println!("{:#?} -> {:#?}", net.network(), net.broadcast());
            println!("{:#?} -> {:#?}", next_net.network(), next_net.broadcast());
            println!("{:#?} -> {:#?}", ip, ip_next);
            //            println!("{:#?} -> {:#?}", u32::from(next_net.network()), next_net.broadcast());
            //            println!("{:#?} -> {:#?}", net, next_net);

            if ip.next() == ip_next {
                index += 1;
                continue;
            }

            self[index] = Ipv4Net::new(ip, new_prefix).unwrap();

            index += 1;
        }

        print_sep();
        print_sep();
        for net in self.iter() {
            println!("{:#?} -> {:#?}", net, net.size());
        }

        Ipv4Net::aggregate(&self)
    }
}

//
//fn interval(net: &Ipv4Net) -> (u32, u32) {
//    (
//        u32::from(net.network()),
//        u32::from(net.broadcast()).saturating_add(1),
//    )
//}

//        let _: () = self;
