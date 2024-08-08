use std::fmt::Debug;

pub struct Test<T> {
    pub some: T,
}

impl<T> Test<T> {
    pub fn new(val: T) -> Self {
        Self { some: val }
    }

    pub fn newww<U: Debug>(&self, val2: U) {
        println!("{:?}", val2);
    }
}
