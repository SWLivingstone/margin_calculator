#[macro_use]
extern crate helix;

ruby! {
    class MarginCalculator {
        def hello() {
            println!("Hello from margin_calculator!");
        }
    }
}
