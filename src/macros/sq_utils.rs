#[cfg(doc)]
use crate::{
    bindings::squirreldatatypes::SQObject,
    wrappers::{
        errors::CallError,
        squirrel::{call_sq_function, call_sq_object_function},
    },
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
                use rrplug::high::squirrel_traits::PushToSquirrelVm; // weird
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
                use rrplug::high::squirrel_traits::PushToSquirrelVm; // weird
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

#[cfg(test)]
mod test {
    use crate as rrplug;
    use rrplug::prelude::*;
    use rrplug_proc::*;

    use rrplug::bindings::squirreldatatypes::SQObject;
    use rrplug::{call_sq_function, call_sq_object_function};

    #[sqfunction(VM=Server)]
    fn test_call_funcse(func: fn(String)) -> String {
        call_sq_object_function!(sqvm, sq_functions, func, "test".to_string()).unwrap();

        call_sq_function!(sqvm, sq_functions, "SomeSQFunc", 9347).unwrap();

        Ok("test".to_string())
    }

    #[doc(hidden)]
extern "C" fn sq_func_test_call_funcs(
    sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM,
) -> rrplug::bindings::squirrelclasstypes::SQRESULT {
    let sq_functions = SQFUNCTIONS.server.wait();
    let mut func = unsafe {
        let mut obj = Box::new(std::mem::MaybeUninit::<SQObject>::zeroed());
        (sq_functions.sq_getobject)(sqvm, 1i32, obj.as_mut_ptr());
        obj
    };
    fn inner_function(
        sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM,
        sq_functions: &SquirrelFunctionsUnwraped,
        mut func: Box<std::mem::MaybeUninit<SQObject>>,
    ) -> Result<String, String> {
        call_sq_object_function!(sqvm, sq_functions, func, "test".to_string()).unwrap();
        call_sq_function!(sqvm, sq_functions, "SomeSQFunc", 9347).unwrap();
        Ok("test".to_string())
    }
    match inner_function(sqvm, sq_functions, func) {
        Ok(output) => {
            use rrplug::high::squirrel_traits::PushToSquirrelVm;
            output.push_to_sqvm(sqvm, sq_functions);
            rrplug::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NOTNULL
        }
        Err(err) => {
            let err = rrplug::to_sq_string!(err);
            unsafe { (sq_functions.sq_raiseerror)(sqvm, err.as_ptr()) };
            rrplug::bindings::squirrelclasstypes::SQRESULT::SQRESULT_ERROR
        }
    }
}
const fn test_call_funcs() -> rrplug::high::northstar::SQFuncInfo {
    rrplug::high::northstar::SQFuncInfo {
        cpp_func_name: "test_call_funcs",
        sq_func_name: "test_call_funcs",
        types: "void functionref( string ) func",
        return_type: "string",
        vm: ScriptVmType::Server,
        function: Some(sq_func_test_call_funcs),
    }
}
}
