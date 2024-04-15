const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    println!("Firing {} of my {} missiles...", READY_AMOUNT, missiles);
    missiles = missiles - READY_AMOUNT;
    println!("{} left", missiles);

}
