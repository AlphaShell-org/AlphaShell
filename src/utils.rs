#[macro_export]
macro_rules! create_map {
  {$($k: expr => $v: expr),* $(,)?} => {{
    let mut map = HashMap::new();
    $( map.insert($k, $v); )*
    map
  }}
}
