pub fn valid_ip_address(ip: String) -> String {
    let ipv4 = "IPv4".to_string();
    let ipv6 = "IPv6".to_string();
    let neither = "Neither".to_string();

    fn is_ipv6(ip: String) -> Option<String> {
        let ipv6 = "IPv6".to_string();
        let neither = "Neither".to_string();

        let mut colons = vec![0; 7];
        colons[0] = ip.find(':')?;
        for i in 1..7 {
            colons[i] = ip.as_str()[colons[i-1]+1..].find(':')? + colons[i-1] + 1;
        }

        let mut digits = vec![];
        digits.push(&ip.as_str()[..colons[0]]);
        for i in 1..7 {
            digits.push(&ip.as_str()[colons[i-1]+1..colons[i]]);
        }
        digits.push(&ip.as_str()[colons[6]+1..]);
        let isv6 = digits.iter().map(|s| {
            if s.len() == 0 || s.len() > 4 {
                return false
            }
            if s == &"0" || s == &"0000" {
                return true
            }
            s.chars().map(|c| {
                if '0' <= c && c <= '9' {
                    true
                } else if 'a' <= c && c <= 'f' {
                    true
                } else if 'A' <= c && c <= 'F' {
                    true
                } else {
                    false
                }
            }).fold(true, |acc, x| acc && x)
        }).fold(true, |acc, x| acc && x);
        if isv6 {
            Some(ipv6)
        } else {
            Some(neither)
        }
    }

    fn is_ipv4(ip: String) -> Option<String> {
        let ipv4 = "IPv4".to_string();
        let neither = "Neither".to_string();

        let dot0 = ip.find('.')?;
        let dot1 = ip.as_str()[dot0+1..].find('.')? + dot0 + 1;
        let dot2 = ip.as_str()[dot1+1..].find('.')? + dot1 + 1;

        let digits = vec![&ip.as_str()[..dot0], &ip.as_str()[dot0+1..dot1], &ip.as_str()[dot1+1..dot2], &ip.as_str()[dot2+1..]];
        let isv4 = digits.iter().map(|s| {
            if s.len() == 0 {
                return false
            }
            if s == &"0" {
                return true
            }
            if s.chars().next().unwrap() == '0' || s.chars().next().unwrap() == '-' {
                return false
            }
            match s.parse::<i32>() {
                Ok(number) => {
                    0 <= number && number <= 255
                },
                Err(e) => false,
            }
        }).fold(true, |acc, x| acc && x);
        if isv4 {
            Some(ipv4)
        } else {
            Some(neither)
        }
    }

    if ip.len() == 0 {
        return neither
    }

    match ip.find('.') {
        None => {
            match ip.find(':') {
                None => neither,
                Some(_) => is_ipv6(ip).unwrap_or(neither),
            }
        },
        Some(_) => is_ipv4(ip).unwrap_or(neither),
    }
}

#[test]
fn test_valid_ip_address() {
    assert_eq!(valid_ip_address("172.16".to_string()), "Neither");
    assert_eq!(valid_ip_address("0.0.0.-0".to_string()), "Neither");
    assert_eq!(valid_ip_address("172.16.254.1".to_string()), "IPv4");
    assert_eq!(valid_ip_address("172.16.254.01".to_string()), "Neither");
    assert_eq!(valid_ip_address("256.256.256.256".to_string()), "Neither");
    assert_eq!(valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string()), "IPv6");
    assert_eq!(valid_ip_address("2001:db8:85a3:0:0:8A2E:0370:7334".to_string()), "IPv6");
    assert_eq!(valid_ip_address("2001:0db8:85a3::8A2E:0370:7334".to_string()), "Neither");
    assert_eq!(valid_ip_address("02001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string()), "Neither");
}