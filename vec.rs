#![feature(register_tool)]
#![register_tool(flux)]

// A refined vector tracking size of the vector

#[flux::opaque]
#[flux::refined_by(len: int)]
pub struct RVec<T> {
    inner: Vec<T>,
}

// impl<T> std::ops::Index<usize> for RVec<T> {
//     type Output = T;
//     #[flux::sig(fn (&RVec<T>[@n], i: usize{i < n}) -> &T)]
//     fn index(&self, index: usize) -> &T {
//         self.get(index)
//     }
// }

// impl<T> std::ops::IndexMut<usize> for RVec<T> {
//     #[flux::sig(fn (&mut RVec<T>[@n], i: usize{i < n}) -> &mut T)]
//     fn index_mut(&mut self, index: usize) -> &mut T {
//         self.get_mut(index)
//     }
// }

impl<T> RVec<T> {
    #[flux::trusted]
    #[flux::sig(fn() -> RVec<T>[0])]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[flux::trusted]
    #[flux::sig(fn(self: &strg RVec<T>[@n], T)
        ensures self: RVec<T>[n+1])]
    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    #[flux::trusted]
    #[flux::sig(fn(self: &strg {RVec<T>[@n] | 0 < n}) -> T
        ensures self: RVec<T>[n-1])]
    pub fn pop(&mut self) -> T {
        self.inner.pop().unwrap()
    }

    #[flux::trusted]
    #[flux::sig(fn (&RVec<T>[@n]) -> usize[n])]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[flux::sig(fn (&RVec<T>[@n], i: usize{i < n}) -> &T)]
    pub fn get(&self, i: usize) -> &T {
        &self.inner[i]
    }

    #[flux::sig(fn (&mut RVec<T>[@n], i: usize{i < n}) -> &mut T)]
    pub fn get_mut(&mut self, i: usize) -> &mut T {
        &mut self.inner[i]
    }
}

pub fn test_push_pop() {
    let mut vec = RVec::new();
    vec.push(10);
    vec.push(20);
    assert(vec.len() == 2);
    vec.pop();
    assert(vec.len() == 2);
    vec.pop();
    vec.pop();
}

pub fn vec_sum_off_by_one (vec: &RVec<i32>) -> i32 {
    let mut i = 0;
    let mut res = 0;
    while i <= vec.len() {
        res += vec.get(i);
        i += 1;
    }
    res
}

pub fn vec_sum_correct (vec: &RVec<i32>) -> i32 {
    let mut i = 0;
    let mut res = 0;
    while i < vec.len() {
        res += vec.get(i);
        i += 1;
    }
    res
}

// memoized version of fibonnaci
pub fn fib_wrong(n: usize) -> i32 {
    let mut r = RVec::new();
    let mut i = 0;
    while i < n {
        if i == 0 {
            r.push(0);
        } else if i == 1 {
            r.push(1);
        } else {
            let a = r.get(i-1);
            let b = r.get(i-2);
            r.push(a + b);
        }
        i += 1;
    }
    r.pop() // precondition of `pop` cannot be proved (such that vector is non-empty)
}

// corrected version from the Flux error message
pub fn fib_correct(n: usize) -> i32 {
    if n == 0 { return 0; }
    let mut r = RVec::new();
    let mut i = 0;
    while i < n {
        if i == 0 {
            r.push(0);
        } else if i == 1 {
            r.push(1);
        } else {
            let a = r.get(i-1);
            let b = r.get(i-2);
            r.push(a + b);
        }
        i += 1;
    }
    r.pop()
}

pub fn binary_search_wrong(vec: &RVec<i32>, x: i32) -> Result<usize, usize> {
    let mut size = vec.len();
    let mut left = 0;
    let mut right = size;
    while left <= right {
        let mid = left + size / 2;
        let val = *vec.get(mid); // error: mid can be out of bound
        if val < x {
            left = mid + 1;
        } else if x < val {
            right = mid;
        } else {
            return Ok(mid);
        }
        size = right - left; // error: may underflow
    }
    Err(left)
}


// WIP
// Tells developer which line is out-of-bound -> easier debugging
pub fn binary_search_correct(vec: &RVec<i32>, x: i32) -> Result<usize, usize> {
    let mut size = vec.len();
    let mut left = 0;
    let mut right = size - 1;
    while left <= right {
        let mid = left + size / 2;
        let val = *vec.get(mid);
        if val < x {
            left = mid + 1;
        } else if x < val {
            right = mid;
        } else {
            return Ok(mid);
        }
        size = right - left;
    }
    Err(left)
}

#[flux::sig(fn (b: bool[true]))]
pub fn assert(b: bool) {
    if !b { panic!("assertion failed") }
}