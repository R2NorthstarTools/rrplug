use crate::bindings::plugin_abi::CreateObjectFunc;
use once_cell::sync::OnceCell;

pub static CREATE_OBJECT_FUNC: OnceCell<CreateObjectFunc> = OnceCell::new();