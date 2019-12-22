
#[cfg(test)]
mod tests;

use std::collections::{HashMap,BTreeMap};
use std::collections::{HashSet,BTreeSet};

/// Combines the array literal with `.into()`.
/// # Examples
/// ```
/// use literal::array_from;
/// let a: &[String] = &array_from!["a", "b", "c", "d"];
/// ```
#[macro_export]
macro_rules! array_from {
    ($($item:expr),* $(,)?) => {[$($item.into(),)*]}
}

/// Combines the vector literal with `.into()`.
/// # Examples
/// ```
/// use literal::vec_from;
/// let a: Vec<String> = vec_from!["a", "b", "c", "d"];
/// ```
#[macro_export]
macro_rules! vec_from {
    ($($item:expr),* $(,)?) => {vec![$($item.into(),)*]}
}

/// Literal for `LinkedList`.
/// # Examples
/// ```
/// use std::collections::LinkedList;
/// use literal::list;
///
/// let a: LinkedList<i32> = list![1, 2, 3, 4];
#[macro_export]
macro_rules! list {
    ($($item:expr),* $(,)?) => {{
        let mut _list = LinkedList::new();
        $(_list.push_back($item);)*
        _list
    }}
}

/// Combines the list literal with `.into()`.
/// # Examples
/// ```
/// use std::collections::LinkedList;
/// use literal::list_from;
///
/// let a: LinkedList<String> = list_from!["a", "b", "c", "d"];
/// ```
#[macro_export]
macro_rules! list_from {
    ($($item:expr),* $(,)?) => {{
        let mut _list = LinkedList::new();
        $(_list.push_back($item.into());)*
        _list
    }}
}

/// Interface between set literals and set data types.
pub trait SetLiteral<Item>: Sized {
    fn new() -> Self;
    fn insert(m: &mut Self, item: Item);
    fn with_capacity(_n: usize) -> Self {Self::new()}
}

impl<Item> SetLiteral<Item> for HashSet<Item>
where Item: std::cmp::Eq+std::hash::Hash
{
    fn new() -> Self {HashSet::new()}
    fn with_capacity(n: usize) -> Self {HashSet::with_capacity(n)}
    fn insert(m: &mut Self, item: Item){m.insert(item);}
}

impl<Item> SetLiteral<Item> for BTreeSet<Item>
where Item: std::cmp::Ord
{
    fn new() -> Self {BTreeSet::new()}
    fn insert(m: &mut Self, item: Item){m.insert(item);}
}

/// Set literal with `.into()` for each element.
/// # Examples
/// ```
/// use std::collections::{HashSet,BTreeSet};
/// use literal::{set,SetLiteral};
///
/// let a: HashSet<String> = set!{"x", "y"};
///
/// let a: BTreeSet<String> = set!{"x", "y"};
/// ```
#[macro_export]
macro_rules! set {
    ($( $item:expr ),* $(,)?) => {{
        let mut _a = SetLiteral::new();
        $(SetLiteral::insert(&mut _a,$item.into());)*
        _a
    }};
}

/// Interface between map literals and map data types.
pub trait MapLiteral<K,V>: Sized {
    fn new() -> Self;
    fn insert(m: &mut Self, k: K, v: V);
    fn with_capacity(_n: usize) -> Self {Self::new()}
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
    fn insert(m: &mut Self, k: K, v: V){m.insert(k,v);}
}

/// Map literal with `.into()` for each key and value.
/// # Examples
/// ```
/// use std::collections::{HashMap,BTreeMap};
/// use literal::{map,MapLiteral};
///
/// let m: HashMap<String,i32> = map!{"x": 1, "y": 2};
///
/// let m: BTreeMap<String,i32> = map!{"x": 1, "y": 2};
/// ```
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

