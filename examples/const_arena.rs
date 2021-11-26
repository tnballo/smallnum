#![feature(type_name_of_val)]

use smallnum::{small_unsigned, SmallUnsigned};

/*
#![feature(generic_const_exprs)] doesn't support macro expansion, so we can't use:

pub struct Arena<T: Copy, const N: usize> {
    pub storage: [Option<T>; N],
    pub free_list: [small_unsigned!(N); N],
}
*/

pub struct Arena<T, I, const N: usize> {
    pub storage: [Option<T>; N],
    pub free_list: [I; N],
}

impl<T: Copy, I: Default + Copy + SmallUnsigned, const N: usize> Arena<T, I, N> {
    fn new() -> Self {
        Self {
            storage: [None; N],
            free_list: [I::default(); N],
        }
    }

    fn len(&self) -> usize {
        N
    }
}

#[derive(Copy, Clone)]
pub struct Item {
    pub field_1: usize,
    pub field_2: usize,
    pub field_3: usize,
}

fn main() {
    const MAX_CAPACITY: usize = 2_048;
    let arena = Arena::<Item, small_unsigned!(MAX_CAPACITY), MAX_CAPACITY>::new();

    assert_eq!(arena.storage.len(), MAX_CAPACITY);
    assert_eq!(arena.free_list.len(), MAX_CAPACITY);
    assert_eq!(arena.len(), MAX_CAPACITY);

    println!(
        "arena.storage<{}>",
        std::any::type_name_of_val(&arena.storage)
    );
    println!(
        "arena.free_list<{}>",
        std::any::type_name_of_val(&arena.free_list)
    );
}
