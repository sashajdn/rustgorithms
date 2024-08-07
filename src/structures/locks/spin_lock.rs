use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl SpinLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            std::hint::spin_loop();
        }

        SpinLockGuard { lock: self }
    }
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

unsafe impl<T> Send for SpinLockGuard<'_, T> where T: Send {}
unsafe impl<T> Sync for SpinLockGuard<'_, T> where T: Sync {}

impl Deref<T> for SpinLockGuard<'_, T> {
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl DerefMut<T> for SpinLockGuard<'_, T> {
    fn deref(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl Drop<T> for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}
