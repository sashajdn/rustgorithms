use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

struct RcBox<T> {
    ref_count: usize,
    value: T,
}

pub struct Rc<T> {
    ptr: NonNull<RcBox<T>>,
    _marker: PhantomData<RcBox<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let box_ = Box::new(RcBox {
            ref_count: 1,
            value,
        });

        Self {
            ptr: NonNull::new(Box::into_raw(box_)).expect("BUG: can create non null ptr"),
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            let inner = self.ptr.as_ptr();
            (*inner).ref_count += 1;
        }
        Self {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.ptr.as_ref().value }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            let inner = self.ptr.as_mut();
            inner.ref_count -= 1;
            if inner.ref_count == 0 {
                Box::from_raw(self.ptr.as_ptr());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_impl() {}
}
