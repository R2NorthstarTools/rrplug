#[macro_export]
macro_rules! to_sq_string {
    ($value:expr) => (
        std::ffi::CString::new($value.replace("\0", "").as_bytes()).unwrap()
    )
}

/// # sq_return_string
/// sq_return abstracts away the return procces of a sq functionv.
///
/// ## expected input
/// value: [`String`], sqvm: [`crate::bindings::squirreldatatypes::HSquirrelVM`], sq_functions: [`crate::bindings::unwraped::SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_return_string {
    ($value:expr, $sqvm:expr, $sq_functions:expr) => (
        // boxing this would be a good idea and leaking
        let cstring = to_sq_string!($value);
        // its impossble for it to crash since we replace null with space if it does it must be reported
        unsafe { ($sq_functions.sq_pushstring)($sqvm, cstring.as_ptr(), -1) };
        return $crate::bindings::squirrelclasstypes::SQRESULT_SQRESULT_NOTNULL;
    )
}

/// # sq_return_bool
/// sq_return abstracts away the return procces of a sq function.
///
/// ## expected input
/// value: [`bool`], sqvm: [`crate::bindings::squirreldatatypes::HSquirrelVM`], sq_functions: [`crate::bindings::unwraped::SquirrelFunctionsUnwraped`]
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
/// value: [`Into<i32>`], sqvm: [`crate::bindings::squirreldatatypes::HSquirrelVM`], sq_functions: [`crate::bindings::unwraped::SquirrelFunctionsUnwraped`]
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
/// value: [`Into<f32>`], sqvm: [`crate::bindings::squirreldatatypes::HSquirrelVM`], sq_functions: [`crate::bindings::unwraped::SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_return_float {
    ($value:expr, $sqvm:expr, $sq_functions: expr) => {
        unsafe { ($sq_functions.sq_pushfloat)($sqvm, $value.into()) };
        return $crate::bindings::squirrelclasstypes::SQRESULT_SQRESULT_NOTNULL;
    };
}

/// # sq_return_vecotr
/// sq_return abstracts away the return procces of a sq function.
///
/// ## expected input
/// value: [`Vector3`], sqvm: [`crate::bindings::squirreldatatypes::HSquirrelVM`], sq_functions: [`crate::bindings::unwraped::SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_return_vecotr {
    ($value:expr, $sqvm:expr, $sq_functions: expr) => {
        unsafe { ($sq_functions.sq_pushvector)($sqvm, $value.into()) };
        return $crate::bindings::squirrelclasstypes::SQRESULT_SQRESULT_NOTNULL;
    };
}

/// # sq_return_null
/// just returns null
#[macro_export]
macro_rules! sq_return_null {
    () => {
        return $crate::bindings::squirrelclasstypes::SQRESULT_SQRESULT_NULL
    };
}

/// # sq_raise_error
/// yeets an error into the sqvm
/// 
/// doesn't work for some reason
///
/// ## expected input
/// value: [`String`], sqvm: [`crate::bindings::squirreldatatypes::HSquirrelVM`], sq_functions: [`crate::bindings::unwraped::SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_raise_error {
    ($value:expr, $sqvm:expr, $sq_functions: expr) => {
        let err = to_sq_string!($value);
        unsafe { ($sq_functions.sq_raiseerror)($sqvm, err.as_ptr()) };
        return $crate::bindings::squirrelclasstypes::SQRESULT_SQRESULT_ERROR;
    };
}
