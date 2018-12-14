#[allow(dead_code)]
pub mod senda {
    pub mod mituo {
        pub fn naha() {
            println!("なはなは");
        }
        fn senda() {
            println!("せんだ");
        }
        pub fn mituo() {
            super::super::senda_outer();
        }
        pub mod senda {
            pub fn mituo() {
                super::senda();
            }
        }
    }
}

fn senda_outer() {
    println!("せんだ");
}

mod my_secret_enum {
    // can access
    pub struct MyBankAccountNumber {
        pub number: i32,
        pub pin: i32,
    }

    impl MyBankAccountNumber {
        pub fn new(pin: i32) -> MyBankAccountNumber {
            use rand::Rng;
            let rng = rand::thread_rng().gen_range(0, 100);

            MyBankAccountNumber {
                number: rng,
                pin: pin,
            }
        }
    }
}

mod my_bitcoin_wallet {
    #[derive(Debug)]
    pub enum Wallet {
        Password,
        BitCoinHash,
    }
}

mod senda_mituo_module {
    use crate::senda::mituo;
    pub fn say_it() {
        mituo::naha();
    }
}

mod senda_mituo_module_pub {
    pub use crate::senda::mituo;
    pub fn say_it() {
        mituo::naha();
    }
}

fn main() {
    // Senda and Main is defined in the same module.
    // -> can access Senda

    // IF pub OK
    senda::mituo::naha();
    crate::senda::mituo::naha();

    // IF NO PUB, NO ACCESS
    senda::mituo::senda();
    crate::senda::mituo::senda();

    // Use super when you crimb up path.
    senda::mituo::mituo();
    senda::mituo::senda::mituo();

    // my_secret_enum::MyCrediCardNumber(123, 123);
    // structはPubにしたものだけアクセスできる
    let my_bank_acct = my_secret_enum::MyBankAccountNumber::new(123);
    println!("{}", my_bank_acct.pin);
    println!("{}", my_bank_acct.number);

    // EnumはPubにすると全部アクセスできる。
    let my_wallet_pass = my_bitcoin_wallet::Wallet::Password;
    let my_wallet_hash = my_bitcoin_wallet::Wallet::BitCoinHash;

    println!("{:?}, {:?}", my_wallet_pass, my_wallet_hash);

    // use 使うと呼び出しを短縮できる
    use crate::senda::mituo;
    mituo::naha();

    // absolute path 使っとくと別Modに切り分けしやすい
    senda_mituo_module::say_it();

    // realtive path -> use self
    use self::my_secret_enum::MyBankAccountNumber;
    println!("{:?}", MyBankAccountNumber::new(1234).pin);

    // これはどこに何があるのかわからなくなるのでやめたほうが良い。
    use crate::senda::mituo::naha;
    naha();

    // だめ(Wordy)
    use std::collections;
    let mut map = collections::HashMap::new();
    map.insert(1, 2);

    // よい
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {}

    fn function2() -> io::Result<()> {}

    // asも使える
    use crate::senda::mituo as SayIt;
    SayIt::naha();

    // pub use ってやるとUSEがPubになる
    senda_mituo_module_pub::say_it();
    senda_mituo_module_pub::mituo::naha();

    // USEはまとめてかける
    use std::cmp::Ordering;
    use std::io;

    use std::{cmp::Ordering, io};

    use std::io;
    use std::io::Write;

    use std::io::{self, Write};
}
