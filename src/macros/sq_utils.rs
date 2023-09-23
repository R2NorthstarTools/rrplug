//! various macros to help call squirrel functions
//!
//! for non async calls the args are not checked to be correct at compile time

#[cfg(doc)]
use crate::{
    bindings::squirreldatatypes::SQObject,
    errors::CallError,
    high::squirrel::{call_sq_function, call_sq_object_function},
};

/// calls any function defined on the sqvm
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
///
/// macro version of [`crate::high::squirrel::call_sq_function`], used to call a function with args
/// returns `Result<(), CallError>`
///
/// ## example
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::call_sq_function;
///  
/// #[rrplug::sqfunction(VM="Server")]
/// fn test_call_funcs() {
///     call_sq_function!(sqvm, sq_functions, "SomeSQFunc", 9347).map_err(|err| err.to_string())?;
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! call_sq_function {
    ($sqvm:expr, $sqfunctions:expr, $function_name:expr, $( $arg:expr ),* ) => (
        {
            {
                use $crate::high::squirrel_traits::PushToSquirrelVm;
                const ARGS_AMOUNT: i32 = 1 + $crate::macros::sq_utils::__arg_count_helper([$($crate::__replace_expr!($arg)),*]) as i32;

                let mut obj = std::mem::MaybeUninit::<$crate::bindings::squirreldatatypes::SQObject>::zeroed();
                let ptr = obj.as_mut_ptr();

                let function_name = $crate::to_c_string!(std::convert::Into::<String>::into($function_name));

                let result = unsafe {
                    ($sqfunctions.sq_getfunction)($sqvm, function_name.as_ptr(), ptr, std::ptr::null())
                };

                if result != 0 {
                    Err($crate::errors::CallError::FunctionNotFound(function_name.to_string_lossy().into())) // totaly safe :clueless:
                } else {
                    unsafe {
                        ($sqfunctions.sq_pushobject)($sqvm, ptr);
                        ($sqfunctions.sq_pushroottable)($sqvm);

                        $(
                            $arg.push_to_sqvm($sqvm, $sqfunctions);
                        )*

                        if ($sqfunctions.sq_call)($sqvm, ARGS_AMOUNT, true as u32, true as u32) == $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_ERROR  {
                            Err($crate::errors::CallError::FunctionFailedToExecute)
                        } else {
                            Ok(())
                        }
                    }
                }
            }
        }
    )
}

/// calls any function defined on the sqvm from its [`SQObject`]
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
///
/// macro version of [`crate::high::squirrel::call_sq_object_function`], used to call a function with args
/// returns `Result<(), CallError>`
///
/// ## example
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::call_sq_object_function;
///
/// #[rrplug::sqfunction(VM="Server")]
/// fn test_call_funcs(mut func: fn(String)) {
///     call_sq_object_function!(sqvm, sq_functions, func, "test".to_string()).map_err(|err| err.to_string())?;
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! call_sq_object_function {
    ($sqvm:expr, $sqfunctions:expr, $obj:expr, $( $arg:expr ),* ) => (
        {
            {
                use $crate::high::squirrel_traits::PushToSquirrelVm;
                const ARGS_AMOUNT: i32 = 1 + $crate::macros::sq_utils::__arg_count_helper([$($crate::__replace_expr!($arg)),*]) as i32;

                let sqfunctions = $sqfunctions;
                let sqvm = $sqvm;
                let ptr = $obj.as_callable();
                unsafe {
                    (sqfunctions.sq_pushobject)(sqvm, ptr);
                    (sqfunctions.sq_pushroottable)(sqvm);

                    $(
                        $arg.push_to_sqvm(sqvm, sqfunctions);
                    )*

                    let result = if (sqfunctions.sq_call)(sqvm, ARGS_AMOUNT, true as u32, true as u32) == $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_ERROR {
                        Err($crate::errors::CallError::FunctionFailedToExecute)
                    } else {
                        Ok(())
                    };

                    result
                }
            }
        }
    )
}

/// calls any function defined on the sqvm
/// the call will happen on the next engine frame
///
/// macro version of [`crate::high::squirrel::async_call_sq_function`], used to call a function with args
///
/// ## example
/// ```no_run
/// # use rrplug::prelude::*;
///
/// rrplug::async_call_sq_function!(ScriptVmType::Client, "SomeSQFunc", 9347, "Test".to_string());
/// ```
#[macro_export]
macro_rules! async_call_sq_function {
    ($context:expr, $function_name:expr, $( $arg:expr ),* ) => (
        $crate::high::squirrel::async_call_sq_function(
            $context,
            $function_name,
            Some( Box::new( move |sqvm,sqfunctions| {
                use $crate::high::squirrel_traits::PushToSquirrelVm;
                $(
                    $arg.push_to_sqvm(sqvm, sqfunctions);
                )*

                $crate::macros::sq_utils::__arg_count_helper([$($crate::__replace_expr!($arg)),*]) as i32
            } )),
        )
    );
}

/// internal macro used in counting args in some macros
#[doc(hidden)]
#[macro_export]
macro_rules! __replace_expr {
    ($_t:tt) => {
        ()
    };
}

/// internal const function to count args in some macros
#[doc(hidden)]
pub const fn __arg_count_helper<const N: usize>(_: [(); N]) -> usize {
    N
}

#[cfg(test)]
mod test {
    #![allow(unused_mut)]
    use crate as rrplug;
    use rrplug::prelude::*;
    use rrplug_proc::*;

    use rrplug::{async_call_sq_function, call_sq_function, call_sq_object_function};

    #[sqfunction(VM = "Server")]
    fn test_call_funcs2(mut func: fn(String), test: String) -> String {
        call_sq_object_function!(sqvm, sq_functions, func, "test".to_string())
            .map_err(|err| err.to_string())?;

        call_sq_function!(sqvm, sq_functions, "SomeSQFunc", 9347, 3892, 23423)
            .map_err(|err| err.to_string())?;

        async_call_sq_function!(ScriptVmType::Server, "SomeSQFunc", test, 9347);

        Ok("test".to_string())
    }
}
