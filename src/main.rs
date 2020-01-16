#![allow(unused_imports)]
#![allow(unused_mut)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;

const CSV_FILE: &str = "dump2.csv";


fn check_addr(addr: &str) -> (u32, u32) {
    let mut dots: u32 = 0;
    let mut colons: u32 = 0;

    for char in addr.chars() {
        if char == '.' {
            dots += 1;
        } else if char == ':' {
            colons += 1;
        }
    }

    return (dots, colons);
}


fn read_file(file: File) {
    let line_sep = &("=".repeat(120));

    let mut total_file_lines_cnt: u32 = 0;
    let mut ipv4_cnt: u32 = 0;
    let mut ipv4_uq_cnt: u32 = 0;
    let mut ipv6_cnt: u32 = 0;
    let mut ipv6_uq_cnt: u32 = 0;

    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1251))
        .build(file);
    let file_reader = BufReader::new(transcoded);

    for line in file_reader.lines() {
        total_file_lines_cnt += 1;
        let l = line.unwrap();

        for addr in l.split(" | ") {
            let mut a: &str = addr.split(";").next().unwrap();
            let b = check_addr(a);

            if b.0 == 3 {
                ipv4_cnt += 1;
            } else if b.1 == 7 {
                ipv6_cnt += 1;
            } else if !a.starts_with("http") {
                println!("{}", a);
            }
        }
    }

    println!("{}", line_sep);
    println!("Total lines in file:   {}", total_file_lines_cnt);
    println!("Total ipv4 addresses:  {}", ipv4_cnt);
    println!("Unique ipv4 addresses: {}", ipv4_uq_cnt);
    println!("Total ipv6 addresses:  {}", ipv6_cnt);
    println!("Unique ipv6 addresses: {}", ipv6_uq_cnt);
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
