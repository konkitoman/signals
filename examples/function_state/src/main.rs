use signals_kman::prelude::*;

pub struct State {
    a: i32,
}

pub fn main() {
    let mut signal = AdvancedSignal::<i32, ()>::new();

    let state = State { a: 0 };

    signal.connect(&method, vec![Box::new(state)]);

    signal.call(1);
    signal.call(2);
    signal.call(1);
}

advanced_method! {
    pub fn method(a: i32|state: State){
        state.a += a;

        println!("A: {}", state.a);
    }
}
