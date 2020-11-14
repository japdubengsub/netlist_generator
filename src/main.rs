#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unreachable_code)]

use std::any::type_name;
use std::fs::{File, OpenOptions};
use std::io::{stdin, BufRead, BufReader, Read, Write};
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::exit;
use std::slice::Split;
use std::time::{Duration, Instant};

use encoding_rs::mem::ensure_utf16_validity;
use encoding_rs::WINDOWS_1251;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use ipnet::{IpNet, Ipv4Net, Ipv6Net};

use netlist_generator::print_sep;
use netlist_generator::NetSize;
use netlist_generator::Resize;
use std::fmt::format;
use std::iter::Inspect;
use std::path::Path;

const CSV_FILE: &str = "A:\\dump2.csv";
const OUT_FILE: &str = "A:\\dump2.txt";
const PREFIX: u8 = 24;
const MAX_ROUTES: u32 = 300_000;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// struct for statistics of the file
#[derive(Debug, Default)]
struct Stat {
    total_file_lines: u32,
    ipv4_hosts: u32,
    ipv4_networks: u32,
    ipv6_hosts: u32,
    ipv6_networks: u32, // isn't supported
}

fn read_file(file: File) -> Vec<Ipv4Net> {
    //    color_backtrace::install();
    println!("reading file...");

    let mut stat: Stat = Default::default();
    let file_size = file.metadata().unwrap().len();

    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1251))
        .build(file);
    let file_reader = BufReader::new(transcoded);

    // let mut net_list: Vec<Ipv4Net> = Vec::with_capacity(3_000_000);
    let mut net_list: Vec<Ipv4Net> = Vec::new();

    // todo
    // will use 1 cache/previous value as input filter
    // let mut prev_line: &str = "";
    // let mut prev_line: String: = "";

    for file_line in file_reader.lines() {
        stat.total_file_lines += 1;
        let file_line = file_line.unwrap();

        for substring in file_line.split(" | ") {
            // println!("{}", "=".repeat(20));
            // println!("{:#?}", substring);
            // println!("{:#?}", type_of(substring));

            let substring: &str = substring.split(';').next().unwrap();
            // println!("{:#?}", substring);

            let result = check_addr(substring);

            match result {
                AddressType::IPv4 => {
                    stat.ipv4_hosts += 1;
                    let mut net = substring.to_owned() + "/32";
                    let net: Ipv4Net = net.parse().unwrap();
                    net_list.push(net);
                }
                AddressType::IPv4Net => {
                    stat.ipv4_networks += 1;
                    let net: Ipv4Net = substring.parse().unwrap();
                    net_list.push(net);
                }
                AddressType::IPv6 => stat.ipv6_hosts += 1,
                _ => {
                    if !substring.starts_with("http") && substring != "" {
                        println!("{}", substring)
                    }
                }
            };
        }

        // prev_line = &file_line;
        // println!("hello {}", &prev_line);

        // fixme remove
        // if stat.total_file_lines >= 5 {
        //     exit(101);
        //     break;
        // }
    }

    println!("reading finished...");
    print_sep();
    println!(
        "IPv4 Hosts:          {:>12}",
        decimal_mark(stat.ipv4_hosts.to_string())
    );
    println!(
        "IPv4 Nets:           {:>12}",
        decimal_mark(stat.ipv4_networks.to_string())
    );
    println!(
        "IPv6 Hosts:          {:>12}",
        decimal_mark(stat.ipv6_hosts.to_string())
    );
    println!(
        "IPv6 Nets:           {:>12}",
        decimal_mark(stat.ipv6_networks.to_string())
    );
    println!(
        "Total file lines:    {:>12}",
        decimal_mark(stat.total_file_lines.to_string())
    );
    println!(
        "File size:           {:>12} bytes",
        decimal_mark(file_size.to_string())
    );
    print_sep();

    net_list
}

fn check_addr(addr_string: &str) -> AddressType {
    let mut digits_counter: u32 = 0;
    let mut letters_counter: u32 = 0;
    let mut dots_counter: u32 = 0;
    let mut slashes_counter: u32 = 0;
    let mut colons_counter: u32 = 0;

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
    let start_timestamp = Instant::now();

    print_sep();
    println!("RKN-NETLIST NORMALIZER/AGGREGATOR V1.0");
    print_sep();

    let file = File::open(CSV_FILE).unwrap();

    let mut net_list = read_file(file);
    let original_stat = Stat {
        total_file_lines: 0,
        ipv4_hosts: net_list.size(),
        ipv4_networks: net_list.len() as u32,
        ipv6_hosts: 0,
        ipv6_networks: 0,
    };
    print_stat(&net_list, &original_stat, start_timestamp);

    let timestamp = Instant::now();
    net_list.sort();
    net_list.dedup();
    println!("after dedup");
    print_stat(&net_list, &original_stat, timestamp);

    let timestamp = Instant::now();
    net_list = Ipv4Net::aggregate(&net_list);
    println!("after normalization");
    print_stat(&net_list, &original_stat, timestamp);

    for prefix in (PREFIX..32).rev() {
        print_sep();
        println!("resizing with prefix = {:#?} ...", prefix);
        let timestamp = Instant::now();
        net_list = net_list.resize_with_prefix(prefix);
        print_stat(&net_list, &original_stat, timestamp);
    }

    print_sep();
    println!("Duration: {:#?}", start_timestamp.elapsed());

    write_file(net_list);
}

fn parse_cmd_args() {
    exit(1);
}

fn print_stat(net_list: &Vec<Ipv4Net>, stats: &Stat, start_time: Instant) {
    let s1 = net_list.len().to_string();
    let s1 = decimal_mark(s1);
    let s2 = net_list.size().to_string();
    let s2 = decimal_mark(s2);

    let size = mem::size_of::<Ipv4Net>();
    let total = size * net_list.len() / 1024;
    let total = decimal_mark(total.to_string());

    let nets_p: f32 = net_list.len() as f32 / stats.ipv4_networks as f32 * 100f32; // fixme div by 0
    let hosts_p: f32 = net_list.size() as f32 / stats.ipv4_hosts as f32 * 100f32; // fixme div by 0

    println!("Nets:    {:>12}       {:>14.1}%", s1, nets_p);
    println!("Hosts:   {:>12}       {:>14.1}%", s2, hosts_p);
    println!("Memory:  {:>12}       {:>13} Kb", size, total);
    println!("Duration:  {:#?}", start_time.elapsed());
    print_sep();
}

fn decimal_mark(s: String) -> String {
    let mut result = String::with_capacity(s.len() + ((s.len() - 1) / 3));
    let mut i = s.len();
    for c in s.chars() {
        result.push(c);
        i -= 1;
        if i > 0 && i % 3 == 0 {
            result.push('.');
        }
    }
    result
}

fn write_file(ip_list: Vec<Ipv4Net>) {
    let path = Path::new(OUT_FILE);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .append(false)
        .open(path)
        .unwrap();

    for net in ip_list {
        let line = format!("route {:#?} reject;\n", net);
        file.write_all(line.as_bytes())
            .expect("Couldn't write to file");
    }
}
