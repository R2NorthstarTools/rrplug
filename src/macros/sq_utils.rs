#[cfg(doc)]
use crate::{
    bindings::squirreldatatypes::SQObject,
    wrappers::{
        errors::CallError,
        squirrel::{call_sq_function, call_sq_object_function},
    },
};

/// calls any function defined on the sqvm
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
///
/// macro version of [`crate::high::squirrel::call_sq_function`], used to call a function with args
/// returns `Result<(), CallError>`
///
/// ## example
/// ```no_run
/// #[sqfunction(VM="Server")]
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
                use $crate::high::squirrel_traits::PushToSquirrelVm; // weird
                let mut args_amount = 1;

                let mut obj = Box::new(std::mem::MaybeUninit::<$crate::bindings::squirreldatatypes::SQObject>::zeroed());
                let ptr = obj.as_mut_ptr();

                let function_name = $crate::to_sq_string!(std::convert::Into::<String>::into($function_name));

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
                            args_amount += 1; // I know this runtime bloat but this the only way, proc marcos are hard
                        )*

                        if ($sqfunctions.sq_call)($sqvm, args_amount, true as u32, true as u32) == $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_ERROR  {
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
/// macro version of [`call_sq_object_function`], used to call a function with args
/// returns `Result<(), CallError>`
///
/// ## example
/// ```no_run
/// #[sqfunction(VM="Server")]
/// fn test_call_funcs(func: fn(String)) {
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
                use $crate::high::squirrel_traits::PushToSquirrelVm; // weird
                let mut args_amount = 1;

                let ptr = $obj.as_mut_ptr();
                unsafe {
                    ($sqfunctions.sq_pushobject)($sqvm, ptr);
                    ($sqfunctions.sq_pushroottable)($sqvm);

                    $(
                        $arg.push_to_sqvm($sqvm, $sqfunctions);
                        args_amount += 1; // I know this runtime bloat but this the only way, proc marcos are hard
                    )*

                    let result = if ($sqfunctions.sq_call)($sqvm, args_amount, true as u32, true as u32) == $crate::bindings::squirrelclasstypes::SQRESULT::SQRESULT_ERROR {
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
/// call_sq_function!(sqvm, sq_functions, "SomeSQFunc", 9347).map_err(|err| err.to_string())?;
/// ```
#[macro_export]
macro_rules! async_call_sq_function {
    ($sqvm:expr, $sqfunctions:expr, $function_name:expr, $( $arg:expr ),* ) => {};
}

#[cfg(test)]
mod test {
    use crate as rrplug;
    use rrplug::prelude::*;
    use rrplug_proc::*;

    use rrplug::bindings::squirreldatatypes::SQObject;
    use rrplug::{call_sq_function, call_sq_object_function};

    #[sqfunction(VM = "Server")]
    fn test_call_funcs(func: fn(String)) -> String {
        call_sq_object_function!(sqvm, sq_functions, func, "test".to_string())
            .map_err(|err| err.to_string())?;

        call_sq_function!(sqvm, sq_functions, "SomeSQFunc", 9347).map_err(|err| err.to_string())?;

        Ok("test".to_string())
    }
}
