struct Coords {
    pub x: i64,
    pub y: i64,
}

fn shift_x_twice(coords: &mut Coords, delta: &i64) {
    coords.x += *delta;
    coords.x += *delta;
}

fn main() {
    let mut a = Coords { x: 10, y: 10 };
    let delta_a = 10;
    shift_x_twice(&mut a, &delta_a);

    let mut b = Coords { x: 10, y: 10 };
    let delta_b = &b.x;
    shift_x_twice(&mut b, delta_b)
}
