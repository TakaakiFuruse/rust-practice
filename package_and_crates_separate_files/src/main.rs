// use -> for external crate
// mod -> for internal module

mod senda_mituo;
use rand::Rng;

fn main() {
    crate::senda_mituo::say_it::naha();
    senda_mituo::say_it::naha();
    let rng = rand::thread_rng().gen_range(0, 100);
    println!("{}", rng);
}
