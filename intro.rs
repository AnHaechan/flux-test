#![feature(register_tool)]
#![register_tool(flux)]

#[flux::sig(fn() -> i32[10])]
pub fn mk_ten() -> i32 {
    5 + 4
}

#[flux::sig(fn (b: bool[true]))]
pub fn assert(b: bool) {
    if !b { panic!("assertion failed") }
}

fn test() {
    assert (2 + 2 == 4);
    assert (2 + 2 == 5);
}

#[flux::sig(fn (n: i32) -> bool[0 < n])]
pub fn is_pos (n: i32) -> bool {
    if 0 < n {
        true
    } else {
        false
    }
}

pub fn test_pos (n: i32) {
    let m = if is_pos(n) { n - 1 } else { 0 };
    assert (0 <= m);
}

#[flux::sig(fn() -> i32{v: 0 < v})]
pub fn mk_ten_weak() -> i32 {
    5 + 5
}

#[flux::sig(fn (n: i32) -> i32{v: 0 <= v && n <= v})]
pub fn abs(n: i32) -> i32 {
    if 0 <= n {
        n
    } else {
        0 - n
    }
}

#[flux::sig(fn (n: i32) -> i32{v: 1 <= v && n <= v})]
pub fn factorial(n: i32) -> i32 {
    let mut i = 0; // i: i32[0]
    let mut res = 1; // r: i32[1]
    while i < n {
        // i: i32{v: 0 <= v <= n} 
        // res: i32{v: 1 <= v && i <=v}
        i += 1;
        res = res * i;
    }
    res
}

#[flux::sig(fn (p: &i32[@n] -> i32{v: 0 <= v && n <= v}))]
pub fn abs_ref (p: &i32) -> i32 {
    let n = *p;
    if 0 <= n {
        n
    } else {
        0 - n
    }
}