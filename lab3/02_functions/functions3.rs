fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    //Answer: Just add an argument to the call. For instance: 8.
    call_me(8);
}