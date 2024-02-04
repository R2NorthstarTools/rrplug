use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::{
    mem::MaybeUninit,
    sync::mpsc::{self, Receiver, SendError, Sender},
};

use crate::{
    bindings::{
        squirrelclasstypes::{ScriptContext, SQRESULT},
        squirreldatatypes::{HSquirrelVM, SQObject},
        squirrelfunctions::SquirrelFunctions,
    },
    high::squirrel_traits::PushToSquirrelVm,
    mid::{
        squirrel::{SQFUNCTIONS, SQVM_CLIENT, SQVM_SERVER, SQVM_UI},
        utils::to_cstring,
    },
};

use super::engine::EngineToken;

static ENGINE_MESSAGE_SEND: OnceCell<Sender<AsyncEngineMessage>> = OnceCell::new();
static ENGINE_MESSAGE_RECV: OnceCell<Mutex<Receiver<AsyncEngineMessage>>> = OnceCell::new();

pub enum AsyncEngineMessage {
    ExecuteSquirrel {
        context: ScriptContext,
        function_name: String,
        args: Box<
            dyn FnOnce(*mut HSquirrelVM, &'static SquirrelFunctions) -> i32 + 'static + Send + Sync,
        >,
    },
    ExecuteFunction(Box<dyn FnOnce(EngineToken) + 'static + Send + Sync>),
}

impl AsyncEngineMessage {
    #[inline]
    pub fn run_squirrel_func(
        name: impl Into<String>,
        context: ScriptContext,
        args: impl IntoSquirrelArgs,
    ) -> Self {
        Self::ExecuteSquirrel {
            function_name: name.into(),
            context,
            args: args.into_function(),
        }
    }

    #[inline]
    pub fn run_func(func: impl FnOnce(EngineToken) + 'static + Send + Sync) -> Self {
        Self::ExecuteFunction(Box::new(func))
    }
}

/// calls any function defined on the sqvm
///
/// they would only run when the sqvm is valid
pub fn async_execute(message: AsyncEngineMessage) -> Result<(), SendError<AsyncEngineMessage>> {
    ENGINE_MESSAGE_SEND.wait().send(message)
}

#[doc(hidden)]
pub fn init_async_routine() {
    let (send, recv) = mpsc::channel();

    ENGINE_MESSAGE_SEND
        .set(send)
        .expect("should only call init_async_routine once");
    ENGINE_MESSAGE_RECV
        .set(recv.into())
        .expect("should only call init_async_routine once");
}

/// this function runs the async routine,
/// therefore it must be called on engine thread (in runframe) otherwise ub will happen
#[doc(hidden)]
pub unsafe fn run_async_routine() {
    let token = unsafe { EngineToken::new_unchecked() };

    if let Ok(to_run) = ENGINE_MESSAGE_RECV.wait().lock().try_recv() {
        match to_run {
            AsyncEngineMessage::ExecuteSquirrel {
                context,
                function_name,
                args,
            } => {
                // TODO: when done with sqvm global add it here
                let (sqvm, sqfunctions) = match context {
                    ScriptContext::SERVER => {
                        (SQVM_SERVER.get(token).borrow(), SQFUNCTIONS.server.wait())
                    }
                    ScriptContext::CLIENT => {
                        (SQVM_CLIENT.get(token).borrow(), SQFUNCTIONS.client.wait())
                    }
                    ScriptContext::UI => (SQVM_UI.get(token).borrow(), SQFUNCTIONS.client.wait()),
                };

                let Some(sqvm) = sqvm.map(|s| s) else {
                    log::warn!("a async sq function was called while the sqvm was destroyed!");
                    return;
                };

                let mut function_obj = MaybeUninit::<SQObject>::zeroed();

                let result = unsafe {
                    (sqfunctions.sq_getfunction)(
                        sqvm,
                        to_cstring(&function_name).as_ptr(), // TODO: safe or not?
                        function_obj.as_mut_ptr(),
                        std::ptr::null(),
                    )
                };

                if result != 0 {
                    log::warn!("async squirrel function failed to executel; it may not be global");
                } else {
                    unsafe {
                        let amount = args(sqvm, sqfunctions);

                        (sqfunctions.sq_pushobject)(sqvm, function_obj.as_mut_ptr());
                        (sqfunctions.sq_pushroottable)(sqvm);

                        if (sqfunctions.sq_call)(sqvm, amount, true as u32, true as u32)
                            == SQRESULT::SQRESULT_ERROR
                        {
                            log::warn!("async squirrel function failed to execute!")
                        }
                    }
                }
            }
            AsyncEngineMessage::ExecuteFunction(func) => func(token),
        }
    }
}

pub trait IntoSquirrelArgs {
    fn into_function(
        self,
    ) -> Box<dyn FnOnce(*mut HSquirrelVM, &'static SquirrelFunctions) -> i32 + 'static + Send + Sync>;
}

// TODO: format this
// TODO: check for correctness
macro_rules! into_squirrel_args_impl{
    ( $( ($($ty_name: ident : $tuple_index:tt),*) );*; ) => { $(
        impl<$($ty_name: PushToSquirrelVm + 'static + Send + Sync,)*> IntoSquirrelArgs for ($($ty_name,)*) {
            fn into_function(
                self
    ) -> Box<dyn FnOnce(*mut HSquirrelVM, &'static SquirrelFunctions) -> i32 + 'static + Send + Sync> {
                Box::new(move |sqvm: *mut HSquirrelVM, sqfunctions: &'static SquirrelFunctions| { _ =
                    $(
                        self.$tuple_index.push_to_sqvm(sqvm, sqfunctions);
                    )*

                    $crate::macros::sq_utils::__arg_count_helper([$($crate::__replace_expr!($ty_name)),*]) as i32
                })
            }
        }
    )* }
}

// TODO: add single parameter

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

#[test]
fn test_async_engine() {}
