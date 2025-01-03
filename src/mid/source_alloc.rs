//! exposed api for the allocator used by titanfall 2

use std::{
    alloc::GlobalAlloc,
    ffi::{c_char, c_void},
};

use once_cell::sync::OnceCell;
use windows::{
    core::PCSTR,
    Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
};

use crate::{create_external_interface, high::UnsafeHandle};

type CreateGlobalMemAlloc = extern "C" fn() -> *const IMemAlloc;

/// the allocator
///
/// will only be init after `engine.dll` laods
pub static SOURCE_ALLOC: SourceAlloc = SourceAlloc(OnceCell::new());

/// the allocator type
pub struct SourceAlloc(OnceCell<UnsafeHandle<&'static IMemAlloc>>);

impl SourceAlloc {
    pub(crate) fn init(&self) {
        let create_global_mem_alloc = unsafe {
            #[allow(clippy::missing_transmute_annotations)]
            std::mem::transmute::<_, CreateGlobalMemAlloc>(
                GetProcAddress(
                    GetModuleHandleA(PCSTR(c"tier0.dll".as_ptr().cast()))
                        .expect("couldn't find tier0"),
                    PCSTR(c"CreateGlobalMemAlloc".as_ptr().cast()),
                )
                .expect("couldn't find CreateGlobalMemAlloc"),
            )
        };
        _ = self.0.set(unsafe {
            UnsafeHandle::new(
                create_global_mem_alloc()
                    .as_ref()
                    .expect("IMemAlloc is invalid"),
            )
        })
    }

    /// returns the underlying source allocator
    ///
    /// the source allocator has more functions which may be useful
    pub fn get_underlying_alloc(&self) -> &'static IMemAlloc {
        unsafe { self.0.get_unchecked().get() } // should be init
    }
}

unsafe impl GlobalAlloc for SourceAlloc {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        debug_assert!(
            self.0.get().is_some(),
            "cannot use SourceAlloc before entry::new"
        );
        unsafe { self.0.get_unchecked().copy().Alloc(layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: std::alloc::Layout) {
        debug_assert!(
            self.0.get().is_some(),
            "cannot use SourceAlloc before entry::new"
        );
        unsafe { self.0.get_unchecked().copy().Free(ptr as *mut c_void) }
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        _layout: std::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        debug_assert!(
            self.0.get().is_some(),
            "cannot use SourceAlloc before entry::new"
        );
        unsafe {
            self.0
                .get_unchecked()
                .copy()
                .Realloc(ptr as *mut c_void, new_size) as *mut u8
        }
    }
}

create_external_interface! {
    pub IMemAlloc + IMemAllocMod => {
        pub(self) fn unk0() -> ();
        pub fn Alloc(Size: usize) -> *const c_void;
        pub(self) fn unk2() -> ();
        pub fn Realloc(Mem: *mut c_void, Size: usize) -> *const c_void;
        pub(self) fn unk4() -> ();
        pub fn Free(Mem: *mut c_void) -> ();
        pub(self) fn unk6() -> ();
        pub(self) fn unk7() -> ();
        pub fn GetSize(Mem: *const c_void) -> usize;
        pub(self) fn unk9() -> ();
        pub(self) fn unk10() -> ();
        pub(self) fn unk11() -> ();
        pub(self) fn unk12() -> ();
        pub(self) fn unk13() -> ();
        pub(self) fn unk14() -> ();
        pub(self) fn unk15() -> ();
        pub(self) fn unk16() -> ();
        pub(self) fn unk17() -> ();
        pub fn DumpStats() -> ();
        pub fn DumpStatsFileBase(FileBase: *const c_char) -> ();
        pub(self) fn unk19() -> ();
        pub(self) fn unk20() -> ();
        pub(self) fn unk21() -> ();
        pub(self) fn unk22() -> ();
        pub fn heapchk() -> i32;
    }
}
