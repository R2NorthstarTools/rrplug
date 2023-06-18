#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! concommands are console commands. yes.
//!
//! registering a concommand
//! ```no_run
//! // inside Plugin impl
//! fn on_engine_load(&self, engine: EngineLoadType) {
//!     let engine = match engine {
//!         EngineLoadType::Engine(engine) => engine,
//!         _ => return;
//!     };
//!
//!     engine.register_concommand("boom", explode, "displays a explosion in the console", 0); // register the concommand
//! }
//! ```
//!
//! concommand use callback
//! ```no_run
//! #[rrplug::concommand]
//! fn explode(_command: CCommandResult) {
//!     log::info!("explode");
//!
//!     const BOOM: &str = r#"
//!           _ ._  _ , _ ._
//!           (_ ' ( `  )_  .__)
//!       ( (  (    )   `)  ) _)
//!       (__ (_   (_ . _) _) ,__)
//!           `~~`\ ' . /`~~`
//!               ;   ;
//!               /   \
//! _____________/_ __ \_____________
//!     "#;
//!
//!     for line in BOOM.split('\n') {
//!         log::info!("{line}")
//!     }
//! }
//! ```

use std::ffi::CStr;

use crate::bindings::command::CCommand;

/// [`CCommandResult`] gets all the usefull stuff from [`*const CCommand`] and puts in this safe struct
pub struct CCommandResult {
    ccommand: Option<CCommand>,
    args: Option<Vec<String>>,
    command: Option<String>,
}

impl CCommandResult {
    pub fn new(ccommand: *const CCommand) -> Self {
        match unsafe { ccommand.as_ref() } {
            Some(c) => Self {
                ccommand: Some(*c),
                args: None,
                command: None,
            },
            None => Self {
                ccommand: None,
                args: Some(Vec::new()),
                command: Some(String::new()),
            },
        }
    }

    fn parse(&mut self) {
        let ccommand = self.ccommand.unwrap();

        let (args, command) = unsafe {
            if ccommand.m_nArgv0Size == 0 {
                (Vec::new(), "".to_string())
            } else {
                let buffer = ccommand.m_pArgSBuffer.to_vec().as_ptr();
                let whole_command = CStr::from_ptr(buffer).to_string_lossy().to_string();
                let mut whole_command = whole_command.split_whitespace();

                let command = whole_command.next().unwrap_or_default().into();
                let args = whole_command.map(|a| a.to_string()).collect();

                (args, command)
            }
        };

        (self.args, self.command) = (Some(args), Some(command));
    }

    pub fn get_args(&mut self) -> &[String] {
        if self.args.is_none() {
            self.parse()
        }

        match &self.args {
            Some(args) => args,
            None => unreachable!(),
        }
    }

    pub fn get_command(&mut self) -> &str {
        if self.command.is_none() {
            self.parse()
        }

        match &self.command {
            Some(command) => command,
            None => unreachable!(),
        }
    }
}

// maybe this will work in the future
/*
impl RegisterConCommands {
    pub(crate) fn register_concommand<T: Fn(CCommandResult)>(
        &self,
        name: String,
        callback: T,
        help_string: String,
        flags: i32,
    ) -> Result<(), RegisterError> {
        let name_ptr = to_sq_string!(name).into_raw();

        let help_string_ptr = to_sq_string!(help_string).into_raw();

        let command: *mut ConCommand = unsafe {
            std::mem::transmute((CREATE_OBJECT_FUNC
                .wait()
                .ok_or(RegisterError::NoneFunction)?)(
                ObjectType::CONCOMMANDS
            ))
        };

        unsafe {
            self.reg_func.unwrap()(
                command,
                name_ptr,
                Some(ccommand_trampoline::<T>),
                help_string_ptr,
                flags,
                ptr::null_mut(),
            )
        };
        Ok(())
    }
}

extern "C" fn ccommand_trampoline<const T: fn(CCommandResult)>(ccommand: *const CCommand) {
    ccommand_trampoline(ccommand.into())
}
 */
