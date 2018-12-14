// use -> for external crate
// mod -> for internal module

mod senda_mituo;

fn main() {
    crate::senda_mituo::say_it::naha();
    senda_mituo::say_it::naha();
}
