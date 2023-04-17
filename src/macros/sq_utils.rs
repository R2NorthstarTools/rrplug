#[cfg(doc)]
use crate::wrappers::{
    errors::CallError,
    squirrel::{call_sq_function, call_sq_object_function},
};

/// "safely" calls any function defined on the sqvm
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
///
/// macro version of [`call_sq_function`], used to call a function with args
/// returns `Result<(), CallError>`
#[macro_export]
macro_rules! call_sq_function {
    ($sqvm:expr, $sqfunctions:expr, $function_name:expr, $( $arg:expr ),* ) => (
        {
            {
                use rrplug::wrappers::squirrel::PushToSquirrelVm; // weird
                let mut args_amount = 1;

                let mut obj = Box::new(std::mem::MaybeUninit::<$crate::bindings::squirreldatatypes::SQObject>::zeroed());
                let ptr = obj.as_mut_ptr();

                let function_name = $crate::to_sq_string!(std::convert::Into::<String>::into($function_name));

                let result = unsafe {
                    ($sqfunctions.sq_getfunction)($sqvm, function_name.as_ptr(), ptr, std::ptr::null())
                };

                if result != 0 {
                    Err($crate::wrappers::errors::CallError::FunctionNotFound(function_name.to_string_lossy().into())) // totaly safe :clueless:
                } else {
                    unsafe {
                        ($sqfunctions.sq_pushobject)($sqvm, ptr);
                        ($sqfunctions.sq_pushroottable)($sqvm);

                        $(
                            $arg.push_to_sqvm($sqvm, $sqfunctions);
                            args_amount += 1; // I know this runtime bloat but this the only way, proc marcos are hard
                        )*

                        let result = if ($sqfunctions.sq_call)($sqvm, args_amount, false as u32, false as u32) == -1 {
                            Err($crate::wrappers::errors::CallError::FunctionFailedToExecute)
                        } else {
                            Ok(())
                        };

                        result
                    }
                }
            }
        }
    )
}

/// "safely" calls any function defined on the sqvm from its [`SQObject`]
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
///
/// macro version of [`call_sq_object_function`], used to call a function with args
/// returns `Result<(), CallError>`
#[macro_export]
macro_rules! call_sq_object_function {
    ($sqvm:expr, $sqfunctions:expr, $obj:expr, $( $arg:expr ),* ) => (
        {
            {
                use rrplug::wrappers::squirrel::PushToSquirrelVm; // weird
                let mut args_amount = 1;

                let ptr = $obj.as_mut_ptr();
                unsafe {
                    ($sqfunctions.sq_pushobject)($sqvm, ptr);
                    ($sqfunctions.sq_pushroottable)($sqvm);

                    $(
                        $arg.push_to_sqvm($sqvm, $sqfunctions);
                        args_amount += 1; // I know this runtime bloat but this the only way, proc marcos are hard
                    )*

                    let result = if ($sqfunctions.sq_call)($sqvm, args_amount, false as u32, false as u32) == -1 {
                        Err($crate::wrappers::errors::CallError::FunctionFailedToExecute)
                    } else {
                        Ok(())
                    };

                    result
                }
            }
        }
    )
}
