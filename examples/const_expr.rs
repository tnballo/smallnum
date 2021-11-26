#![feature(generic_const_exprs)]

use smallnum::small_unsigned;

pub struct Arena<T: Default + Copy, const N: usize> {
    storage: [T; N],
    free_list: [small_unsigned!(N); N],
}

impl<T: Default + Copy, const N: usize> Arena<T, N> {
    fn new() -> Self {
        Self {
            storage: [T::default(); N],
            free_list: [0; N],
        }
    }
}

fn main() {
    let arena = Arena::<usize, 10>::new();
}
