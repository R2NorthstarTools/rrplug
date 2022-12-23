/// # sq_return_string
/// sq_return abstracts away the return procces of a sq function.
///
/// ## expected input
/// value: [`String`], sqvm: [`HSquirrelVM`], sq_functions: [`SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_return_string {
    ($value:expr, $sqvm:expr, $sq_functions:expr) => (
        // boxing this would be a good idea and leaking
        let cstring = std::ffi::CString::new($value.replace("\0", "").as_bytes()).unwrap();
        // its impossble for it to crash since we replace null with space if it does it must be reported
        unsafe { ($sq_functions.sq_pushstring)($sqvm, cstring.as_ptr(), -1) };
        return $crate::bindings::squirrelclasstypes::SQRESULT_SQRESULT_NOTNULL;
    )
}

/// # sq_return_bool
/// sq_return abstracts away the return procces of a sq function.
///
/// ## expected input
/// value: [`bool`], sqvm: [`HSquirrelVM`], sq_functions: [`SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_return_bool {
    ($value:expr, $sqvm:expr, $sq_functions: expr) => {
        unsafe { ($sq_functions.sq_pushbool)($sqvm, if $value { 1 } else { 0 }) };
        return $crate::bindings::squirrelclasstypes::SQRESULT_SQRESULT_NOTNULL;
    };
}

/// # sq_return_int
/// sq_return abstracts away the return procces of a sq function.
///
/// ## expected input
/// value: [`Into<i32>`], sqvm: [`HSquirrelVM`], sq_functions: [`SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_return_int {
    ($value:expr, $sqvm:expr, $sq_functions: expr) => {
        unsafe { ($sq_functions.sq_pushinteger)($sqvm, $value.into()) };
        return $crate::bindings::squirrelclasstypes::SQRESULT_SQRESULT_NOTNULL;
    };
}

/// # sq_return_float
/// sq_return abstracts away the return procces of a sq function.
///
/// ## expected input
/// value: [`Into<f32>`], sqvm: [`HSquirrelVM`], sq_functions: [`SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_return_float {
    ($value:expr, $sqvm:expr, $sq_functions: expr) => {
        unsafe { ($sq_functions.sq_pushfloat)($sqvm, $value.into()) };
        return $crate::bindings::squirrelclasstypes::SQRESULT_SQRESULT_NOTNULL;
    };
}
