fn main() {
    digital_root(16);
}

fn digital_root(n: i64) -> i64 {
    let mut res = n;
    while res >= 10 {
        let mut sum = 0;
        while res > 0 {
           sum += res % 10;
           res /= 10;
        }
        res = sum;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(942), 6);
        assert_eq!(digital_root(132189), 6);
        assert_eq!(digital_root(493193), 2);
        assert_eq!(digital_root(10),1);
    }
}
