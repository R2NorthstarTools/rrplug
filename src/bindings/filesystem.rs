#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::ffi::{c_char, c_void};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct FileHandle_t(pub *const c_void);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct FileFindHandle_t(pub u16);

#[repr(C)]
pub struct VPKData {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub enum SearchPathAdd_t {
    PATH_ADD_TO_HEAD, // First path searched
    PATH_ADD_TO_TAIL, // Last path searched
}

#[repr(C)]
pub struct CSearchPath {
    pub unknown: [u8; 0x18],
    pub debugPath: *const c_char,
}

#[repr(C)]
pub struct IFileSystemVTable {
    pub unknown: [usize; 10],
    pub AddSearchPath: extern "C" fn(
        fileSystem: *const IFileSystem,
        pPath: *const c_char,
        pathID: *const c_char,
        addType: SearchPathAdd_t,
    ),
    pub unknown2: [usize; 19],
    pub FindFirst: extern "C" fn(
        fileSystem: *const IFileSystem,
        pWildCard: *const c_char,
        pHandle: *mut FileFindHandle_t,
    ) -> *const c_char,
    pub FindNext:
        extern "C" fn(fileSystem: *const IFileSystem, handle: FileFindHandle_t) -> *const c_char,
    pub FindIsDirectory:
        extern "C" fn(fileSystem: *const IFileSystem, handle: FileFindHandle_t) -> bool,
    pub FindClose: extern "C" fn(fileSystem: *const IFileSystem, handle: FileFindHandle_t),

    // Same as FindFirst, but you can filter by path ID, which can make it faster.
    pub FindFirstEx: extern "C" fn(
        fileSystem: *const IFileSystem,
        pWildCard: *const c_char,
        pPathID: *const c_char,
        pHandle: *mut FileFindHandle_t,
    ) -> *const c_char,
    pub FindFileAbsoluteList: extern "C" fn(
        fileSystem: *const IFileSystem,
        outAbsolutePathNames: *mut c_void,
        pWildCard: *const c_char,
        pPathID: *const c_char,
    ),
    pub GetLocalPath: usize,
    pub FullPathToRelativePath: usize,
    pub GetCurrentDirectory: extern "C" fn(pDirectory: *mut c_char, maxLen: i32) -> bool,
    pub unknown3: [usize; 57],
    pub ReadFromCache: extern "C" fn(
        fileSystem: *const IFileSystem,
        pPath: *const c_char,
        result: *mut c_void,
    ) -> *mut c_void,
    pub unknown4: [usize; 15],
    pub MountVPK:
        extern "C" fn(fileSystem: *const IFileSystem, vpkPath: *const c_char) -> *const VPKData,
}

// NOTE: seemingly passing pathId to anything as null will result in it searching everywhere
#[repr(C)]
pub struct IFileSystemVTable2 {
    pub Read: extern "C" fn(
        fileSystem: *const *const IFileSystemVTable2,
        pOutput: *mut c_void,
        size: i32,
        file: FileHandle_t,
    ) -> i32,
    pub Write: extern "C" fn(
        fileSystem: *const *const IFileSystemVTable2,
        pInput: *mut c_void,
        size: i32,
        file: FileHandle_t,
    ),
    pub Open: extern "C" fn(
        fileSystem: *const *const IFileSystemVTable2,
        pFileName: *const c_char,
        pOptions: *const c_char,
        pathID: *const c_char,
        unknown: i64,
    ) -> FileHandle_t,
    pub Close: extern "C" fn(fileSystem: *const IFileSystem, file: FileHandle_t),
    pub Seek: extern "C" fn(
        fileSystem: *const *const IFileSystemVTable2,
        file: FileHandle_t,
        offset: i64,
        whence: i64,
    ) -> i64,
    Tell: extern "C" fn(file: FileHandle_t) -> u32,
    Size: extern "C" fn(file: FileHandle_t) -> u32,
    Size2: extern "C" fn(pFileName: *const c_char, pPathID: *const c_char) -> u32,
    Flush: extern "C" fn(file: FileHandle_t),
    Precache: extern "C" fn(pFileName: *const c_char, pPathID: *const c_char),
    pub FileExists: extern "C" fn(
        fileSystem: *const *const IFileSystemVTable2,
        pFileName: *const c_char,
        pPathID: *const c_char,
    ) -> bool,
    IsFileWritable: extern "C" fn(pFileName: *const c_char, pPathID: *const c_char) -> bool,
    SetFileWritable:
        extern "C" fn(pFileName: *const c_char, writable: bool, pPathID: *const c_char) -> bool,
    GetFileTime: extern "C" fn(pFileName: *const c_char, pPathID: *const c_char) -> i64,
    // ReadFile: extern "C" fn(
    //     pFileName: *const c_char,
    //     pPat: *const c_char,
    //     buf: *mut CUltBuffer,
    //     nMaxBytes: i32,
    //     nStartingByte: i32,
    //     pfnAlloc: FSAllocFunc_t,
    // ) -> bool, // last can be null or 0
    // WriteFile:
    //     extern "C" fn(pFileName: *const c_char, pPat: *const c_char, buf: *mut CUltBuffer) -> bool,
    // UnzipFile: extern "C" fn(
    //     pFileName: *const c_char,
    //     pPat: *const c_char,
    //     pDestination: *const c_char,
    // ) -> bool,
}

#[repr(C)]
pub struct IFileSystem {
    pub vtable: *const IFileSystemVTable,
    pub vtable2: *const IFileSystemVTable2,
}

// int				Read( void* pOutput, int size, FileHandle_t file ) = 0;
// int				Write( void const* pInput, int size, FileHandle_t file ) = 0;
// FileHandle_t	Open( const char *pFileName, const char *pOptions, const char *pathID = 0 ) = 0;
// void			Close( FileHandle_t file ) = 0;
// void			Seek( FileHandle_t file, int pos, FileSystemSeek_t seekType ) = 0;
// unsigned int	Tell( FileHandle_t file ) = 0;
// unsigned int	Size( FileHandle_t file ) = 0;
// unsigned int	Size( const char *pFileName, const char *pPathID = 0 ) = 0;
// void			Flush( FileHandle_t file ) = 0;
// bool			Precache( const char *pFileName, const char *pPathID = 0 ) = 0;
// bool			FileExists( const char *pFileName, const char *pPathID = 0 ) = 0;
// bool			IsFileWritable( char const *pFileName, const char *pPathID = 0 ) = 0;
// bool			SetFileWritable( char const *pFileName, bool writable, const char *pPathID = 0 ) = 0;
// long			GetFileTime( const char *pFileName, const char *pPathID = 0 ) = 0;
// bool			ReadFile( const char *pFileName, const char *pPath, CUtlBuffer &buf, int nMaxBytes = 0, int nStartingByte = 0, FSAllocFunc_t pfnAlloc = NULL ) = 0;
// bool			WriteFile( const char *pFileName, const char *pPath, CUtlBuffer &buf ) = 0;
// bool			UnzipFile( const char *pFileName, const char *pPath, const char *pDestination ) = 0;
