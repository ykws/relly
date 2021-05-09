use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::disk::{DiskManager, PageId, PAGE_SIZE};

pub struct BufferId(usize);

pub type Page = [u8; PAGE_SIZE];

pub struct Buffer {
    pub page_id: PageId,
    pub page: RefCell<Page>,
    pub is_dirty: Cell<bool>,
}

pub struct Frame {
    usage_count: u64,
    buffer: Rc<Buffer>,
}

pub struct BufferPool {
    buffers: Vec<Frame>,
    next_victim_id: BufferId,
}

impl Buffer {
    pub fn check_call() {
        println!("from Buffer");
    }
}

impl Frame {
    pub fn check_call() {
        println!("from Frame");
    }
}

impl BufferPool {
    pub fn check_call() {
        println!("from BufferPool");
    }
}

pub struct BufferPoolManager {
    disk: DiskManager,
    pool: BufferPool,
    page_table:HashMap<PageId, BufferId>,
}

impl BufferPoolManager {
    pub fn check_call() {
        println!("from BufferPoolManager");
    }
}