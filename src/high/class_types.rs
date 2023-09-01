#[repr(transparent)]
pub struct PointerIterator<T> {
    ptr: *mut T,
}

impl<T> Iterator for PointerIterator<T> {
    type Item = *mut T;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        let rtn: *mut T = unsafe { self.ptr.as_mut()? };
        self.ptr = unsafe { self.ptr.add(1) };
        Some(rtn)
    }
}

impl<T> PointerIterator<T> {
    /// # Safety
    /// the pointer must be an array of pointers to `T` terminated by a null pointer
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }
}
