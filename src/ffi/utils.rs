use super::error::PluginError;

pub(crate) fn use_get_int_value_func(
    func: unsafe extern "C" fn(*mut i32, i32) -> i32,
    gamestate_type: i32,
) -> Result<Option<i32>, PluginError> {
    let mut int = Box::new(i32::default());
    let ptr = int.as_mut() as *mut i32;

    unsafe {
        let result = func(ptr, gamestate_type);

        if result != 0 {
            return Err(PluginError::Failed(result));
        }

        Ok(match ptr.as_ref() {
            Some(int) => Some(*int),
            None => None,
        })
    }
}

pub(crate) fn use_get_bool_value_func(
    func: unsafe extern "C" fn(*mut bool, i32) -> i32,
    gamestate_type: i32,
) -> Result<Option<bool>, PluginError> {
    let mut boolean = Box::new(bool::default());
    let ptr = boolean.as_mut() as *mut bool;

    unsafe {
        let result = func(ptr, gamestate_type);

        if result != 0 {
            return Err(PluginError::Failed(result));
        }

        // tbh cloning the data here could be safer than dereferecing since I don't really trust c++ and my own ability to manage memory
        Ok(match ptr.as_ref() {
            Some(boolen) => Some(*boolen),
            None => None,
        })
    }
}

// this https://stackoverflow.com/questions/24145823/how-do-i-convert-a-c-string-into-a-rust-string-and-back-via-ffi
// might help
pub(crate) fn use_get_char_value_func(
    func: unsafe extern "C" fn(*mut i8, usize, i32) -> i32,
    gamestate_type: i32,
) -> Result<Option<String>, PluginError> {
    let mut buffer = Box::new(vec![0_i8; 64]);
    unsafe {
        buffer.shrink_to_fit();
        let len = buffer.len();
        let capacity = buffer.capacity();
        let ptr = buffer.as_mut_ptr();

        std::mem::forget(buffer);

        let result = func(ptr, len, gamestate_type);

        if result != 0 {
            return Err(PluginError::Failed(result));
        }

        // Ok(Some(Vec::from_raw_parts(ptr, len, capacity)))

        let string = match String::from_utf8(Vec::from_raw_parts(ptr as *mut u8, len, capacity)) {
            Ok(string) => string,
            Err(err) => return Err(PluginError::StringError(err)),
        };

        // todo optimize this
        let mut new_string = String::new();
        for c in string.chars() {
            if c != '\0' {
                new_string.push(c)
            } else {
                break;
            }
        }

        Ok(Some(new_string))
    }
}
