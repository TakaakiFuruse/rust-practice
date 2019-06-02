use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let mean_arr: Vec<_> = (1..=100).collect();
    let mean_arr1: Vec<_> = (-100..=50).collect();
    let mode_arr: Vec<i32> = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3];
    let mode_arr1: Vec<i32> = vec![-1, -1, -1, -1, 2, 2, 2, 3, 3, 3];
    println!("mean is {}", mean(&mean_arr));
    println!("mean is {}", mean(&mean_arr1));
    println!("mode is {}", mode(&mode_arr));
    println!("mode is {}", mode(&mode_arr1));
    println!("med is {}", median(&mean_arr));
    println!("med is {}", median(&mean_arr1));
    println!("med is {}", median(&mode_arr));
    println!("med is {}", median(&mode_arr1));
}

fn mean(vect: &Vec<i32>) -> i32 {
    (&vect.iter().fold(0, |sum, x| sum + x)) / (*&vect.len() as i32)
}

fn mode(vect: &Vec<i32>) -> i32 {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for el in vect {
        *hm.entry(*el).or_insert(1) += 1
    }
    let mut res: Vec<(&i32, &i32)> = hm.iter().collect();
    res.sort_by(|a, b| a.cmp(b));
    *res[0].0
}

fn median(vect: &Vec<i32>) -> i32 {
    let length = &vect.len();
    let vect_sort = &mut vect.clone();
    vect_sort.sort_by(|a, b| a.cmp(b));
    if length % 2 == 0 {
        let middle1: usize = length / 2;
        let middle2: usize = (length / 2) - 1;
        return (vect_sort[middle1] + vect_sort[middle2]) / 2;
    } else {
        let middle1: usize = length / 2;
        return vect_sort[middle1];
    };
}
