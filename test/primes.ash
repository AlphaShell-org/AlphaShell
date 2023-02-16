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

for i in 0..20 {
  if is_prime(i) {
    echo(i);
  }
}
