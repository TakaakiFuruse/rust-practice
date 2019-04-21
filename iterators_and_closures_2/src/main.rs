use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    // Closure example and Cache logic(Hash map)
    let user_specified_value = 10;
    let random_num = 7;

    // generate_workout(user_specified_value, random_num);

    // Capturing Envs
    let x = 4;
    let equal_to_x = |z| z == x;

    // これはできない
    // fn equal_to_x(z: i32) -> bool {|z|
    //     z == x
    // }

    let y = 4;
    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_num: u32) {
    let mut calc_results = Cacher::new(|num| {
        println!("doing calc..",);
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("DO PUSHUPS, {}", calc_results.value(intensity));
        println!("DO SITUPS, {}", calc_results.value(intensity));
    } else {
        if random_num == 3 {
            println!("TAKE A BREAK");
        } else {
            print!("DO JOG, {}", calc_results.value(intensity));
        };
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calc: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calc: T) -> Cacher<T> {
        Cacher {
            calc,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calc)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_variables() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
