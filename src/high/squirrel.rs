#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! squirrel vm related function and statics

use parking_lot::Mutex;
use std::{ffi::c_void, marker::PhantomData};

use super::{
    northstar::{SQFuncInfo, ScriptVmType},
    squirrel_traits::IsSQObject,
    UnsafeHandle,
};
use crate::{
    bindings::{
        squirrelclasstypes::{CompileBufferState, SQRESULT},
        squirreldatatypes::{CSquirrelVM, HSquirrelVM, SQClosure, SQObject},
        unwraped::SquirrelFunctionsUnwraped,
    },
    errors::{CallError, SQCompileError},
    mid::squirrel::SQFUNCTIONS,
    to_c_string,
};

#[doc(hidden)]
pub static FUNCTION_SQ_REGISTER: Mutex<Vec<SQFuncInfo>> = Mutex::new(Vec::new());

/// holds a pointer to [`CSquirrelVM`] and provides a api to interact with systems that require [`CSquirrelVM`]
///
/// also has the current vm type
#[derive(Debug)]
pub struct CSquirrelVMHandle {
    handle: *mut CSquirrelVM,
    vm_type: ScriptVmType,
}

impl CSquirrelVMHandle {
    /// **should** not be used outside of the [`crate::entry`] macro
    #[doc(hidden)]
    pub fn new(handle: *mut CSquirrelVM, vm_type: ScriptVmType) -> Self {
        Self { handle, vm_type }
    }

    /// defines a constant on the sqvm
    ///
    /// Like `SERVER`, `CLIENT`, `UI`, etc
    pub fn define_sq_constant(&self, name: String, value: impl Into<i32>) {
        let sqfunctions = if self.vm_type == ScriptVmType::Server {
            SQFUNCTIONS.server.wait()
        } else {
            SQFUNCTIONS.client.wait()
        };

        // not sure if I need to leak this
        let name = to_c_string!(name);

        unsafe { (sqfunctions.sq_defconst)(self.handle, name.as_ptr(), value.into()) }
    }

    /// gets the raw pointer to [`HSquirrelVM`]
    ///
    /// # Safety
    /// assumes its valid
    ///
    /// it is not valid after sqvm destruction
    ///
    /// [`UnsafeHandle`] : when used outside of engine thread can cause race conditions or ub
    ///
    /// [`UnsafeHandle`] should only be used to transfer the pointers to other places in the engine thread like sqfunctions or runframe
    pub unsafe fn get_sqvm(&self) -> UnsafeHandle<*mut HSquirrelVM> {
        unsafe { UnsafeHandle::internal_new((*self.handle).sqvm) }
    }
    /// gets the raw pointer to [`CSquirrelVM`]
    ///
    /// # Safety
    /// assumes its valid
    ///
    /// it is not valid after sqvm destruction
    ///
    /// [`UnsafeHandle`] : when used outside of engine thread can cause race conditions or ub
    ///
    /// [`UnsafeHandle`] should only be used to transfer the pointers to other places in the engine thread like sqfunctions or runframe
    pub unsafe fn get_cs_sqvm(&self) -> UnsafeHandle<*mut CSquirrelVM> {
        UnsafeHandle::internal_new(self.handle)
    }

    /// gets the type of the sqvm :D
    pub fn get_context(&self) -> ScriptVmType {
        self.vm_type
    }
}

/// runtime check for [`SQObject`] types
pub struct SQHandle<H: IsSQObject> {
    inner: SQObject,
    marker: PhantomData<H>,
}

impl<H: IsSQObject> SQHandle<H> {
    /// creates a new [`SQHandle`] by checking if the sqobject has the correct type at runtime
    pub fn new(value: SQObject) -> Result<Self, SQObject> {
        let ty = value._Type;
        if ty == H::OT_TYPE || ty == H::RT_TYPE {
            Ok(Self {
                inner: value,
                marker: PhantomData,
            })
        } else {
            Err(value)
        }
    }

    /// creates a new [`SQHandle`] without checking the type
    /// # Safety
    ///
    /// this breaks the type guarantees provided by this struct
    pub unsafe fn new_unchecked(value: SQObject) -> Self {
        Self {
            inner: value,
            marker: PhantomData,
        }
    }

    /// a getter
    pub fn get(&self) -> &SQObject {
        &self.inner
    }

    /// a mut getter
    pub fn get_mut(&mut self) -> &mut SQObject {
        &mut self.inner
    }

    /// consumes itself and returns the [`SQObject`]
    pub fn take(self) -> SQObject {
        self.inner
    }
}

impl SQHandle<SQClosure> {
    /// used in some macros to enforce type safety
    pub fn as_callable(&mut self) -> *mut SQObject {
        &mut self.inner as *mut SQObject
    }
}

/// calls any function defined on the sqvm
///
/// they would only run when the sqvm is valid
pub fn async_call_sq_function<T>(
    context: ScriptVmType,
    function_name: impl Into<String>,
    callback: Option<T>,
) where
    T: FnOnce(*mut HSquirrelVM, &'static SquirrelFunctionsUnwraped) -> i32,
{
    type PayloadType<T> = (T, &'static SquirrelFunctionsUnwraped);
    type BoxedPayloadType<T> = Box<PayloadType<T>>;

    let sqfunctions = match context {
        ScriptVmType::Server => SQFUNCTIONS.server.get(),
        _ => SQFUNCTIONS.client.get(),
    }
    .expect("SQFUNCTIONS should be initialized at this point");

    let c_callback = callback.as_ref().map(|_| {
        callback_trampoline::<T>
            as unsafe extern "C" fn(sqvm: *mut HSquirrelVM, userdata: *mut c_void) -> i32
    });

    let userdata: *mut c_void = match callback {
        Some(callback) => {
            let payload: BoxedPayloadType<T> = Box::new((callback, sqfunctions));
            Box::into_raw(payload) as *mut c_void
        }
        None => std::ptr::null_mut(),
    };

    let function_name = to_c_string!(function_name.into());

    unsafe {
        (sqfunctions.sq_schedule_call_external)(
            context.into(),
            function_name.as_ptr(),
            c_callback,
            userdata,
        )
    }

    unsafe extern "C" fn callback_trampoline<T>(
        sqvm: *mut HSquirrelVM,
        userdata: *mut c_void,
    ) -> i32
    where
        T: FnOnce(*mut HSquirrelVM, &'static SquirrelFunctionsUnwraped) -> i32,
    {
        unsafe {
            match (userdata as *mut PayloadType<T>).as_mut() {
                Some(closure) => {
                    let boxed_tuple: BoxedPayloadType<T> = Box::from_raw(closure);
                    (boxed_tuple.0)(sqvm, (*boxed_tuple).1)
                }
                None => 0,
            }
        }
    }
}

/// calls any function defined on the sqvm
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
///
/// this only allows calls without args use the marco [`crate::call_sq_function`] instead if you want args
///
/// # Panic
///
/// this function can panic if the return value is not [`()`] while begin called outside of a sqvm context
///
/// examples of "outside of a sqvm context"
/// - runframe
/// - callbacks for concommands and convars
///
/// # Example
///
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::high::squirrel::call_sq_function;
/// # use rrplug::{high::squirrel::SQHandle,bindings::squirreldatatypes::SQClosure};
///  
/// #[rrplug::sqfunction(VM="Server")]
/// fn test_call_sq_object_function() -> Result<(),String> {
///     call_sq_function(sqvm, sq_functions, "someFunction").map_err(|err| err.to_string())?;
///
///     Ok(())
/// }
/// ```
pub fn call_sq_function(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    function_name: impl Into<String>,
) -> Result<(), CallError> {
    let mut obj = std::mem::MaybeUninit::<SQObject>::zeroed();
    let ptr = obj.as_mut_ptr();

    let function_name = to_c_string!(function_name.into());

    let result = unsafe {
        (sqfunctions.sq_getfunction)(sqvm, function_name.as_ptr(), ptr, std::ptr::null())
    };

    if result != 0 {
        Err(CallError::FunctionNotFound(
            function_name.to_string_lossy().into(),
        )) // totaly safe :clueless:
    } else {
        _call_sq_object_function(sqvm, sqfunctions, ptr)
    }
}

/// calls any function defined on the sqvm from its [`SQObject`]
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
///
/// this only allows calls without args use the marco [`crate::call_sq_object_function`] instead if you want args
///
/// # Panic
///
/// this function can panic if the return value is not [`()`] while begin called outside of a sqvm context
///
/// examples of "outside of a sqvm context"
/// - runframe
/// - callbacks for concommands and convars
///
/// # Example
///
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::high::squirrel::call_sq_object_function;
/// # use rrplug::{high::squirrel::SQHandle,bindings::squirreldatatypes::SQClosure};
///  
/// #[rrplug::sqfunction(VM="Server")]
/// fn call_sqvm_function(mut func: SQHandle<SQClosure>) -> Result<(),String>{
///     call_sq_object_function(sqvm, sq_functions, func).map_err(|err| err.to_string())?;
///
///     Ok(())
/// }
/// ```
pub fn call_sq_object_function(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    mut obj: SQHandle<SQClosure>,
) -> Result<(), CallError> {
    _call_sq_object_function(sqvm, sqfunctions, obj.as_callable())
}

#[inline]
fn _call_sq_object_function(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    ptr: *mut SQObject,
) -> Result<(), CallError> {
    unsafe {
        (sqfunctions.sq_pushobject)(sqvm, ptr);
        (sqfunctions.sq_pushroottable)(sqvm);

        if (sqfunctions.sq_call)(sqvm, 1, true as u32, true as u32) == SQRESULT::SQRESULT_ERROR {
            Err(CallError::FunctionFailedToExecute)
        } else {
            // Ok(R::get_from_sqvm(
            //     sqvm,
            //     sqfunctions,
            //     (*sqvm)._stackbase,
            // )) // literally imposible to get null sqvm or it would have crashed before
            Ok(())
        }
    }
}

/// compiles a string and runs it on the provided sqvm
///
/// ## Example
///
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::high::squirrel::compile_string;
///  
/// #[rrplug::sqfunction(VM="Server")]
/// fn compile_string_test() -> Result<(),String> {
///     compile_string(sqvm, sq_functions, true, "print(\"helloworld\")").map_err(|err| err.to_string())?;
///
///     Ok(())
/// }
///
/// ```
pub fn compile_string(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    should_throw_error: bool,
    code: impl Into<String>,
) -> Result<(), SQCompileError> {
    let buffer = to_c_string!(code.into());

    let mut compile_buffer = CompileBufferState {
        buffer: buffer.as_ptr(),
        bufferPlusLength: (buffer.as_ptr() as usize + buffer.as_bytes().len()) as *const i8,
        bufferAgain: buffer.as_ptr(),
    };

    let name = unsafe { to_c_string!(const "compile_string\0") };

    unsafe {
        let result = (sqfunctions.sq_compilebuffer)(
            sqvm,
            &mut compile_buffer as *mut CompileBufferState,
            name.as_ptr(),
            -1,
            should_throw_error as u32,
        );

        if result != SQRESULT::SQRESULT_ERROR {
            (sqfunctions.sq_pushroottable)(sqvm);

            if (sqfunctions.sq_call)(sqvm, 1, 0, 0) == SQRESULT::SQRESULT_ERROR {
                Err(SQCompileError::BufferFailedToExecute)
            } else {
                Ok(())
            }
        } else {
            Err(SQCompileError::CompileError)
        }
    }
}
