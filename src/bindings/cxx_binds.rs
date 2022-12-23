
#[cfg(feature = "function_call")]
#[cxx::bridge]
mod function_call {
    unsafe extern "C++" {
        include!("cpp_include/squirrelclasstypes.h");

        type SquirrelMessage;
        
    }
}