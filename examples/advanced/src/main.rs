use signals_kman::prelude::*;

#[derive(Clone)]
pub struct State {
    pub a: i32,
    pub b: i32,
}

impl State {
    pub fn new() -> Self {
        Self { a: 1, b: 2 }
    }
}

impl_get_ptr!(State);

fn main() {
    let mut signal = AdvancedSignal::<State, i32>::new();

    signal.connect(&method_1, Vec::new());
    signal.connect(&method_2, Vec::new());
    signal.connect(&method_3, Vec::new());

    let state = State::new();

    println!("Call the last one:");

    let res = signal.call_last(state.clone());
    println!("Res: {}", res);

    println!("\nCall Sync fron last to first:");
    let outs = signal.call(state.clone());

    for out in outs {
        println!("Out: {}", out)
    }

    println!("\nCall Async from last to first, but is waiting in orter for the results");

    let outs = signal.call_async(state.clone());

    for out in outs {
        println!("Res: {}", out.join().unwrap());
    }

    println!("\nCall Async from last to first, but will be shown when is finished");

    let mut outs = signal.call_async(state);

    let mut index = 0;
    while !outs.is_empty() {
        if outs[index].is_finished() {
            let res = outs.remove(index).join().unwrap();
            println!("Res: {}", res);
        } else {
            index += 1;
        }
        if index >= outs.len() {
            index = 0;
        }
    }
}

advanced_method! {
    pub fn method_1(test: State|) -> i32{
        println!("Method 1: {}", test.a);
        test.a
    }
}

advanced_method! {
    pub fn method_2(test: State|) -> i32{
        let res = test.a + test.b * 2;
        println!("Method 2: {}", res);
        res
    }
}

advanced_method! {
    pub fn method_3(test: State|) -> i32{
        let res = test.a - test.b;
        println!("Method 3: {}", res);
        res
    }
}
