mod balance;
mod system;

#[warn(unused_variables)]
fn main() {
    println!("BPChain");
    let mut balance = balance::Pallet::new();
    let mut system = system::Pallet::new();
}
