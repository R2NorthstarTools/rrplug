#![allow(clippy::not_unsafe_ptr_arg_deref)] // maybe remove this later

use std::mem::MaybeUninit;

use super::vector::Vector3;
use crate::{
    bindings::{
        squirreldatatypes::{HSquirrelVM, SQObject},
        unwraped::SquirrelFunctionsUnwraped,
    },
    mid::squirrel::{
        push_sq_bool, push_sq_float, push_sq_int, push_sq_object, push_sq_string, push_sq_vector,
    },
};

// Push Traits

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

// Get Traits
