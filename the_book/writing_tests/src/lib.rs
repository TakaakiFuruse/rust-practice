// インテグレーションテストを試してみる

pub fn really_heavy_calc(mut var: i32) -> i32 {
    for _ in 1..10 {
        var += 1
    }
    var
}

pub fn super_heavy_calc(mut var: i32) -> i32 {
    for _ in 1..10 {
        var -= 1
    }
    var
}

pub fn extreamly_heavy_calc(mut var: i32) -> i32 {
    var *= 2;
    var
}
