//! rrplug is a safe wrapper around the plugin system in [R2Northstar](https://northstar.tf/)
//!
//! rrplug uses compile time or sometimes runtime checks to guarantee safety in abstractions
//!
//! ## rrplug template
//!
//! install cargo-generate if you don't have it
//! ```bash
//! cargo install cargo-generate
//! ```
//!
//! ```bash
//! cargo generate -g  https://github.com/catornot/rrplug.git
//! ```
//!
//! a git [template](https://github.com/catornot/rrplug-template) also exists but it may or not be maintained as well

#![deny(
    unsafe_op_in_unsafe_fn,
    clippy::correctness,
    clippy::missing_const_for_fn
)]
#![warn(missing_docs)]
// #![allow(clippy::missing_safety_doc)] // toggle theese two for release/dev

#[allow(missing_docs)]
pub mod bindings;
pub mod errors;
pub mod high;
pub mod interfaces;
pub mod low;
pub mod macros;
pub mod mid;
#[doc(hidden)]
pub mod nslog;
pub mod plugin;
pub mod prelude;

pub use rrplug_proc::{as_interface, completion, concommand, convar, sqfunction};

#[doc(hidden)]
/// used by some macros
pub mod exports {
    pub use log;
    pub use once_cell::sync::OnceCell;
    pub use windows;
}

#[doc(hidden)]
/// used by [`rrplug::entry`]
pub mod rrplug {
    pub use super::*;
}

#[cfg(test)]
mod test {
    use crate as rrplug;
    use rrplug::prelude::*;
    use rrplug::{
        high::squirrel_traits::{GetFromSQObject, GetFromSquirrelVm, PushToSquirrelVm, SQVMName},
        mid::squirrel::{SQFuncInfo, SQFunctionContext},
    };
    use rrplug_proc::*;

    #[convar]
    fn test_convar(_old_string: String, _old_float: f32) -> () {}

    #[convar]
    fn test_convar_noargs() -> () {}

    /// test doc
    #[concommand]
    fn test_concommand(command: CCommandResult) {
        log::info!("test {:?}", command.get_args());
    }

    #[concommand]
    fn test_concommand_noargs() {}

    /// test doc
    #[sqfunction(VM = "Server", ExportName = "test")]
    fn test_sqfunction(
        test1: String,
        test2: i32,
        test3: TestEnum,
        mut test4: SquirrelFn<String>,
    ) -> Result<Vec<TestStruct>, String> {
        test4
            .call(sqvm, sq_functions, test1.clone())
            .map_err(|err| err.to_string())?;

        Ok(vec![TestStruct {
            a: test1,
            b: test2,
            c: test3,
        }])
    }

    #[derive(PushToSquirrelVm, GetFromSquirrelVm, GetFromSQObject, SQVMName)]
    #[repr(i32)]
    enum TestEnum {
        Wow,
        Owo,
    }

    #[derive(PushToSquirrelVm, GetFromSquirrelVm, SQVMName, GetFromSQObject)]
    struct TestStruct {
        a: String,
        b: i32,
        c: TestEnum,
    }

    #[test]
    fn test_test_sqfunction() {
        let sqfuncdef = SQFuncInfo {
            cpp_func_name: stringify!(test_sqfunction),
            sq_func_name: "test",
            types: "string test1, int test2, int test3, void functionref(string) test4".into(),
            return_type: <Vec<TestStruct> as SQVMName>::get_sqvm_name(),
            vm: SQFunctionContext::SERVER,
            function: Some(sq_func_test_sqfunction),
        };
        assert_eq!(test_sqfunction(), sqfuncdef);
    }
}
