use crate::bindings::{
    plugin_abi::SquirrelFunctions,
    squirrelclasstypes::SQFuncRegistration,
    squirreldatatypes::{CSquirrelVM, HSquirrelVM},
};

use super::{errors::PluginCreationError, northstar::ScriptVmType};

pub type RegisterSquirrelFuncTypeUnwraped = unsafe extern "C" fn(
    sqvm: *mut CSquirrelVM,
    funcReg: *mut SQFuncRegistration,
    unknown: ::std::os::raw::c_char,
) -> i64;

#[derive(Debug, Clone, Copy)]
pub struct Squirrel {
    sqtype: ScriptVmType,
    sqvm_cs: CSquirrelVM,
    sqvm_hs: HSquirrelVM,
    sqfunctions: SquirrelFunctions,
    register_squirrel_function: RegisterSquirrelFuncTypeUnwraped,
}

impl Squirrel {
    pub fn new(
        sqtype: ScriptVmType,
        sqvm_cs: CSquirrelVM,
        sqfunctions: SquirrelFunctions,
    ) -> Result<Self, PluginCreationError> {
        let sqvm_hs = unsafe { *sqvm_cs.sqvm };
        let register_squirrel_function = match sqfunctions.RegisterSquirrelFunc {
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
        sqvm_cs: CSquirrelVM,
        sqvm_hs: HSquirrelVM,
        sqfunctions: SquirrelFunctions,
        register_squirrel_function: RegisterSquirrelFuncTypeUnwraped,
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

    pub fn get_sqvm_cs(&self) -> CSquirrelVM {
        self.sqvm_cs
    }

    pub fn get_sqvm_hs(&self) -> HSquirrelVM {
        self.sqvm_hs
    }

    pub fn get_sqfunctions(&self) -> SquirrelFunctions {
        self.sqfunctions
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn register_squirrel_function_internal(
        &mut self,
        sqfunction: &mut SQFuncRegistration,
    ) {
        (self.register_squirrel_function)(
            &mut self.sqvm_cs as *mut CSquirrelVM,
            sqfunction as *mut SQFuncRegistration,
            0,
        );
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct SquirrelBuilder {
    sqtype: Option<ScriptVmType>,
    sqvm_cs: Option<CSquirrelVM>,
    sqvm_hs: Option<HSquirrelVM>,
    sqfunctions: Option<SquirrelFunctions>,
    register_squirrel_function: Option<RegisterSquirrelFuncTypeUnwraped>,
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

    pub(crate) fn set_sqvm_cs(&mut self, sqvm_cs: CSquirrelVM) -> &mut Self {
        self.sqvm_hs = Some(unsafe { *sqvm_cs.sqvm });
        self.sqvm_cs = Some(sqvm_cs);

        self
    }

    #[allow(unused)]
    pub(crate) fn set_sqvm_hs(&mut self, sqvm_hs: HSquirrelVM) -> &mut Self {
        self.sqvm_hs = Some(sqvm_hs);

        self
    }

    pub(crate) fn set_sqvm_sqfunctions(&mut self, sqfunctions: SquirrelFunctions) -> &mut Self {
        self.register_squirrel_function = sqfunctions.RegisterSquirrelFunc;
        self.sqfunctions = Some(sqfunctions);

        self
    }

    #[allow(unused)]
    pub(crate) fn set_sqvm_register_squirrel_function(
        &mut self,
        register_squirrel_function: RegisterSquirrelFuncTypeUnwraped,
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
            self.sqvm_cs
                .ok_or(PluginCreationError::SquirrelMissingData(
                    "CSquirrelVM".into(),
                ))?,
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
