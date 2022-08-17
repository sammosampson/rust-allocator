use std::ffi::c_void;

use winapi::um::{memoryapi::VirtualAlloc, winnt::{MEM_RESERVE, MEM_COMMIT, PAGE_READWRITE}};

macro_rules! kilobytes { ($kb:expr) => { $kb * 1024usize }; }
macro_rules! megabytes { ($mb:expr) => { $mb * kilobytes!(1024usize) }; }
macro_rules! gigabytes { ($gb:expr) => { $gb * megabytes!(1024usize) }; }
macro_rules! terrabytes { ($tb:expr) => { $tb * gigabytes!(1024usize) }; } 

pub const HEAP_SIZE: usize = megabytes!(64);

pub trait MemoryChunkFactory {
    fn create(&self) -> MemorySlab;
}

pub struct MemorySlab {
    pub base_address: *mut c_void,
    pub total_size: usize
}

pub struct VirtualMemoryChunkFactory;

impl MemoryChunkFactory for VirtualMemoryChunkFactory {
    fn create(&self) -> MemorySlab {                
        let base_address = terrabytes!(2) as *mut c_void;
        let total_size = HEAP_SIZE;
                
        let base_address = unsafe {
            VirtualAlloc(
            base_address, 
            total_size, 
            MEM_RESERVE|MEM_COMMIT, 
            PAGE_READWRITE
            )
        };

        MemorySlab { base_address, total_size }
    }
}