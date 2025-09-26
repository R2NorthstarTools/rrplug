//! titanfall 2's server related stuff

use std::ffi::c_void;

use crate::offset_functions;

offset_functions! {
   ENTITY_CLASS_VTABLE + EntityClassVtable for WhichDll::Server => {
       cplayer = *const c_void where offset(0x9524F8);
       weaponx = *const c_void where offset(0x98e2b8);
       cplayerdecoy = *const c_void where offset(0x8b4f58);
   }
}
