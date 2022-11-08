use super::error::PluginError;

pub(crate) fn use_get_char_value_func(
    func: &unsafe extern "C" fn(*mut i8, usize, i32) -> i32,
    gamestate_type: i32,
) -> Result<Option<Box<[i8]>>, PluginError> {
    let mut charvec: Box<Vec<i8>> = Box::new(Vec::new());
    unsafe {
        charvec.set_len(128_usize);
        let mut slice = charvec.into_boxed_slice();
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        let result = func(ptr, len, gamestate_type);

        if result != 0 {
            return Err(PluginError::Failed(result));
        }

        Ok(Some(slice))
    }
}
