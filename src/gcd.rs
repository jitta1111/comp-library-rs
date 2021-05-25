pub fn gcd(a: i64, b: i64) -> i64 {
    if a < b {
        return gcd(b, a);
    }
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

/// # Returns
/// (x, y, z) s.t. ax + by = z, z = gcd(a, b) 
#[allow(clippy::many_single_char_names)]
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (1, 0, a);
    }
    let (p, q, r) = ext_gcd(b, a % b);
    (q, p - q * (a / b), r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(8, 12), 4);
        assert_eq!(gcd(13, 7), 1);
        assert_eq!(gcd(7777, 1000000007), 1);
    }
    #[test]
    fn test_lcm() {
        assert_eq!(lcm(8, 12), 24);
        assert_eq!(lcm(13, 7), 91);
    }
    #[test]
    fn test_ext_gcd() {
        let check = |a: i64, b: i64| -> bool {
            let (x, y, z) = ext_gcd(a, b);
            if gcd(a, b) != z || a * x + b * y != z {
                return false;
            }
            true
        };
        assert!(check(8, 3));
        assert!(check(8, 12));
        assert!(check(24, 10));
        assert!(check(7777, 1000000007));
    }
}
