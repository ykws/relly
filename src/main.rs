mod disk;
mod buffer;

fn main() {
    disk::DiskManager::check_call();
    buffer::Buffer::check_call();
    buffer::Frame::check_call();
    buffer::BufferPool::check_call();
    buffer::BufferPoolManager::check_call();

    println!("Hello, world!");
}
