//! abstractions for rSource filesystem

use std::{
    ffi::{CStr, CString},
    fmt::Display,
    io::Read,
    path::Path,
    str::FromStr,
};

use crate::{
    bindings::filesystem::{FileHandle_t, VPKData},
    errors::FsOpenError,
    mid::filesystem::{FileSystemSys, FILE_SYSTEM_SYS},
};

/// fetches the [FileSystemSys] interface
pub fn get_fs() -> &'static FileSystemSys {
    FILE_SYSTEM_SYS.wait()
}

/// virtual file reference for the virtual filesystem in titanfall 2
pub struct VFile {
    inner: FileHandle_t,
}

/// File Reader for [VFile]
pub struct VFileReader<'a> {
    file: &'a VFile,
    fs: &'static FileSystemSys,
}

impl VFile {
    /// returns the inner [FileHandle_t]
    pub const fn get_inner(&self) -> &FileHandle_t {
        &self.inner
    }

    /// constructs a reader for this file
    pub fn reader(&'_ self) -> VFileReader<'_> {
        VFileReader {
            file: self,
            fs: FILE_SYSTEM_SYS.wait(),
        }
    }
}

impl Display for VFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        _ = self.reader().read_to_string(&mut buf);
        f.write_str(&buf)
    }
}

impl Drop for VFile {
    fn drop(&mut self) {
        let fs = FILE_SYSTEM_SYS.wait();
        (fs.vtable2().Close)(fs.get_raw(), self.inner);
    }
}

impl<'a> Read for VFileReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok((self.fs.vtable2().Read)(
            &self.fs.get_raw().vtable2,
            buf.as_mut_ptr().cast(),
            buf.len() as i32,
            self.file.inner,
        ) as usize)
    }
}

/// opens a file in the Source FileSystem
pub fn open_with_options<'a>(
    path: &'a Path,
    options: &CStr,
    path_id: &CStr,
) -> Result<VFile, FsOpenError<'a>> {
    let fs = *FILE_SYSTEM_SYS.wait();

    let cstr_path = CString::from_str(path.to_str().ok_or(FsOpenError::PathToStringFailed(path))?)?;
    if !(fs.vtable2().FileExists)(&fs.get_raw().vtable2, cstr_path.as_ptr(), path_id.as_ptr()) {
        return Err(FsOpenError::NotFound(path));
    }

    let file = (fs.vtable2().Open)(
        &fs.get_raw().vtable2,
        cstr_path.as_ptr(),
        options.as_ptr(),
        path_id.as_ptr(),
        0,
    );

    Ok(VFile { inner: file })
}

/// opens file in vpks in read byte mode
pub fn open<'a>(file: &'a Path) -> Result<VFile, FsOpenError<'a>> {
    open_with_options(file, c"rb", c"GAME")
}

/// mounts a vpk by path in the virtual filesystem
///
/// to mount a vpk by relative or absolute path in the user space filesystem the location of the vpk has to be added to search path or be in a mod
pub fn mount_vpk(vpk_path: &Path) -> Option<&'static VPKData> {
    let fs = *FILE_SYSTEM_SYS.wait();
    unsafe {
        (fs.vtable().MountVPK)(
            fs.get_raw(),
            CString::from_str(vpk_path.to_str()?).ok()?.as_ptr(),
        )
        .as_ref()
    }
}

// incomplete
// fn traverse_files_in_root() {
//     let fs = FILE_SYSTEM_SYS.wait();

//     let mut find_handle = FileFindHandle_t(0);
//     let mut file_path = (fs.vtable().FindFirst)(fs.get_raw(), c"%s".as_ptr(), &mut find_handle);

//     while !file_path.is_null() {
//         log::info!("file: {}", unsafe { from_char_ptr(file_path) });

//         file_path = (fs.vtable().FindNext)(fs.get_raw(), find_handle);
//     }

//     (fs.vtable().FindClose)(fs.get_raw(), find_handle);
// }
