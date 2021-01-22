use clap::{crate_authors, crate_version, Clap};

#[derive(Clap, Debug)]
#[clap(
version = crate_version!(),
author = crate_authors!(),
about = "aggregate networks with netmask (from /31 to /1) or to specified amount of routes"
)]
pub struct Options {
    #[clap(short = 'i', long, about = "input file in cvs-format")]
    pub input: String,

    #[clap(short = 'o', long, about = "output file")]
    pub output: String,

    #[clap(short = 'm', long, validator = check_args_netmask, about = "aggregate networks with this netmask (from /31 to /1)")]
    pub net_mask: Option<u8>,

    #[clap(short = 'r', long, validator = check_args_routes, about = "maximum amount of routes in output file")]
    pub routes_max: Option<u32>,
}

pub fn parse_cmd_args() -> Options {
    let opts = Options::parse();
    println!("options: {:#?}", opts);
    opts
}

fn check_args_netmask(net_mask: &str) -> Result<(), String> {
    let net_mask = net_mask.parse::<u8>();

    return match net_mask {
        Ok(net_mask @ 1..=31) => Ok(()),
        _ => Err(String::from("aggregation netmask must be between 31 and 1")),
    };
}

fn check_args_routes(routes_count: &str) -> Result<(), String> {
    let routes_count = routes_count.parse::<u32>();

    return match routes_count {
        Ok(routes_count @ 1..=u32::MAX) => Ok(()),
        _ => Err(String::from("amount of routes must be > 0")),
    };
}
