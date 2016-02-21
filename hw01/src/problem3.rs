/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut crossedOut = vec![];
    let mut primes = vec![];

    for i in 2..n {
        if ! crossedOut.contains(&i) {
            primes.push(i);
            for j in multiples(i, n) {
                crossedOut.push(j);
            }
        }
    }

    primes
}

fn multiples(i: u32, n: u32) -> Vec<u32> {
    let mut counter = i;
    let mut multiples = vec![];
    let counter_limit = n / i;

    while counter < counter_limit {
        multiples.push(counter * i);
    }

    multiples
}
