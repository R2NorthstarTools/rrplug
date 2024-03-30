//! contains stuff for sending jobs to the engine thread asynchronously
//!
//! requries the `engine_sync` feature

use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::{
    mem::MaybeUninit,
    ptr::NonNull,
    sync::mpsc::{self, Receiver, SendError, Sender},
};

use crate::{
    bindings::{
        squirrelclasstypes::{ScriptContext, SQRESULT},
        squirreldatatypes::{HSquirrelVM, SQObject},
        squirrelfunctions::SquirrelFunctions,
    },
    mid::{
        squirrel::{SQFUNCTIONS, SQVM_CLIENT, SQVM_SERVER, SQVM_UI},
        utils::to_cstring,
    },
};

use super::{engine::EngineToken, squirrel_traits::IntoSquirrelArgs};

static ENGINE_MESSAGE_SEND: OnceCell<Sender<AsyncEngineMessage>> = OnceCell::new();
static ENGINE_MESSAGE_RECV: OnceCell<Mutex<Receiver<AsyncEngineMessage>>> = OnceCell::new();

/// the content that is send to the engine thread to be ran on the next runframe pass.
pub enum AsyncEngineMessage {
    /// execute a global squirrel function
    ExecuteSquirrel {
        /// the context that it is ran on
        context: ScriptContext,
        /// the function's name
        function_name: String,
        /// the arguments that will passed to it via the closure (use `AsyncEngineMessage::run_squirrel_func`)
        args: Box<
            dyn FnOnce(NonNull<HSquirrelVM>, &'static SquirrelFunctions) -> i32
                + 'static
                + Send
                + Sync,
        >,
    },
    /// contains a closure that will be executed once on the next engine frame
    ExecuteFunction(Box<dyn FnOnce(EngineToken) + 'static + Send + Sync>),
}

impl AsyncEngineMessage {
    /// contructs a packet to run a global squirrel function and args to be pushed
    #[inline]
    pub fn run_squirrel_func(
        name: impl Into<String>,
        context: ScriptContext,
        args: impl IntoSquirrelArgs + 'static,
    ) -> Self {
        Self::ExecuteSquirrel {
            function_name: name.into(),
            context,
            args: args.into_function(),
        }
    }

    /// contstructs a execute packet from a closure
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
                        sqvm.as_ptr(),
                        to_cstring(&function_name).as_ptr(),
                        function_obj.as_mut_ptr(),
                        std::ptr::null(),
                    )
                };

                if result != 0 {
                    log::warn!("async squirrel function failed to execute; it may not be global");
                } else {
                    unsafe {
                        (sqfunctions.sq_pushobject)(sqvm.as_ptr(), function_obj.as_mut_ptr());
                        (sqfunctions.sq_pushroottable)(sqvm.as_ptr());

                        let amount = args(sqvm, sqfunctions);

                        if (sqfunctions.sq_call)(
                            sqvm.as_ptr(),
                            amount + 1,
                            true as u32,
                            true as u32,
                        ) == SQRESULT::SQRESULT_ERROR
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

// maybe the test should better than this
#[test]
fn test_async_engine() {
    init_async_routine();

    async_execute(AsyncEngineMessage::run_squirrel_func(
        "test",
        ScriptContext::SERVER,
        "test",
    ))
    .unwrap();
    async_execute(AsyncEngineMessage::run_func(|_| ())).unwrap();

    assert_eq!(
        &(0..10)
            .filter_map(|_| ENGINE_MESSAGE_RECV.wait().lock().try_recv().ok())
            .count(),
        &2
    );
}
