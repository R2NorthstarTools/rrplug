//! trait definitions and implementations to generalize interacting with squirrel

#![allow(clippy::not_unsafe_ptr_arg_deref)] // maybe remove this later

pub use rrplug_proc::{GetFromSQObject, GetFromSquirrelVm, PushToSquirrelVm, SQVMName};
use std::{mem::MaybeUninit, ptr::NonNull};

use crate::{
    bindings::{
        class_types::cplayer::CPlayer,
        squirrelclasstypes::SQRESULT,
        squirreldatatypes::{
            SQArray, SQBool, SQClosure, SQFloat, SQFunctionProto, SQInteger, SQNativeClosure,
            SQObject, SQObjectType, SQObjectValue, SQString, SQStructInstance, SQTable,
        },
    },
    high::squirrel::SQHandle,
    mid::{
        server::cplayer::CPLAYER_VTABLE,
        squirrel::{
            get_sq_array, get_sq_bool, get_sq_float, get_sq_int, get_sq_object, get_sq_string,
            get_sq_vector, push_sq_array, push_sq_bool, push_sq_float, push_sq_int, push_sq_object,
            push_sq_string, push_sq_vector, sqvm_to_context,
        },
        utils::to_cstring,
    },
    prelude::*,
};

use super::UnsafeHandle;

// Push Trait

macro_rules! push_to_sqvm {
    ( $( $function:ident::<$t:ty> );*; ) => { $(

        impl PushToSquirrelVm for $t {
            #[inline]
            fn push_to_sqvm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) {
                $function(sqvm, sqfunctions, self)
            }
        }
    )* }
}

/// trait to used to generalize pushing to the sq stack
///
/// # Use cases
/// - returning from native functions
/// - accumulating in arrays and structs
pub trait PushToSquirrelVm {
    /// used for ()
    #[doc(hidden)]
    const DEFAULT_RESULT: SQRESULT = SQRESULT::SQRESULT_NOTNULL;

    /// pushes the value to the stack
    fn push_to_sqvm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions);
}

push_to_sqvm! {
    push_sq_string::<String>;
    push_sq_string::<&str>;
    push_sq_int::<i32>;
    push_sq_float::<f32>;
    push_sq_bool::<bool>;
    push_sq_vector::<Vector3>;
    push_sq_object::<SQObject>;
}

impl<T> PushToSquirrelVm for Vec<T>
where
    T: PushToSquirrelVm,
{
    fn push_to_sqvm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) {
        push_sq_array(sqvm, sqfunctions, self);
    }
}

impl PushToSquirrelVm for () {
    const DEFAULT_RESULT: SQRESULT = SQRESULT::SQRESULT_NULL;

    #[inline]
    fn push_to_sqvm(self, _: NonNull<HSquirrelVM>, _: &SquirrelFunctions) {}
}

impl PushToSquirrelVm for &CPlayer {
    /// SAFETY: the object is stored inside the entity and the entity is not being modified  
    fn push_to_sqvm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) {
        unsafe {
            let obj =
                (sqfunctions.sq_create_script_instance)((self as *const CPlayer).cast_mut().cast());
            (sqfunctions.sq_pushobject)(sqvm.as_ptr(), obj);
        }
    }
}

impl<T: PushToSquirrelVm, const N: usize> PushToSquirrelVm for [T; N] {
    fn push_to_sqvm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) {
        push_sq_array(sqvm, sqfunctions, self);
    }
}

impl<T: PushToSquirrelVm> PushToSquirrelVm for UnsafeHandle<T>
where
    T: PushToSquirrelVm,
{
    fn push_to_sqvm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) {
        self.take().push_to_sqvm(sqvm, sqfunctions)
    }
}

// Return Trait

/// trait to return diffrent values to the sqvm from a native closure
///
/// [`Option`] will return a `ornull` type in squirrel
///
/// [`Result`] will return its T type in squirrel and will raise an exception in squirrel if it's an error
/// # Use cases
/// - returning from native functions
pub trait ReturnToVm {
    /// returns a value defined by [`SQRESULT`]
    fn return_to_vm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) -> SQRESULT;
}

impl<T: PushToSquirrelVm> ReturnToVm for Option<T> {
    /// returns a `ornull T` to the sqvm
    fn return_to_vm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) -> SQRESULT {
        match self {
            Some(rtrn) => {
                rtrn.push_to_sqvm(sqvm, sqfunctions);
                T::DEFAULT_RESULT
            }
            None => {
                unsafe { (sqfunctions.sq_pushnull)(sqvm.as_ptr()) };
                SQRESULT::SQRESULT_NULL
            }
        }
    }
}

impl<T: PushToSquirrelVm, E: ToString> ReturnToVm for Result<T, E> {
    /// will raise a squirrel exception if it's an error
    ///
    /// result returns of T,R are identical to non result returns of T
    fn return_to_vm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) -> SQRESULT {
        match self {
            Ok(rtrn) => {
                rtrn.push_to_sqvm(sqvm, sqfunctions);
                T::DEFAULT_RESULT
            }
            Err(err) => {
                let err = to_cstring(err.to_string().as_str());
                unsafe { (sqfunctions.sq_raiseerror)(sqvm.as_ptr(), err.as_ptr()) };
                SQRESULT::SQRESULT_ERROR
            }
        }
    }
}

impl<T: PushToSquirrelVm> ReturnToVm for T {
    /// any return for types simply pushes it and returns NonNull
    fn return_to_vm(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions) -> SQRESULT {
        self.push_to_sqvm(sqvm, sqfunctions);
        T::DEFAULT_RESULT
    }
}

// IntoSquirrelArgs Trait

/// closure that simplies pushing groups of items to the squirrel vm; asynchronously or immediately
pub trait IntoSquirrelArgs {
    /// converts a implemenator of this trait into a closure that pushes it to the squirrel stack when ran
    fn into_function(
        self,
    ) -> Box<
        dyn FnOnce(NonNull<HSquirrelVM>, &'static SquirrelFunctions) -> i32 + 'static + Send + Sync,
    >
    where
        Self: Sized + Send + Sync + 'static,
    {
        Box::new(
            move |sqvm: NonNull<HSquirrelVM>, sqfunctions: &'static SquirrelFunctions| {
                self.into_push(sqvm, sqfunctions)
            },
        )
    }

    /// pushes the args to the sqvm and returns the amount pushed
    fn into_push(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &'static SquirrelFunctions) -> i32;
}

impl<T: PushToSquirrelVm> IntoSquirrelArgs for T {
    fn into_push(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &'static SquirrelFunctions) -> i32 {
        // hack :(
        // no specialization
        if T::DEFAULT_RESULT != SQRESULT::SQRESULT_NULL {
            self.push_to_sqvm(sqvm, sqfunctions);
            1
        } else {
            0
        }
    }
}

// TODO: check for correctness
macro_rules! into_squirrel_args_impl{
    ( $( ($($ty_name: ident : $tuple_index:tt),*) );*; ) => { $(
        impl<$($ty_name: PushToSquirrelVm,)*> IntoSquirrelArgs for ($($ty_name,)*) {
            fn into_push(self, sqvm: NonNull<HSquirrelVM>, sqfunctions: &'static SquirrelFunctions) -> i32 {
                $(
                    self.$tuple_index.push_to_sqvm(sqvm, sqfunctions);
                )*
                $crate::macros::sq_utils::__arg_count_helper([$($crate::__replace_expr!($ty_name)),*]) as i32
            }
        }
    )* }
}

into_squirrel_args_impl! {
    (T1: 0);
    (T1: 0, T2: 1);
    (T1: 0, T2: 1, T3: 2);
    (T1: 0, T2: 1, T3: 2, T4: 3);
    (T1: 0, T2: 1, T3: 2, T4: 3, T5: 4);
    (T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5);
    (T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6);
    (T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7);
    (T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8);
    (T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9);
}

// Get Trait

macro_rules! get_from_sqvm {
    ( $( $function:ident::<$t:ty> );*; ) => { $(

        impl GetFromSquirrelVm for $t {
            #[inline]
            fn get_from_sqvm(
                sqvm: NonNull<HSquirrelVM>,
                sqfunctions: &SquirrelFunctions,
                stack_pos: i32,
            ) -> Self {
                $function(sqvm, sqfunctions, stack_pos)
            }
        }
    )* };

    ( $( ($($ty_name: ident : $var_name:ident),*) );*; ) => { $(
        impl<$($ty_name: PushToSquirrelVm,)*> GetFromSquirrelVm for Box<dyn Fn($($ty_name,)*)> {
            fn get_from_sqvm(
                sqvm: NonNull<HSquirrelVM>,
                sqfunctions: &'static SquirrelFunctions,
                stack_pos: i32,
            ) -> Self {
                Box::new(move |$($var_name: $ty_name,)*| { _ =
                    call_sq_object_function!(
                        sqvm,
                        sqfunctions,
                        SQHandle::<SQClosure>::get_from_sqvm(sqvm, sqfunctions, stack_pos),
                        $($var_name),*
                    );
                })
            }
        }
    )* }
}

/// trait to get values out of the squrriel stack
///
/// # Use cases
/// - getting the arguments in native closures
pub trait GetFromSquirrelVm: Sized {
    /// tries to get the value out of the squirrel stack but it cannot fail
    /// so this can panic
    ///
    /// this is the user function do not overwrite the other one
    fn get_from_sqvm(
        sqvm: NonNull<HSquirrelVM>,
        sqfunctions: &'static SquirrelFunctions,
        stack_pos: i32,
    ) -> Self;

    /// this is only for certain internal apis
    ///
    /// don't use this only for internal apis
    #[doc(hidden)]
    #[inline]
    fn get_from_sqvm_internal(
        sqvm: NonNull<HSquirrelVM>,
        sqfunctions: &'static SquirrelFunctions,
        stack_pos: &mut i32,
    ) -> Self {
        let s = Self::get_from_sqvm(sqvm, sqfunctions, *stack_pos);

        // increament by the size this thing in the stack
        // for some reason userdata also has a table pushed with it; quite annoying
        *stack_pos += 1;

        s
    }
}

get_from_sqvm! {
    get_sq_string::<String>;
    get_sq_int::<i32>;
    get_sq_float::<f32>;
    get_sq_bool::<bool>;
    get_sq_vector::<Vector3>;
    get_sq_object::<SQObject>;
}

impl<T> GetFromSquirrelVm for Vec<T>
where
    T: GetFromSQObject,
{
    fn get_from_sqvm(
        sqvm: NonNull<HSquirrelVM>,
        _: &'static SquirrelFunctions,
        stack_pos: i32,
    ) -> Self {
        get_sq_array(sqvm, stack_pos)
    }
}

impl GetFromSquirrelVm for Option<&mut CPlayer> {
    fn get_from_sqvm(
        mut sqvm: NonNull<HSquirrelVM>,
        sqfunctions: &SquirrelFunctions,
        stack_pos: i32,
    ) -> Self {
        unsafe {
            debug_assert_eq!(
                sqvm_to_context(sqvm),
                ScriptContext::SERVER,
                "CPlayer only exists on server vm use C_Player for CLIENT and UI"
            );

            let sqvm = sqvm.as_mut();
            let cs_sqvm = sqvm
                .sharedState
                .as_ref()
                .expect("shared state was invalid")
                .cSquirrelVM;

            let mut obj = MaybeUninit::<SQObject>::uninit();
            (sqfunctions.sq_getobject)(sqvm, stack_pos, obj.as_mut_ptr());

            let ent = (sqfunctions.sq_getentityfrominstance)(
                cs_sqvm,
                obj.as_mut_ptr(),
                (sqfunctions.sq_get_entity_constant_cbase_entity)(),
            )
            .cast::<CPlayer>()
            .as_mut()?;

            if ent.vtable.copy_inner()
                != CPLAYER_VTABLE
                    .get()
                    .expect("CPlayer vtable is missing wtf?")
                    .vtable
                    .cast::<usize>()
            {
                return None;
            }

            Some(ent)
        }
    }
}

impl<'a, T: IsSQObject<'a>> GetFromSquirrelVm for SQHandle<'a, T> {
    fn get_from_sqvm(
        sqvm: NonNull<HSquirrelVM>,
        sqfunctions: &SquirrelFunctions,
        stack_pos: i32,
    ) -> Self {
        unsafe {
            let mut obj = std::mem::MaybeUninit::<SQObject>::uninit();
            (sqfunctions.sq_getobject)(sqvm.as_ptr(), stack_pos, obj.as_mut_ptr());

            match Self::try_new(obj.assume_init()) {
                Ok(handle) => handle,
                Err(_) => {
                    panic!(
                        "the object wasn't the correct type got {:X} expected {}",
                        obj.assume_init()._Type as i32,
                        std::any::type_name::<T>()
                    );
                }
            }
        }
    }
}

impl<'a, T: IntoSquirrelArgs> GetFromSquirrelVm for SquirrelFn<'a, T> {
    #[inline]
    fn get_from_sqvm(
        sqvm: NonNull<HSquirrelVM>,
        sqfunctions: &'static SquirrelFunctions,
        stack_pos: i32,
    ) -> Self {
        SquirrelFn {
            func: GetFromSquirrelVm::get_from_sqvm(sqvm, sqfunctions, stack_pos),
            phantom: std::marker::PhantomData,
        }
    }
}

impl GetFromSquirrelVm for () {
    /// exists for dynamic returns of some functions
    fn get_from_sqvm(_: NonNull<HSquirrelVM>, _: &SquirrelFunctions, _: i32) -> Self {}
}

// Get From SQObject Trait

/// gets the value out of a sqobject
///
/// most implementations don't check the type
///
/// so this can panic if it's not the correct type
///
/// # Use cases
/// - getting fields of arrays and structs
pub trait GetFromSQObject {
    /// gets the value out of a sqobject
    ///
    /// halts if the type is incorrect
    fn get_from_sqobject(obj: &SQObject) -> Self;
}

impl GetFromSQObject for () {
    #[inline]
    fn get_from_sqobject(_: &SQObject) -> Self {}
}

impl GetFromSQObject for String {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        unsafe {
            std::ffi::CStr::from_ptr(
                (&obj._VAL.asString.as_ref().unwrap_unchecked()._val) as *const i8,
            )
            .to_string_lossy()
            .into()
        }
    }
}

impl GetFromSQObject for i32 {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        unsafe { obj._VAL.asInteger }
    }
}

impl GetFromSQObject for f32 {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        unsafe { obj._VAL.asFloat }
    }
}

impl GetFromSQObject for bool {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        unsafe { obj._VAL.asInteger != 0 }
    }
}

impl GetFromSQObject for Vector3 {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        (obj as *const SQObject).into()
    }
}

impl GetFromSQObject for SQObject {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        *obj
    }
}

impl<'a, T: IsSQObject<'a>> GetFromSQObject for SQHandle<'a, T> {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        match Self::try_new(*obj) {
            Ok(handle) => handle,
            Err(_) => {
                panic!(
                    "the object wasn't the correct type got {:X} expected {}",
                    obj._Type as i32,
                    std::any::type_name::<T>()
                );
            }
        }
    }
}

impl<'a, T: IntoSquirrelArgs> GetFromSQObject for SquirrelFn<'a, T> {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        SquirrelFn {
            func: SQHandle::try_new(obj.to_owned())
                .expect("the squirrel object wasn't a function lol L"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> GetFromSQObject for Vec<T>
where
    T: GetFromSQObject,
{
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        unsafe {
            let array = obj
                ._VAL
                .asArray
                .as_ref()
                .expect("the sq object may be invalid");

            (0..array._usedSlots as usize)
                .map(|i| array._values.add(i))
                .filter_map(|obj| obj.as_ref())
                .map(T::get_from_sqobject)
                .collect()
        }
    }
}

// sqvm name

macro_rules! sqvm_name {
    ($( ($($ty_name:ident : $var_name:ident),*) );*;)  => {
        $(
            impl<$($ty_name: SQVMName,)*> SQVMName for ($($ty_name,)*) {
                fn get_sqvm_name() -> String {
                    let mut name = String::new();

                    $(
                        if !name.is_empty() { // bad solution but this will run only once for each use
                            name.push(',');
                            name.push(' ');
                        }
                        name.push_str(&$ty_name::get_sqvm_name());
                    )*

                    name
                }
            }
        )*
    };

    ( $( $t:ty = $sqty:literal );*; ) => {
        $(
            impl SQVMName for $t {
                #[inline]
                fn get_sqvm_name() -> String {
                     $sqty.to_string()
                }
            }
        )*
    };

    ( $( LIFE $t:ty = $sqty:literal );*; ) => {
        $(
            impl<'a> SQVMName for $t {
                #[inline]
                fn get_sqvm_name() -> String {
                     $sqty.to_string()
                }
            }
        )*
    };
}

/// the sqvm name of a type in rust
///
/// used to map a rust function into a sq native function
///
/// # Use cases
/// - translating rust types to squirrel types
pub trait SQVMName {
    /// the name on the sqvm of a type
    ///
    /// the default is "var" which is any type
    fn get_sqvm_name() -> String;
}

sqvm_name! {
    String = "string";
    &str = "string";
    i32 = "int";
    f32 = "float";
    bool = "bool";
    Vector3 = "vector";
    Option<&mut CPlayer> = "entity";
    SQObject = "var";
    () = "void";
}

sqvm_name! {
    LIFE SQHandle<'a, SQClosure> = "var";
    LIFE SQHandle<'a, SQTable> = "table";
    LIFE SQHandle<'a, SQString> = "string";
    LIFE SQHandle<'a, SQArray> = "array";
    LIFE SQHandle<'a, SQFloat> = "float";
    LIFE SQHandle<'a, SQInteger> = "int";
    LIFE SQHandle<'a, SQFunctionProto> = "var";
    LIFE SQHandle<'a, SQStructInstance> = "var";
    LIFE SQHandle<'a, SQBool> = "bool";
    LIFE SQHandle<'a, SQNativeClosure> = "var";
}

sqvm_name! {
    (T1: v2);
    (T1: v1, T2: v2);
    (T1: v1, T2: v2, T3: v3);
    (T1: v1, T2: v2, T3: v3, T4: v4);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7, T8: v8);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7, T8: v8, T9: v9);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7, T8: v8, T9: v9, T10: v10);
}

impl<'a, T: SQVMName + IntoSquirrelArgs> SQVMName for SquirrelFn<'a, T> {
    fn get_sqvm_name() -> String {
        format!("void functionref({})", T::get_sqvm_name())
    }
}

impl<T: SQVMName> SQVMName for Vec<T> {
    fn get_sqvm_name() -> String {
        format!("array<{}>", T::get_sqvm_name())
    }
}

// because of this `void ornull` is possible oops
impl<T: SQVMName> SQVMName for Option<T> {
    fn get_sqvm_name() -> String {
        format!("{} ornull", T::get_sqvm_name())
    }
}

impl<T: SQVMName, E> SQVMName for Result<T, E> {
    fn get_sqvm_name() -> String {
        T::get_sqvm_name() // yeah squirrel doesn't have a way in the type system to sepecify a possible error :|
    }
}

// specialization is not as strong as I though :(
// impl SQVMName for Option<()> {
//     fn get_sqvm_name() -> String {
//         "void".to_string()
//     }
// }

// Markers

macro_rules! is_sq_object {
    ( $( $object:ty,RT: $rt:expr,OT: $ot:expr, EXTRACT: * $extract:ident );*; ) => {
        $(
            impl<'a> IsSQObject<'a> for $object {
                const OT_TYPE: SQObjectType = $ot;
                const RT_TYPE: SQObjectType = $rt;

                fn extract_mut(val: &'a mut SQObjectValue) -> &'a mut Self {
                    unsafe { &mut *val.$extract } // asummed to be init
                }

                fn extract(val: &'a SQObjectValue) -> &'a Self {
                    unsafe { &*val.$extract } // asummed to be init
                }
            }
        )*
    };

    ( $( $object:ty,RT: $rt:expr,OT: $ot:expr, EXTRACT: $extract:ident );*; ) => {
        $(
            impl<'a> IsSQObject<'a> for $object {
                const OT_TYPE: SQObjectType = $ot;
                const RT_TYPE: SQObjectType = $rt;

                fn extract_mut(val: &'a mut SQObjectValue) -> &'a mut Self {
                    unsafe { std::mem::transmute(&mut val.$extract) } // asummed to be init
                }

                fn extract(val: &'a SQObjectValue) -> &'a Self {
                    unsafe { std::mem::transmute(&val.$extract) } // asummed to be init
                }
            }
        )*
    }
}

/// trait to define SQObject types
pub trait IsSQObject<'a> {
    /// ot type
    const OT_TYPE: SQObjectType;
    /// return type
    const RT_TYPE: SQObjectType;

    /// extracts the `Self` out of the SQObjectValue
    ///
    /// this is unsafe if [`SQHandle`] wasn't used
    fn extract(val: &'a SQObjectValue) -> &'a Self;

    /// extracts the `Self` out of the SQObjectValue
    ///
    /// this is unsafe if [`SQHandle`] wasn't used
    fn extract_mut(val: &'a mut SQObjectValue) -> &'a mut Self;
}

is_sq_object! {
    SQTable, RT: SQObjectType::RT_TABLE, OT: SQObjectType::OT_TABLE, EXTRACT: * asTable;
    SQString, RT: SQObjectType::RT_STRING, OT: SQObjectType::OT_STRING, EXTRACT: * asString;
    SQFunctionProto, RT: SQObjectType::RT_FUNCPROTO, OT: SQObjectType::OT_FUNCPROTO, EXTRACT: * asFuncProto;
    SQClosure, RT: SQObjectType::RT_CLOSURE, OT: SQObjectType::OT_CLOSURE, EXTRACT: * asClosure;
    SQStructInstance, RT: SQObjectType::RT_INSTANCE, OT: SQObjectType::OT_INSTANCE, EXTRACT: * asStructInstance;
    SQNativeClosure, RT: SQObjectType::RT_NATIVECLOSURE, OT: SQObjectType::OT_NATIVECLOSURE, EXTRACT: * asNativeClosure;
    SQArray, RT: SQObjectType::RT_ARRAY, OT: SQObjectType::OT_ARRAY, EXTRACT: * asArray;
}
is_sq_object! {
    SQFloat, RT: SQObjectType::RT_FLOAT, OT: SQObjectType::OT_FLOAT, EXTRACT: asFloat;
    SQInteger, RT: SQObjectType::RT_INTEGER, OT: SQObjectType::OT_INTEGER, EXTRACT: asInteger;
    SQBool, RT: SQObjectType::RT_BOOL, OT: SQObjectType::OT_BOOL, EXTRACT: asInteger;
} // not a thing? SQStructDef, RT: SQObjectType::, OT: SQObjectType::;

// TODO: so here is the idea
// have add_sqfunction be generic over extern "C" fn s and have traits to diffrenciate client/server/ui sqfunctions
// the generic would cover mutitple implementation
// but with this version the user would have to specifically ask for a sqvm and sqfunctions
// now that I writing this the biggest problem is the return ...
// but since it's a int we could have a C struct with a i32 and it would be transparent
// this would allow the user to return anything that can become that sturct
// so this is figured out :)
// also the input could be generic over *mut sqvm
// but then it would have to be a tuple :pain:
// maybe a combination of this and proc macro would be better?

// TODO: another thing to think about is the fact that there 5 traits for interacting with the sqvm
// they are all required for everything so why not just combine most of them into one large trait
