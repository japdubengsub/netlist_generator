use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::mem;
use std::ops::Add;
use std::path::Path;
use std::time::Instant;

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use ipnet::Ipv4Net;

use netlist_generator::NetSize;

#[path = "./address.rs"]
mod address;
use address::{AddressType, check_addr};


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

pub fn print_stat(net_list: &Vec<Ipv4Net>, stats: &Stat, start_time: Instant) {
    let s1 = net_list.len().to_string();
    let s1 = decimal_mark(s1);
    let s2 = net_list.size().to_string();
    let s2 = decimal_mark(s2);

    let size = mem::size_of::<Ipv4Net>();
    let total = size * net_list.len() / 1024;
    let total = decimal_mark(total.to_string());

    let nets_p: f32 = net_list.len() as f32 / stats.ipv4_networks as f32 * 100f32; // fixme div by 0
    let hosts_p: f32 = net_list.size() as f32 / stats.ipv4_hosts as f32 * 100f32; // fixme div by 0

    let line = format!(
        "Nets: {:>12} ({:>4.1}%)   Hosts:  {:>12} ({:>4.1}%)",
        s1, nets_p, s2, hosts_p
    );
    let line = line.add(&format!(
        "     Memory: {:>6} Kb   Duration:  {:#?}",
        total,
        start_time.elapsed()
    ));
    println!("{}", line);
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
