use std::sync::Arc;
use std::cell::RefCell;

use crate::bindings::{
    plugin_abi::SquirrelFunctions,
    squirrelclasstypes::SQFuncRegistration,
    squirreldatatypes::{CSquirrelVM, HSquirrelVM},
};

use super::{
    errors::{PluginCreationError},
    northstar::ScriptVmType,
};

pub type RegisterSquirrelFuncTypeUnwraped = unsafe extern "C" fn(
    sqvm: *mut CSquirrelVM,
    funcReg: *mut SQFuncRegistration,
    unknown: ::std::os::raw::c_char,
) -> i64;

#[derive(Debug)]
pub struct Squirrel {
    sqtype: ScriptVmType,
    sqvm_cs: Arc<RefCell<&'static mut CSquirrelVM>>,
    sqvm_hs: &'static HSquirrelVM,
    sqfunctions: &'static SquirrelFunctions,
    register_squirrel_function: &'static RegisterSquirrelFuncTypeUnwraped,
}

impl Squirrel {
    pub fn new(
        sqtype: ScriptVmType,
        sqvm_cs: Arc<RefCell<&'static mut CSquirrelVM>>,
        sqfunctions: &'static SquirrelFunctions,
        sqvm_hs: &'static HSquirrelVM,
    ) -> Result<Self, PluginCreationError> {
        // let sqvm_hs = match unsafe { sqvm_cs.sqvm.as_ref() } {
        //     Some(sqvm_hs) => sqvm_hs,
        //     None => Err(PluginCreationError::NoneFunction)?,
        // };
        let register_squirrel_function = match sqfunctions.RegisterSquirrelFunc.as_ref() {
            Some(rsf) => rsf,
            None => return Err(PluginCreationError::NoneFunction),
        };

        Ok(Self {
            sqtype,
            sqvm_cs,
            sqvm_hs,
            sqfunctions,
            register_squirrel_function,
        })
    }

    pub(crate) fn from_builder(
        sqtype: ScriptVmType,
        sqvm_cs: Arc<RefCell<&'static mut CSquirrelVM>>,
        sqvm_hs: &'static HSquirrelVM,
        sqfunctions: &'static SquirrelFunctions,
        register_squirrel_function: &'static RegisterSquirrelFuncTypeUnwraped,
    ) -> Self {
        Self {
            sqtype,
            sqvm_cs,
            sqvm_hs,
            sqfunctions,
            register_squirrel_function,
        }
    }

    pub fn get_sqvm_type(&self) -> ScriptVmType {
        self.sqtype
    }

    pub fn get_sqvm_cs(&self) -> &'static mut CSquirrelVM {
        unsafe {*self.sqvm_cs.as_ptr()}
    }

    pub fn get_sqvm_hs(&self) -> &'static HSquirrelVM {
        self.sqvm_hs
    }

    pub fn get_sqfunctions(&self) -> &'static SquirrelFunctions {
        self.sqfunctions
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn register_squirrel_function_internal(
        &mut self,
        sqfunction: &mut SQFuncRegistration,
    ) {
        let sqvm = self.sqvm_cs.as_ptr();
        (self.register_squirrel_function)(
            sqvm as *mut CSquirrelVM,
            sqfunction as *mut SQFuncRegistration,
            0,
        );
    }
}

impl Clone for Squirrel {
    fn clone(&self) -> Self {
        Self {
            sqtype: self.sqtype,
            sqvm_cs: Arc::clone( &self.sqvm_cs ),
            sqvm_hs: self.sqvm_hs,
            sqfunctions: self.sqfunctions,
            register_squirrel_function: self.register_squirrel_function,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct SquirrelBuilder {
    sqtype: Option<ScriptVmType>,
    sqvm_cs: Option<Arc<RefCell<&'static mut CSquirrelVM>>>,
    sqvm_hs: Option<&'static HSquirrelVM>,
    sqfunctions: Option<&'static SquirrelFunctions>,
    register_squirrel_function: Option<&'static RegisterSquirrelFuncTypeUnwraped>,
}

impl SquirrelBuilder {
    pub(crate) fn new() -> Self {
        Self {
            sqtype: None,
            sqvm_cs: None,
            sqvm_hs: None,
            sqfunctions: None,
            register_squirrel_function: None,
        }
    }

    pub(crate) fn set_sqtype(&mut self, sqtype: ScriptVmType) -> &mut Self {
        self.sqtype = Some(sqtype);

        self
    }

    pub(crate) fn set_sqvm_cs(&mut self, sqvm_cs: &'static mut CSquirrelVM) -> &mut Self {
        self.sqvm_hs = Some(match unsafe { sqvm_cs.sqvm.as_ref() } {
            Some(sqvm_hs) => sqvm_hs,
            None => {
                log::error!(
                    "didn't generate sqvm_hs correctly in squrriel::SquirrelBuilder::set_sqvm_cs"
                );
                panic!();
            }
        });
        self.sqvm_cs = Some(Arc::new(RefCell::new(sqvm_cs)));

        self
    }

    #[allow(unused)]
    pub(crate) fn set_sqvm_hs(&mut self, sqvm_hs: &'static HSquirrelVM) -> &mut Self {
        self.sqvm_hs = Some(sqvm_hs);

        self
    }

    pub(crate) fn set_sqvm_sqfunctions(
        &mut self,
        sqfunctions: &'static SquirrelFunctions,
    ) -> &mut Self {
        self.register_squirrel_function = Some(
            match sqfunctions.RegisterSquirrelFunc.as_ref()  {
                Some(register_squirrel_function) => register_squirrel_function,
                None => {
                    log::error!(
                    "didn't generate register_squirrel_function correctly in squrriel::SquirrelBuilder::set_sqvm_sqfunctions"
                );
                    panic!();
                }
            },
        );
        self.sqfunctions = Some(sqfunctions);

        self
    }

    #[allow(unused)]
    pub(crate) fn set_sqvm_register_squirrel_function(
        &mut self,
        register_squirrel_function: &'static RegisterSquirrelFuncTypeUnwraped,
    ) -> &mut Self {
        self.register_squirrel_function = Some(register_squirrel_function);

        self
    }

    /// will **override** any fields present in the calling [`SquirrelBuilder`]
    ///
    /// # Errors
    /// returns [`NoScriptVmType`] if one of the builders is missing [`ScriptVmType`] </br>
    /// returns [`MergeDifError`] if the [`ScriptVmType`] don't match </br>
    pub fn merge(&mut self, sqbuilder: Self) -> Result<(), PluginCreationError> {
        match self.sqtype {
            Some(self_sqtype) => match sqbuilder.sqtype {
                Some(sqtype)
                    if self_sqtype == ScriptVmType::UiClient
                        && sqtype == ScriptVmType::UiClient => {}
                Some(sqtype) if self_sqtype != sqtype => {
                    return Err(PluginCreationError::MergeDifError(self_sqtype, sqtype))
                }
                Some(_) => {}
                None => return Err(PluginCreationError::NoScriptVmType),
            },
            None => return Err(PluginCreationError::NoScriptVmType),
        }

        if let Some(sqvm_cs) = sqbuilder.sqvm_cs {
            self.sqvm_cs = Some(sqvm_cs)
        }

        if let Some(sqvm_hs) = sqbuilder.sqvm_hs {
            self.sqvm_hs = Some(sqvm_hs)
        }

        if let Some(sqfunctions) = sqbuilder.sqfunctions {
            self.sqfunctions = Some(sqfunctions)
        }

        if let Some(register_squirrel_function) = sqbuilder.register_squirrel_function {
            self.register_squirrel_function = Some(register_squirrel_function)
        }

        Ok(())
    }

    pub fn build(&self) -> Result<Squirrel, PluginCreationError> {
        Ok(Squirrel::from_builder(
            self.sqtype.ok_or(PluginCreationError::SquirrelMissingData(
                "ScriptVmType".into(),
            ))?,
            Arc::clone(
                self
                    .sqvm_cs
                    .as_ref()
                    .ok_or(PluginCreationError::SquirrelMissingData(
                        "CSquirrelVM".into(),
                    ))?,
            ),
            self.sqvm_hs
                .ok_or(PluginCreationError::SquirrelMissingData(
                    "HSquirrelVM".into(),
                ))?,
            self.sqfunctions
                .ok_or(PluginCreationError::SquirrelMissingData(
                    "SquirrelFunctions".into(),
                ))?,
            self.register_squirrel_function
                .ok_or(PluginCreationError::SquirrelMissingData(
                    "RegisterSquirrelFuncTypeUnwraped".into(),
                ))?,
        ))
    }
}
