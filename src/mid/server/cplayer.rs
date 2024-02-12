//! stuff related to `CPlayer` entity

use std::ffi::c_void;

use crate::offset_functions;

offset_functions! {
   CPLAYER_VTABLE + CPlayerVtable for WhichDll::Server => {
       vtable = *const c_void where offset(0x9524F8);
   }
}
