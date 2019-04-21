use rand::Rng;
use std::thread;
use std::time::Duration;

fn heavy_fizbuz(value: u32) -> String {
    if value % 6 == 0 {
        thread::sleep(Duration::from_secs(3));
        return String::from("FizBuz");
    } else if value % 3 == 0 {
        thread::sleep(Duration::from_secs(2));
        return String::from("Fiz");
    } else if value % 2 == 0 {
        thread::sleep(Duration::from_secs(1));
        return String::from("Buz");
    } else {
        return value.to_string();
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_num: u32 = rng.gen();

    let mut heavy_calc = CalcCacher::new(|num| {
        heavy_fizbuz(num);
        num.to_string()
    });

    println!("Calculating... {}", &heavy_calc.value(rand_num));
    println!("Double check... {}", &heavy_calc.value(rand_num));
}

struct CalcCacher<T>
where
    T: Fn(u32) -> String,
{
    calc_logic: T,
    value: Option<String>,
}

impl<T> CalcCacher<T>
where
    T: Fn(u32) -> String,
{
    fn new(calc_logic: T) -> CalcCacher<T> {
        CalcCacher {
            calc_logic: calc_logic,
            value: None,
        }
    }

    fn value(&mut self, value: u32) -> String {
        match &self.value {
            Some(v) => v.to_string(),
            None => {
                let res = (self.calc_logic)(value);
                let res2 = res.clone();
                self.value = Some(res);
                res2
            }
        }
    }
}
