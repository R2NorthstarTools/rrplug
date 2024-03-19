#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! squirrel vm related function and statics

use parking_lot::Mutex;
use std::marker::PhantomData;

use super::{
    squirrel_traits::{GetFromSquirrelVm, IntoSquirrelArgs, IsSQObject},
    UnsafeHandle,
};
use crate::{
    bindings::{
        squirrelclasstypes::{CompileBufferState, SQRESULT},
        squirreldatatypes::{CSquirrelVM, HSquirrelVM, SQClosure, SQObject},
        squirrelfunctions::SquirrelFunctions,
    },
    errors::{CallError, SQCompileError},
    mid::{
        squirrel::{FuncSQFuncInfo, SQFuncInfo, SQFUNCTIONS, SQVM_CLIENT, SQVM_SERVER, SQVM_UI},
        utils::{to_cstring, try_cstring},
    },
    prelude::{EngineToken, ScriptContext},
};

#[doc(hidden)]
pub static FUNCTION_SQ_REGISTER: Mutex<Vec<SQFuncInfo>> = Mutex::new(Vec::new());

/// holds a pointer to [`CSquirrelVM`] and provides a api to interact with systems that require [`CSquirrelVM`]
///
/// also has the current vm type
#[derive(Debug)]
pub struct CSquirrelVMHandle {
    handle: *mut CSquirrelVM,
    vm_type: ScriptContext,
}

impl CSquirrelVMHandle {
    /// **should** not be used outside of the [`crate::entry`] macro
    #[doc(hidden)]
    pub fn new(
        handle: *mut CSquirrelVM,
        context: ScriptContext,
        is_being_dropped: bool,
        token: EngineToken,
    ) -> Self {
        unsafe {
            match (context, is_being_dropped) {
                (ScriptContext::SERVER, false) => {
                    _ = SQVM_SERVER.get(token).replace(Some((*handle).sqvm))
                }
                (ScriptContext::SERVER, true) => _ = SQVM_SERVER.get(token).replace(None),
                (ScriptContext::CLIENT, false) => {
                    _ = SQVM_CLIENT.get(token).replace(Some((*handle).sqvm))
                }
                (ScriptContext::CLIENT, true) => _ = SQVM_CLIENT.get(token).replace(None),
                (ScriptContext::UI, false) => _ = SQVM_UI.get(token).replace(Some((*handle).sqvm)),
                (ScriptContext::UI, true) => _ = SQVM_UI.get(token).replace(None),
            }
        }
        Self {
            handle,
            vm_type: context,
        }
    }

    /// defines a constant on the sqvm
    ///
    /// Like `SERVER`, `CLIENT`, `UI`, etc
    ///
    /// # Panics
    /// will panic if the name has a null char
    pub fn define_sq_constant(&self, name: String, value: impl Into<i32>) {
        let sqfunctions = if self.vm_type == ScriptContext::SERVER {
            SQFUNCTIONS.server.wait()
        } else {
            SQFUNCTIONS.client.wait()
        };

        let name = to_cstring(&name);

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
    pub const unsafe fn get_cs_sqvm(&self) -> UnsafeHandle<*mut CSquirrelVM> {
        UnsafeHandle::internal_new(self.handle)
    }

    /// gets the type of the sqvm :D
    pub const fn get_context(&self) -> ScriptContext {
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
    pub const unsafe fn new_unchecked(value: SQObject) -> Self {
        Self {
            inner: value,
            marker: PhantomData,
        }
    }

    /// a getter
    pub const fn get(&self) -> &SQObject {
        &self.inner
    }

    /// a mut getter
    pub fn get_mut(&mut self) -> &mut SQObject {
        &mut self.inner
    }

    /// consumes itself and returns the [`SQObject`]
    pub const fn take(self) -> SQObject {
        self.inner
    }
}

impl SQHandle<SQClosure> {
    /// used in some macros to enforce type safety
    pub fn as_callable(&mut self) -> *mut SQObject {
        &mut self.inner as *mut SQObject
    }
}

/// provides invariance for calling squirrel functions with little overhead
pub struct SquirrelFn<T: IntoSquirrelArgs> {
    pub(crate) func: SQHandle<SQClosure>,
    pub(crate) phantom: PhantomData<*mut T>,
}

impl<T: IntoSquirrelArgs> SquirrelFn<T> {
    /// calls the underlying squirrel function on the provided sqvm
    ///
    /// # Errors
    ///
    /// This function will return an error if the fails to execute for some reason which is unlikely since it would be type checked
    pub fn run(
        &mut self,
        sqvm: *mut HSquirrelVM,
        sqfunctions: &'static SquirrelFunctions,
        args: T,
    ) -> Result<(), CallError> {
        unsafe {
            let amount = args.into_push(sqvm, sqfunctions);

            (sqfunctions.sq_pushobject)(sqvm, self.func.as_callable());
            (sqfunctions.sq_pushroottable)(sqvm);

            if (sqfunctions.sq_call)(sqvm, amount, true as u32, true as u32)
                == SQRESULT::SQRESULT_ERROR
            {
                return Err(CallError::FunctionFailedToExecute);
            }
        }
        Ok(())
    }

    /// calls the underlying squirrel function on the provided sqvm
    ///
    /// # Errors
    ///
    /// This function will return an error if the fails to execute for some reason which is unlikely since it would be type checked
    pub fn call(
        &mut self,
        sqvm: *mut HSquirrelVM,
        sqfunctions: &'static SquirrelFunctions,
        args: T,
    ) -> Result<(), CallError> {
        self.run(sqvm, sqfunctions, args)
    }
}

/// Adds a sqfunction to the registration list
///
/// The sqfunction will be registered when its vm is loaded
///
/// # Example
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::high::squirrel::call_sq_function;
/// # use rrplug::bindings::class_types::cplayer::CPlayer;
/// #[rrplug::sqfunction(VM="Server")]
/// fn simple_example(name: String, player: Option<&mut CPlayer>) -> Result<(),String> {
///     let _player = player.ok_or("Not a Player!".to_string())?;
///
///     log::info!("the name is {name}");
///     Ok(())
/// }
/// ```
pub fn register_sq_functions(get_info_func: FuncSQFuncInfo) {
    FUNCTION_SQ_REGISTER.lock().push(get_info_func());
}

// TODO: use IntoSquirrelArgs here

/// calls any function defined on the sqvm
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
///
/// this only allows calls without args use the marco [`crate::call_sq_function`] instead if you want args
///
/// # Crashes
///
/// If the return value is incorrect the current api may segfault
///
/// # Example
///
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::high::squirrel::call_sq_function;
/// # use rrplug::{high::squirrel::SQHandle,bindings::squirreldatatypes::SQClosure};
/// #[rrplug::sqfunction(VM="Server")]
/// fn test_call_sq_object_function() -> Result<(),String> {
///     call_sq_function::<()>(sqvm, sq_functions, "someFunction").map_err(|err| err.to_string())?;
///
///     Ok(())
/// }
/// ```
pub fn call_sq_function<R: GetFromSquirrelVm>(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &'static SquirrelFunctions,
    function_name: impl AsRef<str>,
) -> Result<R, CallError> {
    let mut obj = std::mem::MaybeUninit::<SQObject>::zeroed();
    let ptr = obj.as_mut_ptr();

    let function_name = try_cstring(function_name.as_ref())?;

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
/// # Crashes
///
/// If the return value is incorrect the current api may segfault
///
/// # Example
///
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::high::squirrel::call_sq_object_function;
/// # use rrplug::{high::squirrel::SQHandle,bindings::squirreldatatypes::SQClosure};
/// #[rrplug::sqfunction(VM="Server")]
/// fn call_sqvm_function(mut func: SQHandle<SQClosure>) -> Result<(),String>{
///     call_sq_object_function::<()>(sqvm, sq_functions, func).map_err(|err| err.to_string())?;
///
///     Ok(())
/// }
/// ```
pub fn call_sq_object_function<R: GetFromSquirrelVm>(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &'static SquirrelFunctions,
    mut obj: SQHandle<SQClosure>,
) -> Result<R, CallError> {
    _call_sq_object_function(sqvm, sqfunctions, obj.as_callable())
}

#[inline]
fn _call_sq_object_function<R: GetFromSquirrelVm>(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &'static SquirrelFunctions,
    ptr: *mut SQObject,
) -> Result<R, CallError> {
    unsafe {
        let sqvm = &mut *sqvm;
        (sqfunctions.sq_pushobject)(sqvm, ptr);
        (sqfunctions.sq_pushroottable)(sqvm);

        if (sqfunctions.sq_call)(sqvm, 1, true as u32, true as u32) == SQRESULT::SQRESULT_ERROR {
            Err(CallError::FunctionFailedToExecute)
        } else {
            Ok(R::get_from_sqvm(sqvm, sqfunctions, sqvm._top))
        }
    }
}

/// compiles a string and runs it on the provided sqvm
///
/// this may receive a return value in the feature idk
///
/// ## Example
///
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::high::squirrel::compile_string;
/// #[rrplug::sqfunction(VM="Server")]
/// fn compile_string_test() -> Result<(),String> {
///     compile_string(sqvm, sq_functions, true, "print(\"helloworld\")").map_err(|err| err.to_string())?;
///
///     Ok(())
/// }
/// ```
pub fn compile_string(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctions,
    should_throw_error: bool,
    code: impl AsRef<str>,
) -> Result<(), SQCompileError> {
    const BUFFER_NAME: *const i8 = "compile_string\0".as_ptr() as *const i8;

    let buffer =
        try_cstring(code.as_ref()).unwrap_or_else(|_| to_cstring(&code.as_ref().replace('\0', "")));

    let mut compile_buffer = CompileBufferState {
        buffer: buffer.as_ptr(),
        bufferPlusLength: (buffer.as_ptr() as usize + buffer.as_bytes().len()) as *const i8,
        bufferAgain: buffer.as_ptr(),
    };

    unsafe {
        let result = (sqfunctions.sq_compilebuffer)(
            sqvm,
            &mut compile_buffer as *mut CompileBufferState,
            BUFFER_NAME,
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
