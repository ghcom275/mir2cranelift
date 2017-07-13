#![feature(intrinsics, lang_items, no_core, fundamental)]
#![no_core]
#![allow(unused_variables)]

fn fibonacci_iterative(n: i32) -> i32 {
    let mut current = 0;
    let mut next = 1;

    let mut iterator = 0;
    loop {
        if iterator == n {
            break;
        }

        let tmp = current + next;
        current = next;
        next = tmp;

        iterator += 1;
    }

    current
}

#[lang = "panic"]
fn panic() -> ! {
    loop {}
}

macro_rules! panic {
    () => (
        panic!("explicit panic")
    );
    ($msg:expr) => ({
        $crate::panic()
    });
}

macro_rules! assert {
    ($cond:expr) => (
        if !$cond {
            panic!(concat!("assertion failed: ", stringify!($cond)))
        }
    );
}

fn main() {
    let result = fibonacci_iterative(25);
    assert!(result == 75025);
}

#[lang = "sized"]
#[fundamental]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy: Clone {}

pub trait Clone: Sized {}

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;

    #[inline]
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

impl PartialEq for i32 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        (*self) == (*other)
    }
    #[inline]
    fn ne(&self, other: &i32) -> bool {
        (*self) != (*other)
    }
}

#[lang = "add"]
pub trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for i32 {
    type Output = i32;
    fn add(self, rhs: i32) -> Self::Output {
        self + rhs
    }
}

#[lang = "sub"]
pub trait Sub<RHS = Self> {
    type Output;
    fn sub(self, rhs: RHS) -> Self::Output;
}

impl Sub for i32 {
    type Output = i32;
    fn sub(self, rhs: i32) -> Self::Output {
        self - rhs
    }
}

#[lang = "add_assign"]
pub trait AddAssign<Rhs = Self> {
    fn add_assign(&mut self, Rhs);
}

impl AddAssign for i32 {
    #[inline]
    fn add_assign(&mut self, other: i32) {
        *self += other
    }
}
