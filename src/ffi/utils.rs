use super::error::PluginError;

pub(crate) fn use_get_int_value_func(
    func: unsafe extern "C" fn(*mut i32, i32) -> i32,
    gamestate_type: i32,
) -> Result<Option<i32>, PluginError> {
    let mut int = Box::new(0);
    let ptr = int.as_mut();

    unsafe {
        let result = func(ptr, gamestate_type);

        if result != 0 {
            return Err(PluginError::Failed(result));
        }
    }

    Ok(Some(*ptr))
}

pub(crate) fn use_get_bool_value_func(
    func: unsafe extern "C" fn(*mut bool, i32) -> i32,
    gamestate_type: i32,
) -> Result<Option<bool>, PluginError> {
    let mut boolean = Box::new(false);
    let ptr = boolean.as_mut();

    unsafe {
        let result = func(ptr, gamestate_type);

        if result != 0 {
            return Err(PluginError::Failed(result));
        }
    }

    Ok(Some(*ptr))
}

pub(crate) fn use_get_char_value_func(
    func: unsafe extern "C" fn(*mut i8, usize, i32) -> i32,
    gamestate_type: i32,
) -> Result<Option<Vec<i8>>, PluginError> {
    let mut charvec: Box<Vec<i8>> = Box::new(Vec::with_capacity(128));
    unsafe {
        // let mut slice = charvec.into_boxed_slice();
        let len = charvec.len();
        let capacity = charvec.capacity();
        let ptr = charvec.as_mut_ptr();

        std::mem::forget(charvec);

        let result = func(ptr, capacity, gamestate_type);

        if result != 0 {
            return Err(PluginError::Failed(result));
        }

        Ok(Some(Vec::from_raw_parts(ptr, len, capacity)))
    }
}
