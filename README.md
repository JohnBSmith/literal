
## Literals for Rust's collections

```rust
use std::collections::HashMap;
use literal::{map,MapLiteral};

fn main(){
    let m: HashMap<String,i32> = map!{"x": 0, "y": 1};
    println!("{:?}",m);
}
```

