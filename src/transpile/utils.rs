use rand::distributions::{Alphanumeric, DistString};

pub fn random_string(n: usize) -> String {
  Alphanumeric.sample_string(&mut rand::thread_rng(), n)
}
