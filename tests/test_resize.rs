#[cfg(test)]
mod tests {
    use ipnet::Ipv4Net;
    use netlist_generator::Resize;

    fn get_data() -> Vec<Ipv4Net> {
        let net_list: Vec<Ipv4Net> = vec![
            "10.0.0.0/32".parse().unwrap(),
            "10.0.0.1/32".parse().unwrap(),
            "10.0.0.2/32".parse().unwrap(),
            "10.0.0.3/32".parse().unwrap(),
            //
            "10.0.0.4/32".parse().unwrap(),
            // "10.0.0.5/32".parse().unwrap(),
            "10.0.0.6/32".parse().unwrap(),
            "10.0.0.7/32".parse().unwrap(),
            //
            "10.0.0.8/32".parse().unwrap(),
            // "10.0.0.9/32".parse().unwrap(),
            // "10.0.0.10/32".parse().unwrap(),
            "10.0.0.11/32".parse().unwrap(),
            //
            // "10.0.0.12/32".parse().unwrap(),
            // "10.0.0.13/32".parse().unwrap(),
            // "10.0.0.14/32".parse().unwrap(),
            "10.0.0.15/32".parse().unwrap(),
            //
            "10.0.0.16/32".parse().unwrap(),
            //
            "10.0.1.0/24".parse().unwrap(),
        ];

        return net_list;
    }

    #[test]
    fn prefix32() {
        let mut net_list = get_data();
        let answer: Vec<Ipv4Net> = vec![
            "10.0.0.0/30".parse().unwrap(),
            "10.0.0.4/32".parse().unwrap(),
            "10.0.0.6/31".parse().unwrap(),
            "10.0.0.8/32".parse().unwrap(),
            "10.0.0.11/32".parse().unwrap(),
            "10.0.0.15/32".parse().unwrap(),
            "10.0.0.16/32".parse().unwrap(),
            "10.0.1.0/24".parse().unwrap(),
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 32), answer)
    }


    #[test]
    fn prefix31() {
        let mut net_list = get_data();
        let answer: Vec<Ipv4Net> = vec![
            "10.0.0.0/29".parse().unwrap(),
            "10.0.0.8/30".parse().unwrap(),
            "10.0.0.15/32".parse().unwrap(),
            "10.0.0.16/32".parse().unwrap(),
            "10.0.1.0/24".parse().unwrap(),
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 31), answer)
    }


    #[test]
    fn prefix30() {
        let mut net_list = get_data();
        let answer: Vec<Ipv4Net> = vec![
            "10.0.0.0/28".parse().unwrap(),
            "10.0.0.16/32".parse().unwrap(),
            "10.0.1.0/24".parse().unwrap(),
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 30), answer)
    }


    #[test]
    fn prefix_none() {
        let mut net_list: Vec<Ipv4Net> = vec![
            "10.0.0.0/29".parse().unwrap(),
            "10.0.0.16/32".parse().unwrap(),
            "10.0.1.0/24".parse().unwrap(),
        ];
        let answer: Vec<Ipv4Net> = vec![
            "10.0.0.0/27".parse().unwrap(),
            "10.0.1.0/24".parse().unwrap(),
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 26), answer)
    }

    #[test]
    fn test_no_resize___aggr() {
        let mut net_list: Vec<Ipv4Net> = vec![
            "10.0.0.0/32".parse().unwrap(),
            "10.0.0.1/32".parse().unwrap(),
        ];
        let answer: Vec<Ipv4Net> = vec![
            "10.0.0.0/31".parse().unwrap(),
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 30), answer)
    }

    #[test]
    fn test_no_resize___not_aggr() {
        let mut net_list: Vec<Ipv4Net> = vec![
            "10.0.0.1/32".parse().unwrap(),
            "10.0.0.2/32".parse().unwrap(),
        ];
        let answer: Vec<Ipv4Net> = vec![
            "10.0.0.1/32".parse().unwrap(),
            "10.0.0.2/32".parse().unwrap(),
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 32), answer)
    }

    #[test]
    fn test_resize___aggr() {
        let mut net_list: Vec<Ipv4Net> = vec![
            "10.0.0.1/32".parse().unwrap(),
            "10.0.0.2/32".parse().unwrap(),
        ];
        let answer: Vec<Ipv4Net> = vec![
            "10.0.0.0/30".parse().unwrap(),
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 24), answer)
    }

    #[test]
    fn test_resize___aggr2() {
        let mut net_list: Vec<Ipv4Net> = vec![
            "10.0.0.0/32".parse().unwrap(),
            "10.0.0.2/32".parse().unwrap(),
            "10.0.1.0/24".parse().unwrap(),
        ];
        let answer: Vec<Ipv4Net> = vec![
            "10.0.0.0/23".parse().unwrap(),
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 24), answer)
    }

    #[test]
    fn test_resize___aggr3() {
        let mut net_list: Vec<Ipv4Net> = vec![
            "10.0.0.0/32".parse().unwrap(),
            "10.0.0.4/32".parse().unwrap(),
            "10.0.0.10/30".parse().unwrap(),
        ];
        let answer: Vec<Ipv4Net> = vec![
            "10.0.0.0/28".parse().unwrap(),
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 25), answer)
    }

    #[test]
    #[ignore]
    fn test_empty() {
        let mut net_list: Vec<Ipv4Net> = vec![
        ];
        let answer: Vec<Ipv4Net> = vec![
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 25), answer)
    }

    #[test]
    fn test_just_one() {
        let mut net_list: Vec<Ipv4Net> = vec![
            "10.0.0.0/32".parse().unwrap(),
        ];
        let answer: Vec<Ipv4Net> = vec![
            "10.0.0.0/32".parse().unwrap(),
        ];

        assert_eq!(Vec::<Ipv4Net>::resize_with_prefix(&mut net_list, 25), answer)
    }

}
