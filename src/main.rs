use stopwatch::Stopwatch;
#[macro_export]
macro_rules! print_prime_index {
    ($x: expr) =>{
        match get_prime_index($x) {
            Ok(1) => println ! ("{}: the first prime", $x),
            Ok(2) => println !("{}: the 2nd prime", $x),
            Ok(3) => println ! ("{}: the 3rd prime", $x),
            Ok(n) =>
                println ! ("{}: the {}th prime", $x, n),
            Err(e) =>
                println! ("{}: {}", $x, e)
        }
    }
}

/// #
///
/// if p is a prime, return index of prime,
///
/// otherwise return error
///
pub fn get_prime_index(p: u64) -> Result<usize, String> {
    let mut primes = vec![];
    let error_str = "Not a prime!";
    if p <= 1 {
        Err(error_str.to_string())
    } else {
        for x in 2..=p {
            if primes.iter().all(|p| x % p != 0) {
                primes.push(x)
            } else if x == p {
                return Err(error_str.to_string());
            }
        }
        Ok(primes.len())
    }
}

fn main() {
    let sw = Stopwatch::start_new();
    print_prime_index!(186647);
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
    fn test_7() {
        assert_eq!(get_prime_index(7), Ok(4))
    }

    #[test]
    fn test_101() {
        print_prime_index!(101)
    }

    #[test]
    fn test_186647() {
        print_prime_index!(186647)
    }

    #[test]
    fn test_300493() {
        print_prime_index!(300493)
    }
}