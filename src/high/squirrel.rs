#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! squirrel vm related function and statics

use parking_lot::Mutex;
use std::{
    any::TypeId,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
    ops::{Add, Deref, DerefMut},
    ptr::{self, NonNull},
};

use super::{
    squirrel_traits::{
        GetFromSQObject, GetFromSquirrelVm, IntoSquirrelArgs, IsSQObject, PushToSquirrelVm,
        SQVMName,
    },
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
    handle: NonNull<CSquirrelVM>,
    vm_type: ScriptContext,
}

impl CSquirrelVMHandle {
    /// **should** not be used outside of the [`crate::entry`] macro
    #[doc(hidden)]
    pub fn new(
        mut handle: NonNull<CSquirrelVM>,
        context: ScriptContext,
        is_being_dropped: bool,
        token: EngineToken,
    ) -> Self {
        unsafe {
            match (context, is_being_dropped) {
                (ScriptContext::SERVER, false) => {
                    _ = SQVM_SERVER.get(token).replace(Some(
                        NonNull::new(handle.as_mut().sqvm).expect("sqvm cannot be null"),
                    ))
                }
                (ScriptContext::SERVER, true) => _ = SQVM_SERVER.get(token).replace(None),
                (ScriptContext::CLIENT, false) => {
                    _ = SQVM_CLIENT.get(token).replace(Some(
                        NonNull::new(handle.as_mut().sqvm).expect("sqvm cannot be null"),
                    ))
                }
                (ScriptContext::CLIENT, true) => _ = SQVM_CLIENT.get(token).replace(None),
                (ScriptContext::UI, false) => {
                    _ = SQVM_UI.get(token).replace(Some(
                        NonNull::new(handle.as_mut().sqvm).expect("sqvm cannot be null"),
                    ))
                }
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

        unsafe { (sqfunctions.sq_defconst)(self.handle.as_ptr(), name.as_ptr(), value.into()) }
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
    pub const unsafe fn get_sqvm(&self) -> UnsafeHandle<NonNull<HSquirrelVM>> {
        unsafe { UnsafeHandle::internal_new(NonNull::new_unchecked(self.handle.as_ref().sqvm)) }
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
    pub const unsafe fn get_cs_sqvm(&self) -> UnsafeHandle<NonNull<CSquirrelVM>> {
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
    /// creates a new [`SquirrelFn`] using the invariance of [`SQHandle<SQClosure>`]
    ///
    /// # Safety
    ///
    /// doesn't check if the function passed has the correct args and return type
    pub const unsafe fn new_unchecked(obj: SQHandle<SQClosure>) -> Self {
        Self {
            func: obj,
            phantom: PhantomData,
        }
    }

    /// calls the underlying squirrel function on the provided sqvm
    ///
    /// # Errors
    ///
    /// This function will return an error if the fails to execute for some reason which is unlikely since it would be type checked
    pub fn run(
        &mut self,
        sqvm: NonNull<HSquirrelVM>,
        sqfunctions: &'static SquirrelFunctions,
        args: T,
    ) -> Result<(), CallError> {
        unsafe {
            let amount = args.into_push(sqvm, sqfunctions);

            (sqfunctions.sq_pushobject)(sqvm.as_ptr(), self.func.as_callable());
            (sqfunctions.sq_pushroottable)(sqvm.as_ptr());

            if (sqfunctions.sq_call)(sqvm.as_ptr(), amount, true as u32, true as u32)
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
        sqvm: NonNull<HSquirrelVM>,
        sqfunctions: &'static SquirrelFunctions,
        args: T,
    ) -> Result<(), CallError> {
        self.run(sqvm, sqfunctions, args)
    }
}

impl<T: IntoSquirrelArgs> AsRef<SQHandle<SQClosure>> for SquirrelFn<T> {
    fn as_ref(&self) -> &SQHandle<SQClosure> {
        &self.func
    }
}

/// [`UserData`] is used to push user provided data to the sqvm for storage
///
/// [`UserDataRef`] can be used to access the data from functions calls
///
/// [`UserData`] handles dropping the data via a release hook in sqvm
pub struct UserData<T>(Box<T>);

impl<T: 'static> UserData<T> {
    /// Creates a new [`UserData<T>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use rrplug::prelude::*;
    /// # use rrplug::high::squirrel::UserData;
    /// // cannot be pushed to sqvm normally
    /// struct HttpClient;
    ///
    /// #[rrplug::sqfunction(VM = "SERVER | CLIENT | UI")]
    /// fn build_client() -> UserData<HttpClient> {
    ///     UserData::new(HttpClient)
    /// }
    /// ```
    pub fn new(userdata: T) -> Self {
        Self(userdata.into())
    }

    /// Creates a new [`UserData<T>`] from boxed data
    pub fn from_boxed(userdata: Box<T>) -> Self {
        Self(userdata)
    }
}

impl<T: 'static> From<T> for UserData<T> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T: 'static> From<Box<T>> for UserData<T> {
    fn from(value: Box<T>) -> Self {
        Self(value)
    }
}

impl<T> SQVMName for UserData<T> {
    fn get_sqvm_name() -> String {
        "userdata".to_string()
    }
}

impl<T: 'static> PushToSquirrelVm for UserData<T> {
    fn push_to_sqvm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) {
        unsafe {
            (sqfunctions.sq_createuserdata)(sqvm.as_ptr(), std::mem::size_of::<*mut T>() as i32)
                .cast::<*mut T>()
                .write(Box::leak(self.0));

            let mut hasher = DefaultHasher::new();
            TypeId::of::<T>().hash(&mut hasher);

            (sqfunctions.sq_setuserdatatypeid)(sqvm.as_ptr(), -1, hasher.finish());
            (sqfunctions.sq_setreleasehook)(sqvm.as_ptr(), -1, releasehook::<T>);
        };

        extern "C" fn releasehook<T>(userdata: *const std::ffi::c_void, _: i32) {
            unsafe {
                let _ = Box::from_raw(*userdata.cast::<*mut T>());
            };
        }
    }
}

/// Used to refrence [`UserData`] stored on the sqvm
///
/// the data cannot be moved out of since it's referenec counted in the sqvm
///
/// # SAFETY
///
/// [`UserDataRef`] **MUST** remain within the same stackframe (aka get dropped before the squirrel function returns it was created in)
/// otherwise bad things will happen since the underlying [`UserData`] might get dropped
///
/// This should be inforced by `!Move` and `!Sync` constraints so this is just a warning
///
/// # Example
///
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::high::squirrel::UserDataRef;
/// struct HiddenMessage(String); // msg cannot be accessed from the sqvm
///
/// #[rrplug::sqfunction(VM = "SERVER")]
/// fn get_secret_msg(msg: UserDataRef<HiddenMessage>) -> String {
///     msg.0.chars().map(|_| '*').collect()
/// }
/// ```
pub struct UserDataRef<'a, T>(&'a mut T, PhantomData<*mut ()>);

impl<'a, T: 'static> GetFromSquirrelVm for UserDataRef<'a, T> {
    fn get_from_sqvm(
        sqvm: NonNull<HSquirrelVM>,
        sqfunctions: &'static SquirrelFunctions,
        stack_pos: i32,
    ) -> Self {
        let mut out_ptr = ptr::null_mut();

        let mut hasher = DefaultHasher::new();
        TypeId::of::<T>().hash(&mut hasher);
        let type_id = hasher.finish();

        let mut out_type_id = type_id;

        unsafe {
            debug_assert!(
                (sqfunctions.sq_getuserdata)(
                    sqvm.as_ptr(),
                    stack_pos,
                    &mut out_ptr,
                    &mut out_type_id
                ) != SQRESULT::SQRESULT_ERROR
            )
        }

        debug_assert_eq!(type_id, out_type_id, "script provided incorrect userdata");

        UserDataRef(unsafe { &mut **out_ptr.cast::<*mut T>() }, PhantomData)
    }

    fn get_from_sqvm_internal(
        sqvm: NonNull<HSquirrelVM>,
        sqfunctions: &'static SquirrelFunctions,
        stack_pos: &mut i32,
    ) -> Self {
        let userdata = Self::get_from_sqvm(sqvm, sqfunctions, stack_pos.add(1));
        *stack_pos += 2; // for some reason userdata also has a table pushed with it
        userdata
    }
}

impl<'a, T> SQVMName for UserDataRef<'a, T> {
    fn get_sqvm_name() -> String {
        "userdata".to_string()
    }
}

impl<'a, T> Deref for UserDataRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T> DerefMut for UserDataRef<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

/// suspends a thread when returned from a native sqfunction
pub struct SuspendThread<T: PushToSquirrelVm + SQVMName> {
    phantom: PhantomData<T>,
}

impl<T: PushToSquirrelVm + SQVMName> SQVMName for SuspendThread<T> {
    fn get_sqvm_name() -> String {
        T::get_sqvm_name()
    }
}

impl<T: PushToSquirrelVm + SQVMName> PushToSquirrelVm for SuspendThread<T> {
    fn push_to_sqvm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) {
        unsafe { (sqfunctions.sq_suspendthread)(sqvm.as_ptr(), &sqvm.as_ptr(), 5, sqvm.as_ptr()) };
    }
}

impl<T: PushToSquirrelVm + SQVMName> SuspendThread<T> {
    const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    fn is_thread_and_throw_error(
        thread_sqvm: NonNull<HSquirrelVM>,
        sqfunctions: &SquirrelFunctions,
    ) -> bool {
        use super::squirrel_traits::ReturnToVm;
        let mut is_thread = true;

        // idk if this is how to check it
        if 2 < unsafe { thread_sqvm.as_ref()._suspended } {
            Err::<i32, _>("Cannot suspend thread from within code function calls".to_string())
                .return_to_vm(thread_sqvm, sqfunctions);
            is_thread = false
        }

        is_thread
    }

    /// Spawns a native thread.
    /// When completed it resumes the thread
    ///
    /// # SAFETY
    /// this thread cannot live long the parent sqvm
    #[cfg(feature = "async_engine")]
    pub fn new_with_thread<F>(thread_sqvm: NonNull<HSquirrelVM>, mut thread_func: F) -> Self
    where
        F: FnMut() -> T + Send + 'static,
        T: Send + Sync + 'static,
    {
        use crate::high::engine_sync::{async_execute, AsyncEngineMessage};

        if !Self::is_thread_and_throw_error(thread_sqvm, SQFUNCTIONS.from_sqvm(thread_sqvm)) {
            return Self::new();
        }

        let thread_sqvm = unsafe { UnsafeHandle::new(thread_sqvm) };
        std::thread::spawn(move || {
            let result = thread_func();

            // TODO: check if the sqvm has expired
            async_execute(AsyncEngineMessage::run_func(move |_| {
                let thread_sqvm = thread_sqvm.take();
                let sq_functions = SQFUNCTIONS.from_sqvm(thread_sqvm);

                result.push_to_sqvm(thread_sqvm, sq_functions);
                unsafe { resume_thread(thread_sqvm, sq_functions) };
            }))
        });

        Self::new()
    }

    /// calls a function to store the ref to wake up this thread sqvm
    ///
    /// the stored [`ThreadWakeUp`] cannot outlive the parent sqvm
    pub fn new_with_store<F>(thread_sqvm: NonNull<HSquirrelVM>, mut store_func: F) -> Self
    where
        F: FnMut(ThreadWakeUp<T>),
    {
        if !Self::is_thread_and_throw_error(thread_sqvm, SQFUNCTIONS.from_sqvm(thread_sqvm)) {
            return Self::new();
        }

        store_func(ThreadWakeUp {
            thread_sqvm,
            phantom: PhantomData::<T>,
        });

        Self::new()
    }

    /// returns a [`ThreadWakeUp`] for this thread
    ///
    /// # Failure
    ///
    /// fails to return [`ThreadWakeUp`] if the sqvm is not a thread
    pub fn new_both(thread_sqvm: NonNull<HSquirrelVM>) -> (Self, Option<ThreadWakeUp<T>>) {
        if !Self::is_thread_and_throw_error(thread_sqvm, SQFUNCTIONS.from_sqvm(thread_sqvm)) {
            return (Self::new(), None);
        }

        (
            Self::new(),
            Some(ThreadWakeUp {
                thread_sqvm,
                phantom: PhantomData::<T>,
            }),
        )
    }
}

/// stores the thread sqvm to wake up and the return type in the generic
pub struct ThreadWakeUp<T: PushToSquirrelVm> {
    thread_sqvm: NonNull<HSquirrelVM>,
    phantom: PhantomData<T>,
}

impl<T: PushToSquirrelVm> ThreadWakeUp<T> {
    /// resumes the sqvm thread
    ///
    /// ub if it outlived the parent sqvm
    pub fn resume(self, data: T) {
        let sq_functions = SQFUNCTIONS.from_sqvm(self.thread_sqvm);
        data.push_to_sqvm(self.thread_sqvm, sq_functions);
        unsafe { resume_thread(self.thread_sqvm, sq_functions) };
    }
}

/// # SAFETY
/// has to be valid and cannot live long the parent sqvm
unsafe fn resume_thread(thread_sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) {
    unsafe {
        _ = (sqfunctions.sq_threadwakeup)(
            thread_sqvm.as_ptr(),
            5,
            std::ptr::null(),
            thread_sqvm.as_ptr(),
        )
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
///     call_sq_function::<(), _>(sqvm, sq_functions, "someFunction", ()).map_err(|err| err.to_string())?;
///
///     Ok(())
/// }
/// ```
pub fn call_sq_function<R: GetFromSQObject, A: IntoSquirrelArgs>(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &'static SquirrelFunctions,
    function_name: impl AsRef<str>,
    args: A,
) -> Result<R, CallError> {
    let mut obj = std::mem::MaybeUninit::<SQObject>::zeroed();
    let ptr = obj.as_mut_ptr();

    let function_name = try_cstring(function_name.as_ref())?;

    let result = unsafe {
        (sqfunctions.sq_getfunction)(sqvm.as_ptr(), function_name.as_ptr(), ptr, std::ptr::null())
    };

    if result != 0 {
        Err(CallError::FunctionNotFound(
            function_name.to_string_lossy().into(),
        ))
    } else {
        _call_sq_object_function(sqvm, sqfunctions, ptr, args)
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
///     call_sq_object_function::<(), _>(sqvm, sq_functions, func, ()).map_err(|err| err.to_string())?;
///
///     Ok(())
/// }
/// ```
pub fn call_sq_object_function<R: GetFromSQObject, A: IntoSquirrelArgs>(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &'static SquirrelFunctions,
    mut obj: SQHandle<SQClosure>,
    args: A,
) -> Result<R, CallError> {
    _call_sq_object_function(sqvm, sqfunctions, obj.as_callable(), args)
}

#[inline]
fn _call_sq_object_function<R: GetFromSQObject, A: IntoSquirrelArgs>(
    mut sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &'static SquirrelFunctions,
    ptr: *mut SQObject,
    args: A,
) -> Result<R, CallError> {
    unsafe {
        let sqvm_ref = sqvm.as_mut();
        (sqfunctions.sq_pushobject)(sqvm_ref, ptr);
        (sqfunctions.sq_pushroottable)(sqvm_ref);

        let amount = args.into_push(sqvm, sqfunctions);

        if (sqfunctions.sq_call)(sqvm_ref, amount + 1, true as u32, true as u32)
            == SQRESULT::SQRESULT_ERROR
        {
            Err(CallError::FunctionFailedToExecute)
        } else {
            Ok(R::get_from_sqobject(
                sqvm_ref
                    ._stack
                    .add(sqvm_ref._top as usize - 1)
                    .as_ref()
                    .ok_or(CallError::FunctionFailedToExecute)?,
            ))
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
    sqvm: NonNull<HSquirrelVM>,
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
            sqvm.as_ptr(),
            &mut compile_buffer as *mut CompileBufferState,
            BUFFER_NAME,
            -1,
            should_throw_error as u32,
        );

        if result != SQRESULT::SQRESULT_ERROR {
            (sqfunctions.sq_pushroottable)(sqvm.as_ptr());

            if (sqfunctions.sq_call)(sqvm.as_ptr(), 1, 0, 0) == SQRESULT::SQRESULT_ERROR {
                Err(SQCompileError::BufferFailedToExecute)
            } else {
                Ok(())
            }
        } else {
            Err(SQCompileError::CompileError)
        }
    }
}
