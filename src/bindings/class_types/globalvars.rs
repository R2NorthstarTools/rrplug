// TODO: split this into it's base class
#[allow(non_snake_case)]
#[repr(C)]
pub struct CGlobalVars {
    pub m_nUnkTime: f32,
    pub realTime: f32,
    pub frameCount: i32,
    pub absoluteFrameTime: f32,
    pub curTime: f32,
    pub m_flCurTimeUnknown0: f32,
    pub m_flCurTimeUnknown1: f32,
    pub m_flCurTimeUnknown2: f32,
    pub lastFrameTimeSincePause: f32,
    pub m_flCurTimeUnknown3: f32,
    pub exactCurTime: f32,
    pub m_flUnknown4: f32,
    pub frameTime: f32,
    pub maxPlayers: i32,
    pub maxClients: i32,
    pub gameMode: i32,
    pub tickCount: u32,
    pub tickInterval: f32,
    pub m_nUnk1: i32,
    pub m_bClient: bool,
    pub m_nTimestampNetworkingBase: i32,
    pub m_nTimestampRandomizeWindow: i32,
}
