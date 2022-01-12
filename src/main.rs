use stopwatch::Stopwatch;

/// #
/// a macro to print
///
#[macro_export]
macro_rules! print_prime_index {
    ($x: expr) => {
        match get_prime_index($x) {
            Ok(1) => println!("{}: the first prime", $x),
            Ok(2) => println!("{}: the 2nd prime", $x),
            Ok(3) => println!("{}: the 3rd prime", $x),
            Ok(n) => println!("{}: the {}th prime", $x, n),
            Err(e) => println!("{}: {}", $x, e),
        }
    };
}

/// #
///
/// if p is a prime, return index of prime,
///
/// otherwise return error
///
pub fn get_prime_index(p: usize) -> Result<usize, String> {
    const PRIME_START: usize = 7;

    let mut is_prime_vec = [0, 0, 1, 1, 0, 1, 0]
        .into_iter()
        .chain(
            (PRIME_START..=p)
                .into_iter()
                .map(|x| if x % 2 == 0 || x % 3 == 0 || x % 5 == 0 { 0 } else { 1 }),
        )
        .collect::<Vec<_>>();

    let mut start = PRIME_START;
    let sqrt = (p as f64).sqrt().floor() as usize;
    while start <= sqrt {
        (2 * start..=p).into_iter().step_by(start).for_each(|x| is_prime_vec[x] = 0);
        start += 2 + is_prime_vec.iter().skip(start + 2).position(|&x| x == 1).unwrap_or(p);
    }
    if is_prime_vec[p] == 1 {
        // Ok(is_prime_vec.into_iter().take(p + 1).sum())   lower than the next line with filter and count
        Ok(is_prime_vec.into_iter().take(p + 1).filter(|&x| x == 1).count())
    } else {
        Err("Not a prime!".to_string())
    }
}

fn main() {
    let sw = Stopwatch::start_new();
    print_prime_index!(89998849);
    println!("It took {:.8} ms", sw.elapsed_ms());
}

#[cfg(test)]
mod tests {
    use crate::get_prime_index;

    #[test]
    fn test_1() {
        assert!(get_prime_index(1).is_err())
    }

    #[test]
    fn test_2() {
        assert_eq!(get_prime_index(2), Ok(1))
    }

    #[test]
    fn test_not() {
        assert!(get_prime_index(4).is_err());
        assert!(get_prime_index(20).is_err());
        assert!(get_prime_index(81).is_err());
        assert!(get_prime_index(121).is_err());
    }

    #[test]
    fn test_less_100() {
        assert_eq!(get_prime_index(2), Ok(1));
        assert_eq!(get_prime_index(3), Ok(2));
        assert_eq!(get_prime_index(5), Ok(3));
        assert_eq!(get_prime_index(7), Ok(4));
        assert_eq!(get_prime_index(11), Ok(5));
        assert_eq!(get_prime_index(13), Ok(6));
        assert_eq!(get_prime_index(17), Ok(7));
        assert_eq!(get_prime_index(19), Ok(8));
        assert_eq!(get_prime_index(23), Ok(9));
        assert_eq!(get_prime_index(29), Ok(10));
        assert_eq!(get_prime_index(31), Ok(11));
        assert_eq!(get_prime_index(37), Ok(12));
        assert_eq!(get_prime_index(41), Ok(13));
        assert_eq!(get_prime_index(43), Ok(14));
        assert_eq!(get_prime_index(47), Ok(15));
        assert_eq!(get_prime_index(53), Ok(16));
        assert_eq!(get_prime_index(59), Ok(17));
        assert_eq!(get_prime_index(61), Ok(18));
        assert_eq!(get_prime_index(67), Ok(19));
        assert_eq!(get_prime_index(71), Ok(20));
        assert_eq!(get_prime_index(73), Ok(21));
        assert_eq!(get_prime_index(79), Ok(22));
        assert_eq!(get_prime_index(83), Ok(23));
        assert_eq!(get_prime_index(89), Ok(24));
        assert_eq!(get_prime_index(97), Ok(25));
    }

    #[test]
    fn test_186647() {
        assert_eq!(get_prime_index(186_647), Ok(16_888));
        assert_eq!(get_prime_index(13_651_973), Ok(888_888));
        assert_eq!(get_prime_index(89_998_849), Ok(5_216_888));
    }
}
