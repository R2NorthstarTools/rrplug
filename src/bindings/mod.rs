pub mod command;
pub mod convar;
pub mod cvar;
pub mod entity;
pub mod plugin_abi;
pub mod squirrelclasstypes;
pub mod squirreldatatypes;
pub mod unwraped;

#[repr(C)]
pub struct OffsetSructField<T, const U: usize> {
    _pad: [::std::os::raw::c_char; U],
    value: T,
}

impl<T, const U: usize> std::ops::Deref for OffsetSructField<T, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, const U: usize> std::ops::DerefMut for OffsetSructField<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
