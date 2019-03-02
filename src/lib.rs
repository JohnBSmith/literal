
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::BTreeMap;

pub trait MapLiteral<K,V> {
    fn new() -> Self;
    fn insert(m: &mut Self, k: K, v: V);
    fn with_capacity(n: usize) -> Self;
}

impl<K,V> MapLiteral<K,V> for HashMap<K,V>
where K: std::cmp::Eq+std::hash::Hash
{
    fn new() -> Self {HashMap::new()}
    fn with_capacity(n: usize) -> Self {HashMap::with_capacity(n)}
    fn insert(m: &mut Self, k: K, v: V){m.insert(k,v);}
}

impl<K,V> MapLiteral<K,V> for BTreeMap<K,V>
where K: std::cmp::Ord
{
    fn new() -> Self {BTreeMap::new()}
    fn with_capacity(_n: usize) -> Self {BTreeMap::new()}
    fn insert(m: &mut Self, k: K, v: V){m.insert(k,v);}
}

#[macro_export]
macro_rules! map {
    ($( $key:tt: $value:expr ),* $(,)?) => {{
        let mut _temp_map = MapLiteral::new();
        $(MapLiteral::insert(&mut _temp_map,$key.into(),$value.into());)*
        _temp_map
    }};
    ({$init:expr} {$( $key:tt: $value:expr ),* $(,)?}) => {{
        let mut _temp_map = $init;
        $(MapLiteral::insert(&mut _temp_map, $key.into(),$value.into());)*
        _temp_map
    }};
    ({$init:expr; $tk:expr, $tv:expr} {$( $key:tt: $value:expr ),* $(,)?}) => {{
        let mut _temp_map = $init;
        $(MapLiteral::insert(&mut _temp_map,$tk($key),$tv($value));)*
        _temp_map
    }};
    ({$tk:expr, $tv:expr} {$( $key:tt: $value:expr ),* $(,)?}) => {{
        let mut _temp_map = MapLiteral::new();
        $(MapLiteral::insert(&mut _temp_map,$tk($key),$tv($value));)*
        _temp_map
    }};
}

