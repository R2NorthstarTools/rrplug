//! low level abstractions for rSource filesystem

use std::sync::OnceLock;

use crate::{
    bindings::filesystem::{IFileSystem, IFileSystemVTable, IFileSystemVTable2},
    interfaces::external::SourceInterface,
    mid::engine::{DLLPointer, WhichDll},
};

/// holds the reference to the IFileSystem interface
pub static FILE_SYSTEM_SYS: OnceLock<&FileSystemSys> = OnceLock::new();

/// Thin Wrapper around [IFileSystem] for easy importing
#[repr(transparent)]
pub struct FileSystemSys(IFileSystem);

impl SourceInterface for FileSystemSys {
    fn get_vtable(&self) -> std::ptr::NonNull<fn()> {
        panic!("unused");
    }

    fn get_func(&self, _index: usize) -> fn() {
        panic!("unused");
    }
}

impl FileSystemSys {
    /// returns the raw reference to [IFileSystem]
    pub const fn get_raw(&self) -> &IFileSystem {
        &self.0
    }

    /// returns the first [[IFileSystem]] vtable
    pub const fn vtable(&self) -> &IFileSystemVTable {
        unsafe { self.0.vtable.as_ref().unwrap_unchecked() }
    }

    /// returns the second [[IFileSystem]] vtable
    pub const fn vtable2(&self) -> &IFileSystemVTable2 {
        unsafe { self.0.vtable2.as_ref().unwrap_unchecked() }
    }
}

unsafe impl Sync for IFileSystem {}
unsafe impl Send for IFileSystem {}

#[doc(hidden)]
pub fn try_init(dll: &DLLPointer) {
    if let WhichDll::Other("filesystem_stdio.dll") = dll.which_dll() {
        _ = FILE_SYSTEM_SYS.set(unsafe {
            FileSystemSys::from_dll_ptr(
                windows::Win32::Foundation::HMODULE(dll.get_dll_ptr() as isize),
                "VFileSystem017",
            )
            .expect("filesystem_stdio.dll should have had VFileSystem017")
        });
    }
}
