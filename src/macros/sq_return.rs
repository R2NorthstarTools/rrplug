//! macros for deprecated version of [`crate::sqfunction`]

#[cfg(doc)]
use {
    crate::{bindings::squirreldatatypes::HSquirrelVM, high::vector::Vector3},
    std::ffi::{CStr, CString},
};

/// use [`crate::to_c_string`]
#[deprecated = "the name is kinda misleading. use to_c_string instead."]
#[macro_export]
macro_rules! to_sq_string {
    ($value:expr) => {
        std::ffi::CString::new($value.replace("\0", "").as_bytes()).unwrap()
    };
}

/// transforms a [`String`] into a null terminated string
///
/// ### (const $value:literal) => {}
/// creates a [`CStr`] from [`&str`], the string must be terminated manually by `\0`
///
/// ### (unwrap $value:expr) => {}
/// creates a [`CString`] from [`String`], will panic if a `\0` is found in the string
///
/// ### ($value:expr) => {}
/// creates a [`CString`] from [`String`], removes `\0`
#[macro_export]
macro_rules! to_c_string {
    // this must documented since const literals must have \0 at the end
    (const $value:literal) => {
        std::ffi::CStr::from_ptr(($value as *const _ as *const i8).cast_mut())
    };

    (unwrap $value:expr) => {
        std::ffi::CString::new($value).unwrap()
    };

    ($value:expr) => {
        std::ffi::CString::new($value.replace("\0", "").as_bytes()).unwrap()
    };
}

/// # sq_return_string
/// sq_return abstracts away the return procces of a sq functionv.
///
/// ## expected input
/// value: [`String`], sqvm: [`HSquirrelVM`], sq_functions: [`SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_return_string {
    ($value:expr, $sqvm:expr, $sq_functions:expr) => {
        $crate::wrappers::squirrel::push_sq_string($sqvm, $sq_functions, $value);
        return $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NOTNULL
    };
}

/// # sq_return_bool
/// sq_return abstracts away the return procces of a sq function.
///
/// ## expected input
/// value: [`bool`], sqvm: [`HSquirrelVM`], sq_functions: [`SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_return_bool {
    ($value:expr, $sqvm:expr, $sq_functions: expr) => {
        $crate::wrappers::squirrel::push_sq_bool($sqvm, $sq_functions, $value);
        return $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NOTNULL
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
        $crate::wrappers::squirrel::push_sq_int($sqvm, $sq_functions, $value);
        return $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NOTNULL
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
        $crate::wrappers::squirrel::push_sq_float($sqvm, $sq_functions, $value);
        return $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NOTNULL
    };
}

/// # sq_return_vector
/// sq_return abstracts away the return procces of a sq function.
///
/// ## expected input
/// value: [`Vector3`], sqvm: [`HSquirrelVM`], sq_functions: [`SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_return_vector {
    ($value:expr, $sqvm:expr, $sq_functions: expr) => {
        $crate::wrappers::squirrel::push_sq_vector($sqvm, $sq_functions, $value);
        return $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NOTNULL
    };
}

/// # sq_return_null
/// just returns null
#[macro_export]
macro_rules! sq_return_null {
    () => {
        return $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NULL
    };
}

/// # sq_return_notnull
/// just tells squirrel that you returned smth (I trust you to use this correctly)
#[macro_export]
macro_rules! sq_return_notnull {
    () => {
        return $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NOTNULL
    };
}

/// # sq_raise_error
/// yeets an error into the sqvm
///
/// doesn't work for some reason
///
/// ## expected input
/// value: [`String`], sqvm: [`HSquirrelVM`], sq_functions: [`SquirrelFunctionsUnwraped`]
#[macro_export]
macro_rules! sq_raise_error {
    ($value:expr, $sqvm:expr, $sq_functions: expr) => {
        let err = $crate::to_c_string!($value);
        unsafe { ($sq_functions.sq_raiseerror)($sqvm, err.as_ptr()) };
        return $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_ERROR
    };
    ($value:expr, $sqvm:expr, $sq_functions: expr, noreturn) => {
        unsafe { ($sq_functions.sq_raiseerror)($sqvm, $crate::to_c_string!($value).as_ptr()) };
    };
}
