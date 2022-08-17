#![feature(const_mut_refs)]

mod dynamic;
mod locked;
mod bump;
mod allocator;
mod global_allocator;
mod memory;
mod simple;
mod tests;

use global_allocator::*;

fn main() {
    let _s = format!("allocating a string!");
    println!("allocated so far: {}", ALLOCATOR.allocated());
    
    // This `Vec` will allocate memory through `GLOBAL` above
    let mut v = Vec::new();
    v.push(1);

    println!("allocated so far: {}", ALLOCATOR.allocated());

    for i in 0..crate::memory::HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }

}

/*
mod memory;

fn main() {
for i in 0..crate::memory::HEAP_SIZE {
    let x = Box::new(i);
    assert_eq!(*x, i);
}
}
*/
