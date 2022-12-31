#[cfg(feature = "concommand")]
#[cxx::bridge]
pub(crate) mod __concommand {
    unsafe extern "C++" {
        include!("cpp_include/concommand.h");
        
        #[cxx_name = "ccommand"]
        type CCommand;
        
        fn ArgS(self: &CCommand) -> *const c_char;
        fn GetCommandString(self: &CCommand) -> *const c_char;        
    }
}