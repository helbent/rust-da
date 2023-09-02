#[macro_use]
extern crate assert_float_eq;


mod stats;
mod ensemble;
mod lorenz_63;

fn main() {
    println!("Hello, world!");

    let mut state = ensemble::new(12.0, 15.0);

    state.info();

}
