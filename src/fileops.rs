use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::mem;
use std::path::Path;
use std::time::Instant;

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use ipnet::Ipv4Net;

use address::{AddressType, check_addr};
use netlist_generator::NetSize;

#[path = "./address.rs"]
mod address;

// struct for statistics of the input file
#[derive(Debug, Default)]
pub struct Stat {
    pub total_file_lines: u32,
    pub ipv4_hosts: u32,
    pub ipv4_networks: u32,
    pub ipv6_hosts: u32,    // isn't supported
    pub ipv6_networks: u32, // isn't supported
}

pub fn read_file(file_path: &str) -> Vec<Ipv4Net> {
    let file = File::open(file_path).unwrap();

    println!("Reading file...");

    let mut stat: Stat = Default::default();
    let file_size = file.metadata().unwrap().len();

    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1251))
        .build(file);
    let file_reader = BufReader::new(transcoded);

    let mut net_list: Vec<Ipv4Net> = Vec::new();

    for file_line in file_reader.lines() {
        stat.total_file_lines += 1;
        let file_line = file_line.unwrap();

        for substring in file_line.split(" | ") {
            let substring: &str = substring.split(';').next().unwrap();

            let result = check_addr(substring);

            match result {
                AddressType::IPv4 => {
                    stat.ipv4_hosts += 1;
                    let net = substring.to_owned() + "/32";
                    let net: Ipv4Net = net.parse().unwrap();
                    net_list.push(net);
                }
                AddressType::IPv4Net => {
                    stat.ipv4_networks += 1;
                    let net: Ipv4Net = substring.parse().unwrap();
                    net_list.push(net);
                }
                AddressType::IPv6 => {
                    stat.ipv6_hosts += 1;
                }
                _ => {
                    if !substring.starts_with("http") && !substring.is_empty() {
                        println!("Unknown address: {}", substring)
                    }
                }
            };
        }
    }

    println!("Reading finished.");
    print_sep();
    println!(
        "IPv4 Hosts:          {:>12}\n\
        IPv4 Nets:           {:>12}\n\
        IPv6 Hosts:          {:>12}\n\
        IPv6 Nets:           {:>12}\n\
        Lines in file:       {:>12}\n\
        File size in bytes:  {:>12}",
        decimal_mark(stat.ipv4_hosts.to_string()),
        decimal_mark(stat.ipv4_networks.to_string()),
        decimal_mark(stat.ipv6_hosts.to_string()),
        decimal_mark(stat.ipv6_networks.to_string()),
        decimal_mark(stat.total_file_lines.to_string()),
        decimal_mark(file_size.to_string())
    );
    print_sep();

    net_list
}

pub fn write_file(ip_list: Vec<Ipv4Net>, out_file: &str) {
    let path = Path::new(out_file);

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

pub fn print_sep() {
    println!("{}", "=".repeat(120));
}

pub fn print_header() {
    println!(
        "{:>30}{:>28}{:>28}{:>16}{:>18}",
        "", "Subnets/Routes", "Hosts included", "Mem Size Kb", "Duration"
    );
}

pub fn print_stat(net_list: &Vec<Ipv4Net>, stats: &Stat, start_time: Instant, stage: &str) {
    let duration = format!("{:#?}", start_time.elapsed());

    let s1 = net_list.len().to_string();
    let s1 = decimal_mark(s1);
    let s2 = net_list.size().to_string();
    let s2 = decimal_mark(s2);

    let size = mem::size_of::<Ipv4Net>();
    let size = size * net_list.len() / 1024;
    let total = decimal_mark(size.to_string());

    let mut nets_p: f32 = 0.0f32;
    let mut hosts_p: f32 = 0.0f32;

    if stats.ipv4_networks > 0 {
        nets_p = net_list.len() as f32 / stats.ipv4_networks as f32 * 100f32;
    }
    if stats.ipv4_hosts > 0 {
        hosts_p = net_list.size() as f32 / stats.ipv4_hosts as f32 * 100f32;
    }

    let nets_p = format!("{:>.1}%", nets_p);
    let hosts_p = format!("{:>.1}%", hosts_p);

    println!(
        "{:<30}{:>18}{:>10}{:>18}{:>10}{:>16}{:>18}",
        stage, s1, nets_p, s2, hosts_p, total, duration
    );
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
