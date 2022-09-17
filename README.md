# Signals

## Simple  callback system for calling multiples callbacks.

### inspired by QT and godot.



is including `Signal` is used

```rust
fn main(){
    let mut signal = Signal::<(),()>::new();

    signal.connect(&method_1);
    
    signal.call(());
}


fn method_1(){
    println!("Method called!");
}
```
