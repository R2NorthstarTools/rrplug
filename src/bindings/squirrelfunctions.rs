use super::squirrelclasstypes::*;
use crate::offset_functions;

offset_functions! {
    SQUIRREL_SERVER_FUNCS + ServerSQFunctions for WhichDll::Server => {
        register_squirrel_func = RegisterSquirrelFuncType where offset(0x1DD10);

        sq_defconst = sq_defconstType where offset(0x1F550);

        sq_compilebuffer = sq_compilebufferType where offset(0x3110);
        sq_pushroottable = sq_pushroottableType where offset(0x5840);
        sq_call = sq_callType where offset(0x8620);
        sq_compilefile = sq_compilefileType where offset(0x1CD80);

        sq_newarray = sq_newarrayType where offset(0x39F0);
        sq_arrayappend = sq_arrayappendType where offset(0x3C70);

        sq_newtable = sq_newtableType where offset(0x3960);
        sq_newslot = sq_newslotType where offset(0x7080);

        sq_pushstring = sq_pushstringType where offset(0x3440);
        sq_pushinteger = sq_pushintegerType where offset(0x36A0);
        sq_pushfloat = sq_pushfloatType where offset(0x3800);
        sq_pushbool = sq_pushboolType where offset(0x3710);
        sq_pushasset = sq_pushassetType where offset(0x3560);
        sq_pushvector = sq_pushvectorType where offset(0x3780);
        sq_pushobject = sq_pushobjectType where offset(0x83A0);

        sq_raiseerror = sq_raiseerrorType where offset(0x8440);

        sq_getstring = sq_getstringType where offset(0x5840);
        sq_getinteger = sq_getintegerType where offset(0x60C0);
        sq_getfloat = sq_getfloatType where offset(0x60E0);
        sq_getbool = sq_getboolType where offset(0x6110);
        sq_getasset = sq_getassetType where offset(0x5FF0);
        sq_getuserdata = sq_getuserdataType where offset(0x63B0);
        sq_getvector = sq_getvectorType where offset(0x6120);
        sq_get = sq_getType where offset(0x7C00);

        sq_getthisentity = sq_getthisentityType where offset(0x203B0);
        sq_getobject = sq_getobjectType where offset(0x6140);

        sq_createuserdata = sq_createuserdataType where offset(0x38D0);
        sq_setuserdatatypeid = sq_setuserdatatypeidType where offset(0x6470);

        sq_get_entity_constant_cbase_entity = sq_GetEntityConstantType where offset(0x418AF0);
        sq_getentityfrominstance = sq_getentityfrominstanceType where offset(0x1E920);

        sq_getfunction = sq_getfunctionType where offset(0x6C85);
        sq_stackinfos = sq_stackinfosType where offset(0x35920);

        sq_pushnewstructinstance = sq_pushnewstructinstanceType where offset(0x53e0);
        sq_sealstructslot = sq_sealstructslotType where offset(0x5510);
    }
}

offset_functions! {
    SQUIRREL_CLIENT_FUNCS + ClientSQFunctions for WhichDll::Client => {
        register_squirrel_func = RegisterSquirrelFuncType where offset(0x108E0);

        sq_defconst = sq_defconstType where offset(0x12120);

        sq_compilebuffer = sq_compilebufferType where offset(0x3110);
        sq_pushroottable = sq_pushroottableType where offset(0x5840);
        sq_call = sq_callType where offset(0x8650);
        sq_compilefile = sq_compilefileType where offset(0xF950);

        sq_newarray = sq_newarrayType where offset(0x39F0);
        sq_arrayappend = sq_arrayappendType where offset(0x3C70);

        sq_newtable = sq_newtableType where offset(0x3960);
        sq_newslot = sq_newslotType where offset(0x70B0);

        sq_pushstring = sq_pushstringType where offset(0x3440);
        sq_pushinteger = sq_pushintegerType where offset(0x36A0);
        sq_pushfloat = sq_pushfloatType where offset(0x3800);
        sq_pushbool = sq_pushboolType where offset(0x3710);
        sq_pushasset = sq_pushassetType where offset(0x3560);
        sq_pushvector = sq_pushvectorType where offset(0x3780);
        sq_pushobject = sq_pushobjectType where offset(0x83D0);

        sq_raiseerror = sq_raiseerrorType where offset(0x8470);

        sq_getstring = sq_getstringType where offset(0x60C0);
        sq_getinteger = sq_getintegerType where offset(0x60E0);
        sq_getfloat = sq_getfloatType where offset(0x6100);
        sq_getbool = sq_getboolType where offset(0x6130);
        sq_getasset = sq_getassetType where offset(0x6010);
        sq_getuserdata = sq_getuserdataType where offset(0x6110);
        sq_getvector = sq_getvectorType where offset(0x6140);
        sq_get = sq_getType where offset(0x7C30);

        sq_getthisentity = sq_getthisentityType where offset(0x12F80);
        sq_getobject = sq_getobjectType where offset(0x6160);

        sq_createuserdata = sq_createuserdataType where offset(0x38D0);
        sq_setuserdatatypeid = sq_setuserdatatypeidType where offset(0x6490);

        sq_get_entity_constant_cbase_entity = sq_GetEntityConstantType where offset(0x3E49B0);
        sq_getentityfrominstance = sq_getentityfrominstanceType where offset(0x114F0);

        sq_getfunction = sq_getfunctionType where offset(0x6CB0);
        sq_stackinfos = sq_stackinfosType where offset(0x35970);

        sq_pushnewstructinstance = sq_pushnewstructinstanceType where offset(0x5400);
        sq_sealstructslot = sq_sealstructslotType where offset(0x5530);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SquirrelFunctions {
    pub register_squirrel_func: RegisterSquirrelFuncType,
    pub sq_defconst: sq_defconstType,
    pub sq_compilebuffer: sq_compilebufferType,
    pub sq_call: sq_callType,
    pub sq_raiseerror: sq_raiseerrorType,
    pub sq_compilefile: sq_compilefileType,
    pub sq_newarray: sq_newarrayType,
    pub sq_arrayappend: sq_arrayappendType,
    pub sq_newtable: sq_newtableType,
    pub sq_newslot: sq_newslotType,
    pub sq_pushroottable: sq_pushroottableType,
    pub sq_pushstring: sq_pushstringType,
    pub sq_pushinteger: sq_pushintegerType,
    pub sq_pushfloat: sq_pushfloatType,
    pub sq_pushbool: sq_pushboolType,
    pub sq_pushasset: sq_pushassetType,
    pub sq_pushvector: sq_pushvectorType,
    pub sq_pushobject: sq_pushobjectType,
    pub sq_getstring: sq_getstringType,
    pub sq_getinteger: sq_getintegerType,
    pub sq_getfloat: sq_getfloatType,
    pub sq_getbool: sq_getboolType,
    pub sq_get: sq_getType,
    pub sq_getasset: sq_getassetType,
    pub sq_getuserdata: sq_getuserdataType,
    pub sq_getvector: sq_getvectorType,
    pub sq_getthisentity: sq_getthisentityType,
    pub sq_getobject: sq_getobjectType,
    pub sq_stackinfos: sq_stackinfosType,
    pub sq_createuserdata: sq_createuserdataType,
    pub sq_setuserdatatypeid: sq_setuserdatatypeidType,
    pub sq_getfunction: sq_getfunctionType,
    pub sq_getentityfrominstance: sq_getentityfrominstanceType,
    pub sq_get_entity_constant_cbase_entity: sq_GetEntityConstantType,
    pub sq_pushnewstructinstance: sq_pushnewstructinstanceType,
    pub sq_sealstructslot: sq_sealstructslotType,
}

impl From<&ClientSQFunctions> for SquirrelFunctions {
    fn from(val: &ClientSQFunctions) -> Self {
        SquirrelFunctions {
            register_squirrel_func: val.register_squirrel_func,
            sq_defconst: val.sq_defconst,
            sq_compilebuffer: val.sq_compilebuffer,
            sq_call: val.sq_call,
            sq_raiseerror: val.sq_raiseerror,
            sq_compilefile: val.sq_compilefile,
            sq_newarray: val.sq_newarray,
            sq_arrayappend: val.sq_arrayappend,
            sq_newtable: val.sq_newtable,
            sq_newslot: val.sq_newslot,
            sq_pushroottable: val.sq_pushroottable,
            sq_pushstring: val.sq_pushstring,
            sq_pushinteger: val.sq_pushinteger,
            sq_pushfloat: val.sq_pushfloat,
            sq_pushbool: val.sq_pushbool,
            sq_pushasset: val.sq_pushasset,
            sq_pushvector: val.sq_pushvector,
            sq_pushobject: val.sq_pushobject,
            sq_getstring: val.sq_getstring,
            sq_getinteger: val.sq_getinteger,
            sq_getfloat: val.sq_getfloat,
            sq_getbool: val.sq_getbool,
            sq_get: val.sq_get,
            sq_getasset: val.sq_getasset,
            sq_getuserdata: val.sq_getuserdata,
            sq_getvector: val.sq_getvector,
            sq_getthisentity: val.sq_getthisentity,
            sq_getobject: val.sq_getobject,
            sq_stackinfos: val.sq_stackinfos,
            sq_createuserdata: val.sq_createuserdata,
            sq_setuserdatatypeid: val.sq_setuserdatatypeid,
            sq_getfunction: val.sq_getfunction,
            sq_getentityfrominstance: val.sq_getentityfrominstance,
            sq_get_entity_constant_cbase_entity: val.sq_get_entity_constant_cbase_entity,
            sq_pushnewstructinstance: val.sq_pushnewstructinstance,
            sq_sealstructslot: val.sq_sealstructslot,
        }
    }
}

impl From<&ServerSQFunctions> for SquirrelFunctions {
    fn from(val: &ServerSQFunctions) -> Self {
        SquirrelFunctions {
            register_squirrel_func: val.register_squirrel_func,
            sq_defconst: val.sq_defconst,
            sq_compilebuffer: val.sq_compilebuffer,
            sq_call: val.sq_call,
            sq_raiseerror: val.sq_raiseerror,
            sq_compilefile: val.sq_compilefile,
            sq_newarray: val.sq_newarray,
            sq_arrayappend: val.sq_arrayappend,
            sq_newtable: val.sq_newtable,
            sq_newslot: val.sq_newslot,
            sq_pushroottable: val.sq_pushroottable,
            sq_pushstring: val.sq_pushstring,
            sq_pushinteger: val.sq_pushinteger,
            sq_pushfloat: val.sq_pushfloat,
            sq_pushbool: val.sq_pushbool,
            sq_pushasset: val.sq_pushasset,
            sq_pushvector: val.sq_pushvector,
            sq_pushobject: val.sq_pushobject,
            sq_getstring: val.sq_getstring,
            sq_getinteger: val.sq_getinteger,
            sq_getfloat: val.sq_getfloat,
            sq_getbool: val.sq_getbool,
            sq_get: val.sq_get,
            sq_getasset: val.sq_getasset,
            sq_getuserdata: val.sq_getuserdata,
            sq_getvector: val.sq_getvector,
            sq_getthisentity: val.sq_getthisentity,
            sq_getobject: val.sq_getobject,
            sq_stackinfos: val.sq_stackinfos,
            sq_createuserdata: val.sq_createuserdata,
            sq_setuserdatatypeid: val.sq_setuserdatatypeid,
            sq_getfunction: val.sq_getfunction,
            sq_getentityfrominstance: val.sq_getentityfrominstance,
            sq_get_entity_constant_cbase_entity: val.sq_get_entity_constant_cbase_entity,
            sq_pushnewstructinstance: val.sq_pushnewstructinstance,
            sq_sealstructslot: val.sq_sealstructslot,
        }
    }
}
