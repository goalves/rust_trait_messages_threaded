mod module_a;
mod module_b;

fn main() {
    let (a_handler, a_tx) = module_a::new();
    let b_handler = module_b::new(a_tx);
    a_handler.join().ok();
    b_handler.join().ok();
}
