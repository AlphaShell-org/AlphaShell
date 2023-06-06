let N = $1;
if ! N {
  N = 100;
}

// simple
fn simple_primes(upper_bound) {
  fn is_prime(n) {
    if n <= 3 {
      return n <= 1 ? false : true;
    }

    if $(n % 2) == 0 || $(n % 3) == 0 {
      return false;
    }

    let i = 5;
    while $(i ** 2) < n {
      if $(n % i) == 0 || $(n % i + 2) == 0 {
        return false;
      }

      $(i += 6);
    }

    return true;
  }

  for i in 0..upper_bound {
    if is_prime(i) {
      echo(i);
    }
  }
}

// sieve of eratosthenes
fn sieve(upper_bound) {
  let primes = [];

  fn is_prime(n) {
    let upper_bound = $(n ** 0.5);
    for p in primes {
      if p > upper_bound { break; }

      if $(n % p) == 0 {
        return false;
      }
    }

    return true;
  }

  for i in 2..upper_bound {
    if is_prime(i) {
      echo(i);
      primes += i;
    }
  }
}

simple_primes(N);
sieve(N);

