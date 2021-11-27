#![feature(type_name_of_val)]

use smallnum::{small_unsigned, SmallUnsigned};

// This is a demo/template for a const arena design, does not include actual add/remove operations, etc.
// The novelty here is that I (array of free indexes) is akin to a "dependant type" computed from
// the value of N (size of arena). But that computation is done by the caller at construction time, with
// the `small_unsigned` macro.
pub struct Arena<T, I, const N: usize> {
    pub storage: [Option<T>; N],
    pub free_list: [I; N],
}

/*
Note: it'd be cleaner for the `Arena` to do the type computation and not have the caller (e.g. lib usr) worry about it.
But even Nightly's #![feature(generic_const_exprs)] doesn't support macro expansion, so we can't use:

pub struct Arena<T: Copy, const N: usize> {
    pub storage: [Option<T>; N],
    pub free_list: [small_unsigned!(N); N],
}
*/

impl<T: Copy, I: Default + Copy + SmallUnsigned, const N: usize> Arena<T, I, N> {
    fn new() -> Self {
        let mut a = Self {
            storage: [None; N],
            free_list: [I::default(); N],
        };

        for i in 0..N {
            a.free_list[i] = I::checked_from(i);
        }

        a
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

    // Caller specifies compile-time contract between I and N
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
