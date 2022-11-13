use rand::{distributions::Alphanumeric, Rng};

pub fn random_string(n: usize) -> String {
  rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(n)
    .map(char::from)
    .collect()
}
