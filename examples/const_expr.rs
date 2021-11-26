#![feature(generic_const_exprs)]

use smallnum::small_unsigned;

/*
trait ArenaIdxArr {
    type Idx;
}

impl <T, const N: usize> ArenaIndexArr {

}
*/

#[derive(Clone, Debug)]
pub struct Arena<T, const N: usize> {
    storage: [T; N],
    free_list: [small_unsigned!(N); N],
}

impl<T, const N: usize> Arena<T, N> {
    const fn new() -> Self {
        Self {
            storage: [T; N],
            free_list: [T; N],
        }
    }
}

fn main() {
    let arena = Arena::new();
}
