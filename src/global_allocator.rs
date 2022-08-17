#[cfg(feature = "simple")]
#[global_allocator]
pub static ALLOCATOR: crate::simple::SimpleAllocator = crate::simple::simple_allocator();

#[cfg(feature = "bump")]
#[global_allocator]
pub static ALLOCATOR: crate::bump::BumpAllocator = crate::bump::bump_allocator();

#[cfg(feature = "fixed_size_block")]
#[global_allocator]
pub static ALLOCATOR: crate::dynamic::FixedSizeBlockAllocator = crate::dynamic::fixed_size_block_allocator();