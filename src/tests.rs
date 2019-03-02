
use std::collections::{HashMap,BTreeMap};
use std::rc::Rc;
use crate::{map,MapLiteral};

#[derive(Debug)]
struct Point{x: i32, y: i32}

#[test]
fn test1() {
    let m: HashMap<i32,i32> = map!{0: 1, 1: 0};
    println!("{:?}",m);

    let m: HashMap<i32,i32> = map!{0: 1, 1: 0,};
    println!("{:?}",m);

    let m: HashMap<String,i32> = map!{"x": 0, "y": 1};
    println!("{:?}",m);

    let m: HashMap<Rc<str>,i32> = map!{"x": 0, "y": 1};
    println!("{:?}",m);

    let m: HashMap<i32,Option<i32>> = map!{0: 0, 1: None};
    println!("{:?}",m);

    let m: HashMap<i32,Option<String>> = map!{
        0: "x".to_string(), 1: None
    };
    println!("{:?}",m);

    let m: HashMap<(i32,i32),i32> = map!{(0,0): 0, (0,1): 1};
    println!("{:?}",m);

    let m: HashMap<String,Point> = map!{
        "p": Point{x: 0, y: 0},
        "q": Point{x: 1, y: 0}
    };
    println!("{:?}",m);

    let m: BTreeMap<i32,i32> = map!{0: 1, 1: 0};
    println!("{:?}",m);

    let m: BTreeMap<String,i32> = map!{"x": 0, "y": 1};
    println!("{:?}",m);

    let m: BTreeMap<Rc<str>,i32> = map!{"x": 0, "y": 1};
    println!("{:?}",m);

    let m: HashMap<String,HashMap<String,i32>> = map!{
        "p": HashMap::from(map!{"x": 0, "y": 0}),
        "q": HashMap::from(map!{"x": 1, "y": 0})
    };
    println!("{:?}",m);
}

