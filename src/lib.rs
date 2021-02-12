use std::cmp::{max, min};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use ipnet::Ipv4Net;

pub trait NetSize {
    fn size(&self) -> u32;
}

impl NetSize for Ipv4Net {
    fn size(&self) -> u32 {
        let base: u32 = 2;
        let p: u8 = 32 - self.prefix_len();
        base.pow(p.into())
    }
}

impl NetSize for Vec<Ipv4Net> {
    fn size(&self) -> u32 {
        let mut total: u32 = 0;

        for n in self.iter() {
            total += n.size();
        }
        total
    }
}

pub trait Resize {
    fn resize_with_prefix(&mut self, new_prefix: u8) -> Vec<Ipv4Net>;
}

impl Resize for Vec<Ipv4Net> {
    fn resize_with_prefix(&mut self, new_prefix: u8) -> Vec<Ipv4Net> {
        let mut net_buff: Vec<Ipv4Net> = Vec::with_capacity(2);
        net_buff.push("10.10.0.0/32".parse().unwrap());
        net_buff.push(net_buff[0]);

        // amount of iterations
        let mut range = max(self.len(), 1) - 1;

        for index in 0..range {
            let mut current_net = self[index];
            let mut next_net = self[index + 1];

            let ip = current_net.network();
            let ip_next = next_net.network();

            //
            // RESIZE
            //
            let max_prefix = max(current_net.prefix_len(), next_net.prefix_len());
            let n: u8 = max(max_prefix as i8 - new_prefix as i8 + 1, 0) as u8; // todo remove cast

            for i in 0..n {
                let current_prefix = max_prefix - i;

                if current_net.prefix_len() > current_prefix {
                    current_net = Ipv4Net::new(ip, current_prefix).unwrap();
                }
                if next_net.prefix_len() > current_prefix {
                    next_net = Ipv4Net::new(ip_next, current_prefix).unwrap();
                }

                net_buff[0] = current_net;
                net_buff[1] = next_net;

                let net_buff2 = Ipv4Net::aggregate(&net_buff);

                if net_buff2.len() == 1 {
                    self[index] = current_net;
                    self[index + 1] = next_net;
                    break;
                }
            }
        }

        let mut out = Ipv4Net::aggregate(&self);

        return out;
    }
}
