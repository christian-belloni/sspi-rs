use std::slice::from_raw_parts;

use libc::c_char;

pub fn into_raw_ptr<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}

/// # Safety
///
/// *Note*: the resulting [String] will contain a null-terminator char at the end.
/// Behavior is undefined is any of the following conditions are violated:
///
/// * `s` must be a [valid] C string.
pub unsafe fn c_w_str_to_string(s: *const u16) -> String {
    let mut len = 0;

    // SAFETY: The user must provide guarantees that `s` is a valid C string.
    while unsafe { *(s.add(len)) } != 0 {
        len += 1;
    }

    // SAFETY: The user must provide guarantees that `s` is a valid C string.
    String::from_utf16_lossy(unsafe { from_raw_parts(s, len) })
}

/// # Safety
///
/// The returned length includes the null terminator char.
/// Behavior is undefined is any of the following conditions are violated:
///
/// * `s` must be a [valid] C string.
pub unsafe fn w_str_len(s: *const u16) -> usize {
    let mut len = 0;

    // SAFETY: The user must provide guarantees that `s` is a valid C string.
    while unsafe { *(s.add(len)) } != 0 {
        len += 1;
    }

    len
}

/// Converts raw credentials string into [Vec] of bytes.
///
/// Credentials are often represented as strings. For example, username, domain, password.
/// It is OK for Windows SSPI to accept `null` or empty credential strings. The `AcquireCredentialsHandle`
/// function will return successful status code is we pass the `null` username value. Thus, this function
/// will return an empty [Vec] in such a case. It is done on purpose to follow the Windows SSPI behaviour.
///
/// # Safety
///
/// * the `raw_buffer` pointer can be null.
/// * if `raw_buffer` is not null, then it must be valid for reads for `len` many bytes, and it must be properly aligned.
/// * The total size `len` of the slice must be no larger than `isize::MAX`, and adding that size to `data`
///   must not "wrap around" the address space.
pub unsafe fn credentials_str_into_bytes(raw_buffer: *const c_char, len: usize) -> Vec<u8> {
    if !raw_buffer.is_null() {
        // SAFETY:
        // `raw_buffer` is not null: checked above. All other guarantees should be upheld by the caller.
        unsafe { from_raw_parts(raw_buffer as *const u8, len) }.to_vec()
    } else {
        Vec::new()
    }
}

pub fn str_to_w_buff(data: &str) -> Vec<u16> {
    data.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(any(feature = "scard", feature = "tsssp"))]
pub fn str_encode_utf16(data: &str) -> Vec<u8> {
    data.encode_utf16().flat_map(|c| c.to_le_bytes()).collect()
}
