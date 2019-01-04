#![allow(unused_variables)]
#![allow(dead_code)]

type Int32 = i32;
struct ComplexRefs<'a> {
    int_ref: &'a i32,
    unsigned_ref: &'a u32,
    str_ref: &'a str,
}

fn main() {
    let a: Int32 = -423;
    let b: Int32 = 444_444;
    let c: Int32 = 2321;

    let d: u32 = 5;
    let str_ref: &str = "YOYOYOYO";

    let cr: ComplexRefs = ComplexRefs {
        int_ref: &a,
        unsigned_ref: &d,
        str_ref,
    };

    let e;
    {
        let r = cr;
        e = three_ref(&a, &b, &c, &r);
    }
    println!("{}", e);
}

fn three_ref<'a, 'b>(a: &'a Int32, b: &Int32, c: &Int32, cr: &'b ComplexRefs<'a>) -> &'a Int32 {
    if *a < 5 {
        cr.int_ref
    } else {
        a
    }
}
