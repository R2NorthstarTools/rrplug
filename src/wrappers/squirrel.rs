#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! squirrel vm related function and statics

use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::mem::MaybeUninit;

use super::{
    errors::{CallError, SQCompileError},
    northstar::{FuncSQFuncInfo, ScriptVmType},
    vector::Vector3,
};
use crate::{
    bindings::{
        squirrelclasstypes::{CompileBufferState, SQFunction, SQRESULT_ERROR},
        squirreldatatypes::{CSquirrelVM, HSquirrelVM, SQObject},
        unwraped::SquirrelFunctionsUnwraped,
    },
    sq_return_null, to_sq_string,
};

#[doc(hidden)]
pub static FUNCTION_SQ_REGISTER: Mutex<Vec<FuncSQFuncInfo>> = Mutex::new(Vec::new());

/// functions that used to interact with the sqvm
///
/// client functions are both for ui and client vms
pub static SQFUNCTIONS: SqFunctions = SqFunctions {
    client: OnceCell::new(),
    server: OnceCell::new(),
};

type CSquirrelVMStatic = Mutex<OnceCell<CSquirrelVMHandle<Saved>>>;
pub static SV_CS_VM: CSquirrelVMStatic = Mutex::new(OnceCell::new());
pub static CL_CS_VM: CSquirrelVMStatic = Mutex::new(OnceCell::new());
pub static UI_CS_VM: CSquirrelVMStatic = Mutex::new(OnceCell::new());

/// functions that used to interact with the sqvm
///
/// client functions are both for ui and client vms
pub struct SqFunctions {
    pub client: OnceCell<SquirrelFunctionsUnwraped>,
    pub server: OnceCell<SquirrelFunctionsUnwraped>,
}

#[derive(Debug)] // super cringe

pub struct Save;
#[derive(Debug)] // super cringe

pub struct NoSave;
#[derive(Debug)] // super cringe

pub struct Saved;

#[derive(Debug)]
pub struct CSquirrelVMHandle<T> {
    handle: *mut CSquirrelVM,
    vm_type: ScriptVmType,
    marker: std::marker::PhantomData<T>,
}

impl CSquirrelVMHandle<Save> {
    pub fn new(handle: *mut CSquirrelVM, vm_type: ScriptVmType) -> Self {
        Self::_save(handle, vm_type);

        Self {
            handle,
            vm_type,
            marker: std::marker::PhantomData::<Save>,
        }
    }
}

impl CSquirrelVMHandle<NoSave> {
    pub fn new(handle: *mut CSquirrelVM, vm_type: ScriptVmType) -> Self {
        Self {
            handle,
            vm_type,
            marker: std::marker::PhantomData::<NoSave>,
        }
    }

    pub fn save(&self) {
        Self::_save(self.handle, self.vm_type);
    }

    // gets the [`CSquirrel`] from the handle
    pub fn get_cs_sqvm(&self) -> *mut CSquirrelVM {
        self.handle
    }
}

impl CSquirrelVMHandle<Saved> {
    // gets the [`CSquirrel`] from the handle
    pub fn get_cs_sqvm(&self) -> *mut CSquirrelVM {
        self.handle
    }

    // pub fn get
}

impl<T> CSquirrelVMHandle<T> {
    /// defines a constant on the sqvm
    ///
    /// Like `SERVER`, `CLIENT`, `UI`, etc
    pub fn define_sq_constant(&self, name: String, value: bool) {
        let sqfunctions = if self.vm_type == ScriptVmType::Server {
            SQFUNCTIONS.server.wait()
        } else {
            SQFUNCTIONS.client.wait()
        };

        // not sure if I need to leak this
        let name = to_sq_string!(name);

        unsafe { (sqfunctions.sq_defconst)(self.handle, name.as_ptr(), value as i32) }
    }

    /// gets the raw pointer to [`HSquirrelVM`]
    ///
    /// # Safety
    /// assumes its valid
    ///
    /// it is not valid after sqvm destruction
    pub unsafe fn get_sqvm(&self) -> *mut HSquirrelVM {
        (*self.handle).sqvm
    }

    /// gets the type of the sqvm :D
    pub fn get_context(&self) -> ScriptVmType {
        self.vm_type
    }

    pub(super) fn _save(handle: *mut CSquirrelVM, vm_type: ScriptVmType) {
        let save_handle = CSquirrelVMHandle {
            handle,
            vm_type,
            marker: std::marker::PhantomData::<Saved>,
        };

        let save_static = match vm_type {
            ScriptVmType::Server => &SV_CS_VM,
            ScriptVmType::Client => &CL_CS_VM,
            ScriptVmType::Ui => &UI_CS_VM,
            _ => &UI_CS_VM, // impossible to reach
        };

        let mut lock = save_static.lock();
        _ = lock.take();
        lock.set(save_handle)
            .expect("someone we failed to set a new CSquirrelVM"); // this is impossible because of the previous line;
    }
}

unsafe impl Sync for CSquirrelVMHandle<Saved> {}
unsafe impl Send for CSquirrelVMHandle<Saved> {}

/// "safely" calls any function defined on the sqvm
///
/// they would only run when the sqvm is valid
pub fn async_call_sq_function(
    context: ScriptVmType,
    function_name: impl Into<String>,
    pop_function: Option<SQFunction>,
) {
    let sqfunctions = match context {
        ScriptVmType::Server => SQFUNCTIONS.server.wait(),
        _ => SQFUNCTIONS.client.wait(),
    };

    let pop_function = match pop_function {
        Some(callback) => callback,
        None => __pop_function,
    };

    let function_name = to_sq_string!(function_name.into());

    unsafe {
        (sqfunctions.sq_schedule_call_external)(
            context.into(),
            function_name.as_ptr(),
            pop_function,
        )
    }
}

/// "safely" calls any function defined on the sqvm
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
///
/// this only allows calls without args use the marco [`crate::call_sq_function`] instead if you want args
pub fn call_sq_function(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    function_name: impl Into<String>,
) -> Result<(), CallError> {
    let mut obj = Box::new(std::mem::MaybeUninit::<SQObject>::zeroed());
    let ptr = obj.as_mut_ptr();

    let function_name = to_sq_string!(function_name.into());

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

/// "safely" calls any function defined on the sqvm from its [`SQObject`]
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
///
/// this only allows calls without args use the marco [`crate::call_sq_object_function`] instead if you want args
pub fn call_sq_object_function(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    mut obj: MaybeUninit<SQObject>,
) -> Result<(), CallError> {
    _call_sq_object_function(sqvm, sqfunctions, obj.as_mut_ptr())
}

#[inline] // let rust decide I just follow dry :)
fn _call_sq_object_function(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    ptr: *mut SQObject,
) -> Result<(), CallError> {
    unsafe {
        (sqfunctions.sq_pushobject)(sqvm, ptr);
        (sqfunctions.sq_pushroottable)(sqvm);

        if (sqfunctions.sq_call)(sqvm, 1, true as u32, true as u32) == -1 {
            Err(CallError::FunctionFailedToExecute)
        } else {
            Ok(())
        }
    }
}

/// compiles a string and runs it on the provided sqvm
pub fn compile_string(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    should_throw_error: bool,
    code: impl Into<String>,
) -> Result<(), SQCompileError> {
    let buffer = to_sq_string!(code.into());

    let mut compile_buffer = CompileBufferState {
        buffer: buffer.as_ptr(),
        bufferPlusLength: (buffer.as_ptr() as usize + buffer.as_bytes().len()) as *const i8,
        bufferAgain: buffer.as_ptr(),
    };

    let name = to_sq_string!("compile_string");

    unsafe {
        let result = (sqfunctions.sq_compilebuffer)(
            sqvm,
            &mut compile_buffer as *mut CompileBufferState,
            name.as_ptr(),
            -1,
            should_throw_error as u32,
        );

        if result != SQRESULT_ERROR {
            (sqfunctions.sq_pushroottable)(sqvm);

            if (sqfunctions.sq_call)(sqvm, 1, 0, 0) == SQRESULT_ERROR {
                Err(SQCompileError::BufferFailedToExecute)
            } else {
                Ok(())
            }
        } else {
            Err(SQCompileError::CompileError)
        }
    }
}

pub fn push_sq_array<T>(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    arguments: Vec<T>,
) where
    T: PushToSquirrelVm,
{
    unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

    for e in arguments.into_iter() {
        e.push_to_sqvm(sqvm, sqfunctions);
        unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
    }
}

pub fn push_sq_float(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    float: impl Into<f32>,
) {
    unsafe { (sqfunctions.sq_pushfloat)(sqvm, float.into()) };
}

pub fn push_sq_int(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    int: impl Into<i32>,
) {
    unsafe { (sqfunctions.sq_pushinteger)(sqvm, int.into()) };
}

pub fn push_sq_bool(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped, boolen: bool) {
    unsafe { (sqfunctions.sq_pushbool)(sqvm, boolen as u32) };
}

pub fn push_sq_string(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    string: impl Into<String>,
) {
    // boxing this would be a good idea and leaking; altough we don't need to?
    let cstring = to_sq_string!(string.into());
    // its impossble for it to crash since we replace null with space if it does it must be reported
    unsafe { (sqfunctions.sq_pushstring)(sqvm, cstring.as_ptr(), -1) }; // why -1?
}

pub fn push_sq_vector(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    vector: Vector3,
) {
    unsafe { (sqfunctions.sq_pushvector)(sqvm, (&vector).into()) };
}

pub fn push_sq_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    mut object: MaybeUninit<SQObject>,
) {
    unsafe { (sqfunctions.sq_pushobject)(sqvm, object.as_mut_ptr()) };
}

unsafe extern "C" fn __pop_function(_: *mut HSquirrelVM) -> i32 {
    sq_return_null!()
}

pub trait PushToSquirrelVm {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped);
}

impl PushToSquirrelVm for String {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_string(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for i32 {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_int(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for f32 {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_float(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for bool {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_bool(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for Vector3 {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_vector(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for MaybeUninit<SQObject> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_object(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = String>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = i32>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = f32>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = bool>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = Vector3>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = MaybeUninit<SQObject>>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}
