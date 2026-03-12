#![no_std]

use core::ptr::null_mut;

use vfs_core::{noop_close, noop_ioctl, noop_seek, FileOps};

fn urandom_read(_file: *mut u8, buf: *mut u8, count: usize) -> isize {
    if count != 0 && buf.is_null() {
        return -(libc::EFAULT as isize);
    }
    unsafe { foundation::kfn::random::krandom(buf, count) }
}

fn urandom_write(_file: *mut u8, _buf: *const u8, _count: usize) -> isize {
    -(libc::EBADF as isize)
}

pub const URANDOM_FOPS: FileOps = FileOps {
    read: urandom_read,
    write: urandom_write,
    release: noop_close,
    llseek: noop_seek,
    ioctl: noop_ioctl,
};

pub fn urandom_factory() -> vfs_core::FdEntry {
    vfs_core::FdEntry {
        ops: &URANDOM_FOPS,
        private_data: null_mut(),
    }
}
