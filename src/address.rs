pub enum AddressType {
    None,
    IPv4,
    IPv4Net,
    IPv6,
}

pub fn check_addr(addr_string: &str) -> AddressType {
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
