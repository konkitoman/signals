use signals_kman::prelude::*;

fn main() {
    // all ways a function is using a turple
    // for only one argument need to be put as ({type/value},)
    // for multiples will be as a simple turple ({type/value}, {type/value})
    let mut signal = Signal::<(i32,), i32>::new();

    signal.connect(&method_1);
    signal.connect(&method_2);
    signal.connect(&method_3);

    println!("Call the last one:");
    let res = signal.call_last((3,));
    println!("Res: {}", res);

    println!("\nCall Sync last to first:");

    // will wait for every method/function to finish
    // the methods will be called from last to first(method_3, method_2, method_1)
    let outs = signal.call((5,));

    // results will be in the same order
    // (method_3, method_2, method_1)
    for out in outs {
        println!("Out: {}", out)
    }

    println!("\nCall Async, but show the result sync:");

    // all methods/function will be puted on ther own thread
    // will return JoinHandles for the threads
    // JoinHandles will be in order
    let outs = signal.call_async((5,));

    for out in outs {
        // join will block wait for the thread to finish.
        let res = out.join().unwrap();
        println!("Res: {}", res);
    }

    println!("\nCall Async, but show what result is finished first:");
    let mut outs = signal.call_async((5,));

    let mut index = 0;
    while !outs.is_empty() {
        if outs[index].is_finished() {
            let res = outs.remove(index).join().unwrap();
            println!("Res: {}", res)
        } else {
            index += 1;
        }
        if index >= outs.len() {
            index = 0;
        }
    }
}

fn method_1(a: i32) -> i32 {
    println!("Method 1!");
    a + 1
}
fn method_2(a: i32) -> i32 {
    println!("Method 2!");
    a + 2
}
fn method_3(a: i32) -> i32 {
    println!("Method 3!");
    a + 3
}
